#[macro_use]
extern crate nom;

use nom::{HexDisplay,Needed,IResult,FlatMapOpt,Functor,FileProducer,be_u16,be_u32,be_u64,be_f32};
use nom::{Consumer,ConsumerState};
use nom::IResult::*;

pub struct Gif;

pub fn header(input:&[u8]) -> IResult<&[u8], Gif> {

  chain!(input,
    tag!("GIF")     ~
    alt!(
      tag!("87a") |
      tag!("89a")
   )                ,
   || { Gif }
  )
  //Done(input, Gif )
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::{HexDisplay,IResult};

  #[test]
  fn test() {
    let data = include_bytes!("../axolotl-piano.gif");
    println!("bytes:\n {}", &data[0..100].to_hex(8));
    let res = header(data);


    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n {}", &i[0..100].to_hex(8));
      },
      _  => { println!("error or incomplete") }
    }
    panic!("abc");
  }

}
