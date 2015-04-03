use nom::util::*;
use std::slice::bytes::copy_memory;

pub fn decode_lzw(colors: Vec< Vec<u8> >, code_size: u8, blocks: Vec<&[u8]>, buffer: &mut [u8]) -> Option<usize> {

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

pub fn decode_block(mut dictionary: Vec< Vec<u8> >, code_size: u8, block: &[u8], buffer: &mut [u8]) -> Option<usize> {
    println!("will decode block:\n{}", block.to_hex(8));
    let mut count = 0;
    for idx in 0..block.len() {
      //println!("c: {}", block[idx]);
      if idx == 0 {
        let code = block[idx] as usize;
        println!("will copy {:?} in {} bytes", &dictionary[code], (&buffer[count..]).len());
        //FIXME: https://github.com/rust-lang/rust/issues/22890 reordered the arguments
        copy_memory(&dictionary[code][..], buffer);
        count = count + dictionary[code].len();
      } else {
        let code = block[idx] as usize;
        if code <= dictionary.len() {
          copy_memory(&dictionary[code][..], &mut buffer[count..]);
          let k = dictionary[code][0];
          let mut v: Vec<u8> = Vec::new();
          v.push_all(&dictionary[block[idx - 1] as usize]);
          v.push(k);
          dictionary.push(v);

          println!("c: {} => {:?}", block[idx], dictionary[code as usize]);
          count = count + dictionary[code].len();
        } else {
          let k = dictionary[block[idx - 1] as usize][0];
          let mut v: Vec<u8> = Vec::new();
          v.push_all(&dictionary[block[idx - 1] as usize]);
          v.push(k);
          copy_memory(&v, &mut buffer[count..]);
          dictionary.push(v.clone());

          println!("c: {} => {:?}", block[idx], v);
          count = count + v.len();
        }
      }
    }

  Some(count)
}
