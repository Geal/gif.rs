#![feature(core,trace_macros)]

#[macro_use]
extern crate nom;

use nom::{HexDisplay,Needed,IResult,FlatMapOpt,Functor,FileProducer,be_u8,le_u8,le_u16,length_value};
use nom::{Consumer,ConsumerState};
use nom::IResult::*;
use std::num::Int;
use std::str::from_utf8;

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

#[derive(Debug,PartialEq,Eq)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8
}

pub type GlobalColorTable = Vec<Color>;

pub fn global_color_table(input:&[u8], count: u16) -> IResult<&[u8], GlobalColorTable> {

  count!(input,
    chain!(
      r: be_u8 ~
      g: be_u8 ~
      b: be_u8 ,
      || {
        Color { r: r, g: g, b: b }
      }
    ),
    count
  )
}

#[derive(Debug,PartialEq,Eq)]
pub struct Application<'a> {
  identifier:          &'a str,
  authentication_code: &'a [u8],
  data:                &'a [u8]
}

#[derive(Debug,PartialEq,Eq)]
pub enum Block<'a> {
  GraphicBlock,
  ApplicationExtension(Application<'a>),
  CommentExtension
}


pub fn application_extension<'a>(input: &'a[u8]) -> IResult<&'a[u8], Block<'a>> {
  chain!(input,
    tag!( &[0xff, 11][..] )                   ~
    identifier: map_res!(take!(8), from_utf8) ~
    code: take!(3)                            ~
    data: length_value                        ~
    tag!( &[0][..] )                          ,
    || { Block::ApplicationExtension(Application{
      identifier:          identifier,
      authentication_code: code,
      data:                data
    }) }
  )
}

pub fn block(input:&[u8]) -> IResult<&[u8], Block> {
  chain!(input,
    tag!( "!" ) ~
  blk: alt!(
    application_extension
  ),
  || {
    blk
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
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = logical_screen_descriptor(&data);


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
        panic!("cannot parse logical screen descriptor");
      }
    }
  }

  #[test]
  fn global_color_table_test() {
    let d = include_bytes!("../axolotl-piano.gif");
    let data = &d[13..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    // we know the color table size
    let res = global_color_table(data, 256);
    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn block_test() {
    let d = include_bytes!("../axolotl-piano.gif");
    let data = &d[781..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = block(data);
    match res {
      IResult::Done(i, o) => {
        println!("offset: {:?}", d.offset(i));
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
        panic!("hello");
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }
}
