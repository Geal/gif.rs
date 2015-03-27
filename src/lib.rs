#![feature(core)]

#[macro_use]
extern crate nom;

use nom::{HexDisplay,Needed,IResult,FlatMapOpt,Functor,FileProducer,be_u8,le_u8,le_u16};
use nom::{Consumer,ConsumerState};
use nom::IResult::*;
use std::num::Int;

#[derive(Debug,PartialEq,Eq)]
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
}

#[derive(Debug,PartialEq,Eq)]
pub struct LogicalScreenDescriptor {
  width:                  u16,
  height:                 u16,
  gct_flag:               bool,
  color_resolution:       u8,
  gct_sorted:             bool,
  gct_size:               u16,
  background_color_index: u8,
  pixel_aspect_ratio:     u8
}

pub fn logical_screen_descriptor(input:&[u8]) -> IResult<&[u8], LogicalScreenDescriptor> {
  chain!(input,
    width:  le_u16 ~
    height: le_u16 ~
    fields: be_u8  ~
    index:  be_u8  ~
    ratio:  be_u8  ,
   || {
     LogicalScreenDescriptor {
       width:                  width,
       height:                 height,
       gct_flag:               fields & 0b10000000 == 0b10000000,
       color_resolution:       (fields & 0b01110000) >> 4,
       gct_sorted:             fields & 0b00001000 == 0b00001000,
       gct_size:               2.pow((1 + (fields & 0b00000111)) as u32),
       background_color_index: index,
       pixel_aspect_ratio:     ratio
     }
   }
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::{HexDisplay,IResult};

  #[test]
  fn header_test() {
    let data = include_bytes!("../axolotl-piano.gif");
    println!("bytes:\n{}", &data[0..100].to_hex(8));
    let res = header(data);


    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n{}", &i[0..100].to_hex(8));
      },
      _  => {
        println!("error or incomplete");
        panic!("cannot parse header");
      }
    }
  }

  #[test]
  fn logical_screen_descriptor_test() {
    let d = include_bytes!("../axolotl-piano.gif");
    let data = &d[6..];
    println!("bytes:\n{}", &data[0..94].to_hex_from(8, d.offset(data)));

    let res = logical_screen_descriptor(&d[6..]);


    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
        assert_eq!(o, LogicalScreenDescriptor {
          width:                  400,
          height:                 300,
          gct_flag:               true,
          color_resolution:       7,
          gct_sorted:             false,
          gct_size:               256,
          background_color_index: 255,
           pixel_aspect_ratio:    0
        });
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse header");
      }
    }
  }

}
