use nom::util::*;
use std::io;
use std::io::Read;
use std::io::ErrorKind::InvalidInput;
use std::slice::bytes::copy_memory;
use byteorder::ReadBytesExt;

fn subblocks_to_buffer(blocks: Vec<&[u8]>) -> Vec<u8> {
  let mut data: Vec<u8> = Vec::new();
  for b in blocks.iter() {
    data.push_all(b);
  }
  data
}

pub trait BitReader: Read {
  /// Returns the next `n` bits.
  fn read_bits(&mut self, n: u8) -> io::Result<u16>;
}

pub struct LsbReader<R> where R: Read {
  r: R,
  bits: u8,
  acc: u32,
}

impl<R: Read> LsbReader<R> {

  /// Creates a new bit reader
  pub fn new(reader: R) -> LsbReader<R> {
    LsbReader {
      r: reader,
      bits: 0,
      acc: 0,
    }
  }

  /// Returns true if the reader is aligned to a byte of the underlying byte stream.
  #[inline(always)]
  fn is_aligned(&self) -> bool {
    self.bits == 0
  }
}

impl<R: Read> Read for LsbReader<R> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    if self.is_aligned() {
      self.r.read(buf)
    } else {
      let mut i = 0;
      for (j, byte) in buf.iter_mut().enumerate() {
        *byte = try!(self.read_bits(8)) as u8;
        i = j;
      }
      Ok(i)
    }
  }
}

impl<R> BitReader for LsbReader<R> where R: Read {

  fn read_bits(&mut self, n: u8) -> io::Result<u16> {
    if n > 16 {
      return Err(io::Error::new(InvalidInput, "Cannot read more than 16 bits"))
    }
    while self.bits < n {
      self.acc |= (try!(self.r.read_u8()) as u32) << self.bits;
      self.bits += 8;
    }
    let res = self.acc & ((1 << n) - 1);
    self.acc >>= n;
    self.bits -= n;
    Ok(res as u16)
  }

}

const MAX_CODESIZE: u8 = 12;
type Code = u16;

struct DecodingDict {
  min_size: u8,
  table: Vec<(Option<Code>, u8)>,
  buffer: Vec<u8>,
}

impl DecodingDict {
  /// Creates a new dict
  fn new(min_size: u8) -> DecodingDict {
    DecodingDict {
      min_size: min_size,
      table: Vec::with_capacity(512),
      buffer: Vec::with_capacity((1 << MAX_CODESIZE as usize) - 1)
    }
  }

    /// Resets the dictionary
    fn reset(&mut self) {
        self.table.clear();
        for i in (0..(1u16 << self.min_size as usize)) {
            self.table.push((None, i as u8));
        }
    }

    /// Inserts a value into the dict
    #[inline(always)]
    fn push(&mut self, key: Option<Code>, value: u8) {
        self.table.push((key, value))
    }

    /// Reconstructs the data for the corresponding code
    fn reconstruct(&mut self, code: Option<Code>) -> io::Result<&[u8]> {
        self.buffer.clear();
        let mut code = code;
        let mut cha;
        // Check the first access more thoroughly since a bad code
        // could occur if the data is malformed
        if let Some(k) = code {
            match self.table.get(k as usize) {
                Some(&(code_, cha_)) => {
                    code = code_;
                    cha = cha_;
                }
                None => return Err(io::Error::new(InvalidInput, "invalid code occured"))
            }
            self.buffer.push(cha);
        }
        while let Some(k) = code {
            //(code, cha) = self.table[k as usize];
            // Node this could possibly replaced by unsafe access because this
            // struct had been contructed by this algorithm correctly
            let entry = self.table[k as usize]; code = entry.0; cha = entry.1;
            self.buffer.push(cha);
        }
        self.buffer.reverse();
        Ok(&self.buffer[..])
    }

    /// Returns the buffer constructed by the last reconstruction
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        &self.buffer[..]
    }

    /// Number of entries in the dictionary
    #[inline(always)]
    fn next_code(&self) -> u16 {
        self.table.len() as u16
    }
}

pub fn decode_lzw(colors: Vec< Vec<u8> >, min_code_size: usize, blocks: Vec<&[u8]>, buffer: &mut [u8]) -> io::Result<usize> {

  println!("buffer size: {}", buffer.len());
  let mut data = subblocks_to_buffer(blocks);
  let mut r = LsbReader::new(&data[..]);

  let mut prev = None;
  println!("min code size: {}", min_code_size);
  let clear_code: u16 = 1 << min_code_size;
  let end_code = clear_code + 1;
  let mut table = DecodingDict::new(min_code_size as u8);
  let mut code_size:u8 = min_code_size as u8 + 1;
  let mut count:usize = 0;
  println!("start decoding");
  loop {
    let code = try!(r.read_bits(code_size));
    println!("{}| current code: {}", count, code);
    if code as u16 == clear_code {
      table.reset();
      table.push(None, 0); // clear code
      table.push(None, 0); // end code
      code_size = min_code_size as u8 + 1;
      prev = None;
    } else if code as u16 == end_code {
      return Ok(count)
    } else {
      let next_code = table.next_code();
      if prev.is_none() {
        let cols = translate_color(&colors, code);
        copy_memory(&cols[..], &mut buffer[count..]);
        count = count + cols.len();
      } else {
        let data = if (code as u16) == next_code {
          let cha = try!(table.reconstruct(prev))[0];
          table.push(prev, cha);
          try!(table.reconstruct(Some(code as u16)))
        } else if (code as u16) < next_code {
          let cha = try!(table.reconstruct(Some(code as u16)))[0];
          table.push(prev, cha);
          table.buffer()
        } else {
          println!("invalid code, expected {} <= {}", code, next_code);
          return Err(io::Error::new(InvalidInput, "Invalid code"))
        };
        let cols = translate_colors(&colors, data);
        copy_memory(&cols[..], &mut buffer[count..]);
        count = count + cols.len();
      }
      if next_code == (1 << code_size as usize) - 1
        && code_size < MAX_CODESIZE as u8 {
          code_size += 1;
        }
      prev = Some(code as u16);
    }
  }
  Ok(0)
}

pub fn translate_color(colors: &[Vec<u8>], code: u16) -> Vec<u8> {
  let mut res:Vec<u8> = Vec::with_capacity(3);
  res.push_all(&colors[code as usize][..]);
  res
}

pub fn translate_colors(colors: &[Vec<u8>], codes: &[u8]) -> Vec<u8> {
  let mut res:Vec<u8> = Vec::with_capacity(codes.len() * 3);
  for &code in codes.iter() {
    res.push_all(&colors[code as usize][..])
  }
  res
}
