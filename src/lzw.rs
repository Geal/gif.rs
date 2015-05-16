use nom::util::*;
use std::num::Int;
use std::io;
use std::old_io::{Reader,MemReader,IoResult,IoError};
use std::old_io::IoErrorKind::InvalidInput;
use std::slice::bytes::copy_memory;

fn subblocks_to_buffer(blocks: Vec<&[u8]>) -> Vec<u8> {
  let mut data: Vec<u8> = Vec::new();
  for b in blocks.iter() {
    data.push_all(b);
  }
  data
}

pub trait BitReader: Reader {
  /// Returns the next `n` bits.
  fn read_bits(&mut self, n: u8) -> IoResult<u16>;
}

pub struct LsbReader<R> where R: Reader {
  r: R,
  bits: u8,
  acc: u32,
}

impl<R: Reader> LsbReader<R> {

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

impl<R: Reader> Reader for LsbReader<R> {
  fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
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

impl<R> BitReader for LsbReader<R> where R: Reader {

  fn read_bits(&mut self, n: u8) -> IoResult<u16> {
    if n > 16 {
      return Err(IoError {
        kind: InvalidInput,
        desc: "Cannot read more than 16 bits",
        detail: None
      })
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
    fn reconstruct(&mut self, code: Option<Code>) -> IoResult<&[u8]> {
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
                None => return Err(IoError {
                    kind: InvalidInput,
                    desc: "invalid code occured",
                    detail: Some(format!("{} < {} expected", k, self.table.len()))
                })
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

pub fn decode_lzw(colors: Vec< Vec<u8> >, min_code_size: usize, blocks: Vec<&[u8]>, buffer: &mut [u8]) -> IoResult<usize> {

  println!("buffer size: {}", buffer.len());
  let mut data = subblocks_to_buffer(blocks);
  let mut r = LsbReader::new(MemReader::new(data));

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
        //try!(w.write_u8(code as u8));
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
          return Err(IoError {
            kind: InvalidInput,
            desc: "Invalid code",
            detail: Some(format!("expected {} <= {}",
                                 code,
                                 next_code)
                        )
          })
        };
        //try!(w.write(data));
        let cols = translate_colors(&colors, data);
        //println!("will copy {:?} at address {} of buffer of size {}", &cols[..], count, buffer.len());
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
  /*let mut index = 0;
  for b in blocks.iter() {
    let mut dictionary = colors.clone();
    match decode_block(dictionary, code_size, b, &mut buffer[index..]) {
      Some(count) => index = index + count,
      None        => return None
    }
  }
  Some(index)*/
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
/*
pub fn decode_block(mut colors: Vec< Vec<u8> >, code_size: usize, block: &[u8], buffer: &mut [u8]) -> Option<usize> {
    println!("will decode block:\n{}", block.to_hex(8));
    //println!("will decode block");
    let mut dictionary: Vec< Vec<u16> > = Vec::new();
    let clear_code = 2.pow(1 + code_size as u32);
    let stop_code = clear_code + 1;
    println!("clear code '{}' and stop code '{}'", clear_code, stop_code);
    let end = 2.pow(code_size as u32);
    for i in 0..end {
    //for i in 0..100 {
      let mut v: Vec<u16> = Vec::new();
      v.push(i);
      //println!("adding {:?}", v);
      dictionary.push(v);
    }
    let mut count = 0;
    let mut old   = 0;
    for idx in 0..block.len() {
      println!("c: {}", block[idx]);
      let code = block[idx] as usize;
      if code == clear_code {
        println!("found clear code {} at {}", clear_code, count);
      } else if code == stop_code {
        println!("found stop  code {} at {}", stop_code,  count);
      }
      if idx == 0 {
        old = code;
        //println!("will copy {:?} in {} bytes", &dictionary[code], (&buffer[count..]).len());
        //FIXME: https://github.com/rust-lang/rust/issues/22890 reordered the arguments
        let cols = translate_colors(&colors, &dictionary[code][..]);
        println!("will copy {:?} at address {} of buffer of size {}", &cols[..], count, buffer.len());
        copy_memory(&cols[..], buffer);
        //copy_memory(code, buffer);
        //count = count + dictionary[code].len();
        count = count + cols.len();
      } else {
        if code <= dictionary.len() {
          let cols = translate_colors(&colors, &dictionary[code][..]);
          println!("will copy {:?} at address {} of buffer of size {}", &cols[..], count, buffer.len());
          copy_memory(&cols[..], &mut buffer[count..]);
          let k = dictionary[code][0];
          let mut v: Vec<u16> = Vec::new();
          v.push_all(&dictionary[old]);
          v.push(k);
          dictionary.push(v);

          //println!("c: {} => {:?}", block[idx], dictionary[code as usize]);
          //count = count + dictionary[code].len();
          count = count + cols.len();
        } else {
          let k = dictionary[old][0];
          let mut v: Vec<u16> = Vec::new();
          v.push_all(&dictionary[old]);
          v.push(k);
          let cols = translate_colors(&colors, &v);
          println!("will copy {:?} at address {} of buffer of size {}", &cols[..], count, buffer.len());
          copy_memory(&cols[..], &mut buffer[count..]);
          dictionary.push(v.clone());

          //println!("c: {} => {:?}", block[idx], v);
          //count = count + v.len();
          count = count + cols.len();
        }
        let old = code;
      }
    }

  Some(count)
}
*/
