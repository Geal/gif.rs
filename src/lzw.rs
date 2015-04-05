use nom::util::*;
use std::num::Int;
use std::slice::bytes::copy_memory;

pub fn decode_lzw(colors: Vec< Vec<u8> >, code_size: usize, blocks: Vec<&[u8]>, buffer: &mut [u8]) -> Option<usize> {

  println!("buffer size: {}", buffer.len());
  let mut index = 0;
  for b in blocks.iter() {
    let mut dictionary = colors.clone();
    match decode_block(dictionary, code_size, b, &mut buffer[index..]) {
      Some(count) => index = index + count,
      None        => return None
    }
  }
  Some(index)
}

pub fn translate_colors(colors: &[Vec<u8>], codes: &[u16]) -> Vec<u8> {
  let mut res:Vec<u8> = Vec::with_capacity(codes.len() * 3);
  for &code in codes.iter() {
    res.push_all(&colors[code as usize][..])
  }
  res
}

pub fn decode_block(mut colors: Vec< Vec<u8> >, code_size: usize, block: &[u8], buffer: &mut [u8]) -> Option<usize> {
    //println!("will decode block:\n{}", block.to_hex(8));
    //println!("will decode block");
    let mut dictionary: Vec< Vec<u16> > = Vec::new();
    let clear_code = 2.pow(1 + code_size as u32);
    let stop_code = clear_code + 1;
    //println!("clear code '{}' and stop code '{}'", clear_code, stop_code);
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
      //println!("c: {}", block[idx]);
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
        copy_memory(&cols[..], buffer);
        //copy_memory(code, buffer);
        //count = count + dictionary[code].len();
        count = count + cols.len();
      } else {
        if code <= dictionary.len() {
          let cols = translate_colors(&colors, &dictionary[code][..]);
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
