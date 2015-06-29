use nom::{HexDisplay,Needed,IResult,be_u8,le_u8,le_u16,length_value};
use nom::Err;
use nom::IResult::*;
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
  pub  width:                  u16,
  pub  height:                 u16,
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
       gct_size:               2u16.pow((1 + (fields & 0b00000111)) as u32),
       background_color_index: index,
       pixel_aspect_ratio:     ratio
     }
   }
  )
}

pub fn header_and_logical_screen_descriptor(input: &[u8]) -> IResult<&[u8], LogicalScreenDescriptor> {
  chain!(input,
                header                    ~
    descriptor: logical_screen_descriptor ,
                || { descriptor }
  )
}

//#[derive(Debug,PartialEq,Eq)]
pub type Color = Vec<u8>;
/*pub struct Color {
  r: u8,
  g: u8,
  b: u8
}*/

pub type GlobalColorTable = Vec<Color>;

pub fn color_table(input:&[u8], count: u16) -> IResult<&[u8], GlobalColorTable> {
  count!(input,
    chain!(
      r: be_u8 ~
      g: be_u8 ~
      b: be_u8 ,
      || {
        let mut v: Vec<u8> = Vec::new();
        v.push(r);
        v.push(g);
        v.push(b);
        //Color { r: r, g: g, b: b }
        v
      }
    ),
    count as usize
  )
}

#[derive(Debug,PartialEq,Eq)]
pub struct Application<'a> {
  identifier:          &'a str,
  authentication_code: &'a [u8],
  data:                &'a [u8]
}

#[derive(Debug,PartialEq,Eq)]
pub struct GraphicControl {
  disposal_method:    u8,
  user_input:         bool,
  transparency:       bool,
  delay_time:         u16,
  transparency_index: u8
}

pub type LocalColorTable = Vec<Color>;

#[derive(Debug,PartialEq,Eq)]
pub struct ImageDescriptor {
  left_position:          u16,
  top_position:           u16,
  width:                  u16,
  height:                 u16,
  local_color_table_flag: bool,
  interlace:              bool,
  sort:                   bool,
  local_color_table_size: u32,
  local_color_table:      Option<LocalColorTable>
}

#[derive(Debug,PartialEq,Eq)]
pub enum GraphicRenderingBlock<'a> {
  TableBasedImage(ImageDescriptor, u8, Vec<&'a [u8]>),
  PlainTextExtension
}

#[derive(Debug,PartialEq,Eq)]
pub enum Block<'a> {
  GraphicBlock(Option<GraphicControl>, GraphicRenderingBlock<'a>),
  ApplicationExtension(Application<'a>),
  CommentExtension
}


pub fn application_extension<'a>(input: &'a[u8]) -> IResult<&'a[u8], Block<'a>> {
  chain!(input,
                tag!( &[0xff, 11][..] )       ~
    identifier: map_res!(take!(8), from_utf8) ~
    code:       take!(3)                      ~
    data:       length_value                  ~
                tag!( &[0][..] )              ,
    || {
      Block::ApplicationExtension(Application{
        identifier:          identifier,
        authentication_code: code,
        data:                data
      })
    }
  )
}

named!(graphic_control<&[u8], GraphicControl>,
  chain!(
                  tag!( &[0xf9, 4][..] ) ~
    fields:       be_u8                  ~
    delay:        le_u16                 ~
    transparency: be_u8                  ~
                  tag!( &[0][..] )       ,
    || {
      GraphicControl {
        disposal_method:    (fields & 0b00011100) >> 2,
        user_input:         fields & 0b00000010 == 0b00000010,
        transparency:       fields & 0b00000001 == 0b00000001,
        delay_time:         delay,
        transparency_index: transparency
      }
    }
  )
);

named!(image_descriptor<&[u8], ImageDescriptor>,

  chain!(
            tag!( &[0x2C][..] ) ~
    left:   le_u16              ~
    top:    le_u16              ~
    width:  le_u16              ~
    height: le_u16              ~
    fields: be_u8               ~
    color_table:
      cond!(
        fields & 0b10000000 == 0b10000000,
        count!(
          chain!(
            r: be_u8 ~
            g: be_u8 ~
            b: be_u8 ,
            || {
              //Color { r: r, g: g, b: b }
              let mut v: Vec<u8> = Vec::new();
              v.push(r);
              v.push(g);
              v.push(b);
              v
            }
          ),
          2u16.pow((1u16 + (fields as u16 & 0b00000111)) as u32) as usize
        )
      ),
    || {
      ImageDescriptor{
        left_position:          left,
        top_position:           top,
        width:                  width,
        height:                 height,
        local_color_table_flag: fields & 0b10000000 == 0b10000000,
        interlace:              fields & 0b01000000 == 0b01000000,
        sort:                   fields & 0b00100000 == 0b00100000,
        local_color_table_size: 2u32.pow((1 + (fields as u16 & 0b00000111)) as u32),
        local_color_table:      color_table
      }
    }
  )
);

pub fn not_null(input: &[u8]) -> IResult<&[u8], u8> {
  if input.len() == 0 {
    IResult::Incomplete(Needed::Size(1))
  } else if input[0] == 0 {
    IResult::Error(Err::Code(0))
  } else {
    IResult::Done(&input[1..], input[0])
  }
}

pub fn not_null_length_value(input:&[u8]) -> IResult<&[u8], &[u8]> {
  let input_len = input.len();
  if input_len == 0 {
    return IResult::Error(Err::Code(0))
  }
  if input[0] == 0 {
    //println!("found empty sub block");
    return IResult::Error(Err::Code(0))
    //return IResult::Done(&input[1..], b"")
  }

  let len = input[0] as usize;
  if input_len - 1 >= len {
    //println!("found {} length in:\n{}", len, (&input[0..len+1]).to_hex(8));
    //println!("remaining:\n{}", (&input[len+1..len+20]).to_hex(8));
    return IResult::Done(&input[len+1..], &input[1..len+1])
  } else {
    return IResult::Incomplete(Needed::Size(1+len as usize))
  }
}

named!(sub_blocks<&[u8], Vec<&[u8]> >,
  many0!( not_null_length_value )
);

named!(table_based_image<&[u8], GraphicRenderingBlock>,
  chain!(
    descriptor:    image_descriptor ~
    lzw_code_size: le_u8            ~
    compressed:    sub_blocks       ~
                tag!( &[0][..] )    ,
    || {
      GraphicRenderingBlock::TableBasedImage(descriptor, lzw_code_size, compressed)
    }
  )
);
named!(graphic_rendering_block<&[u8], GraphicRenderingBlock>,
  alt!(
    table_based_image
    //image_descriptor => { |image| GraphicRenderingBlock::TableBasedImage(image) }
  //| tag!("abcd")     =>  |a| { GraphicRenderingBlock::PlainTextExtension }
  )
);

pub fn graphic_block(input:&[u8]) -> IResult<&[u8], Block> {
  println!("data for graphic block:\n{}", &input[..100].to_hex(8));
  chain!(input,
    control:   graphic_control ?       ~
    rendering: graphic_rendering_block ,
    || {
      Block::GraphicBlock(control, rendering)
    }
  )
}

pub fn block(input:&[u8]) -> IResult<&[u8], Block> {
  chain!(input,
    tag!( "!" ) ~
  blk: alt!(
    application_extension
  | graphic_block
  ),
  || {
    blk
  }
  )
}

pub fn many_blocks(i: &[u8]) -> IResult<&[u8],Vec<Block> > {
  many0!(i, block)
}

#[cfg(test)]
mod tests {
  use super::*;
  use lzw;
  use nom::{HexDisplay,IResult};

  #[test]
  fn header_test() {
    let data = include_bytes!("../assets/axolotl-piano.gif");
    println!("bytes:\n{}", &data[0..100].to_hex(8));
    let res = header(data);


    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n{}", &i[0..100].to_hex(8));
        println!("parsed: {:?}", o);
      },
      _  => {
        println!("error or incomplete");
        panic!("cannot parse header");
      }
    }
  }

  #[test]
  fn header_test2() {
    let data = include_bytes!("../assets/test.gif");
    println!("bytes:\n{}", &data[0..100].to_hex(8));
    let res = header(data);


    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n{}", &i[0..100].to_hex(8));
        println!("parsed: {:?}", o);
      },
      _  => {
        println!("error or incomplete");
        panic!("cannot parse header");
      }
    }
  }

  #[test]
  fn logical_screen_descriptor_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
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
  fn logical_screen_descriptor_test2() {
    let d = include_bytes!("../assets/test.gif");
    let data = &d[6..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = logical_screen_descriptor(&data);


    match res {
      IResult::Done(i, o) => {
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
        assert_eq!(o, LogicalScreenDescriptor {
          width:                  2,
          height:                 2,
          gct_flag:               true,
          color_resolution:       7,
          gct_sorted:             false,
          gct_size:               256,
          background_color_index: 0,
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
  fn color_table_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
    let data = &d[13..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    // we know the color table size
    let res = color_table(data, 256);
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
  fn color_table_test2() {
    let d = include_bytes!("../assets/test.gif");
    let data = &d[13..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    // we know the color table size
    let res = color_table(data, 256);
    match res {
      IResult::Done(i, o) => {
        println!("offset: {:?}", d.offset(i));
        //println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
        //panic!("hello");
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn application_block_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
    let data = &d[781..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = block(data);
    match res {
      IResult::Done(i, o) => {
        println!("offset: {:?}", d.offset(i));
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
        assert_eq!(Block::ApplicationExtension(Application {identifier: "NETSCAPE", authentication_code: &[50, 46, 48][..], data: &[1, 0, 0][..] }),
          o);
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn application_block_test2() {
    let d = include_bytes!("../assets/test.gif");
    let data = &d[781..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = block(data);
    match res {
      IResult::Done(i, o) => {
        println!("offset: {:?}", d.offset(i));
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        println!("parsed: {:?}", o);
        assert_eq!(Block::ApplicationExtension(Application {identifier: "NETSCAPE", authentication_code: &[50, 46, 48][..], data: &[1, 0, 0][..] }),
          o);
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn graphic_block_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
    let data = &d[800..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = block(data);
    match res {
      IResult::Done(i, o) => {
        println!("offset: {:?}", d.offset(i));
        println!("parsed: {:?}", o);
        println!("remaining:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));
        //panic!("hello");
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn graphic_block_test2() {
    let d = include_bytes!("../assets/test.gif");
    let data = &d[800..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = block(data);
    match res {
      IResult::Done(i, o) => {
        println!("offset1: {:?}", d.offset(i));
        println!("parsed1: {:?}", o);
        //println!("remaining1:\n{}", &i[0..100].to_hex_from(8, d.offset(i)));

        let res2 = block(i);
        match res2 {
          IResult::Done(i2, o2) => {
            println!("offset2: {:?}", d.offset(i2));
            println!("parsed2: {:?}", o2);
            //println!("remaining2:\n{}", &i2[0..100].to_hex_from(8, d.offset(i2)));
            //println!("remaining2:\n{}", &i2.to_hex(8));
            //panic!("hello");
          },
          e  => {
            println!("error or incomplete: {:?}", e);
            panic!("cannot parse graphic block");
          }
        }
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse graphic block");
      }
    }
  }

  #[test]
  fn multiple_blocks_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
    let data = &d[781..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    let res = many_blocks(data);
    match res {
      IResult::Done(i, o) => {
        println!("offset: {:?}", d.offset(i));
        println!("parsed: {:?}", o);
        println!("remaining:\n{}", i.to_hex(8));
        //panic!("hello");
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn decode_lzw_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
    let data = &d[13..];
    println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    // we know the color table size
    match color_table(data, 256) {
      IResult::Done(i, colors) => {
        //println!("parsed: {:?}", colors);
        // allocate the image
        let mut buffer: Vec<u8> = Vec::with_capacity(400 * 300 * 3);
        unsafe { buffer.set_len(400 * 300 * 3); }

        let data = &d[801..];
        //println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

        match graphic_block(data) {
          IResult::Done(i, Block::GraphicBlock(opt_control, rendering)) => {
            //let (opt_control, rendering) = grb;
            match rendering {
              GraphicRenderingBlock::TableBasedImage(descriptor, code_size, blocks) => {
                match lzw::decode_lzw(colors, code_size as usize, blocks, &mut buffer[..]) {
                  Ok(nb) => {
                    println!("decoded the image({} bytes):\n{}", nb, buffer.to_hex(8));
                    //panic!("correctly decoded")
                  },
                  _ => panic!("could not decode")
                }
              },
              _ => {
                panic!("plaintext extension");
              }
            }
          },
          e  => {
            println!("error or incomplete: {:?}", e);
            panic!("cannot parse graphic block");
          }

        }
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }

  #[test]
  fn decode_lzw_test2() {
    let d = include_bytes!("../assets/test.gif");
    let data = &d[13..];
    //println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

    // we know the color table size
    match color_table(data, 256) {
      IResult::Done(i, colors) => {
        println!("parsed: {:?}", colors);
        // allocate the image
        let mut buffer: Vec<u8> = Vec::with_capacity(2 * 2 * 3);
        unsafe { buffer.set_len(2 * 2 * 3); }

        let data = &d[801..];
        //println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

        match graphic_block(data) {
          IResult::Done(i, Block::GraphicBlock(opt_control, rendering)) => {
            //let (opt_control, rendering) = grb;
            println!("control: {:?}", opt_control);
            println!("rendering: {:?}", rendering);
            match rendering {
              GraphicRenderingBlock::TableBasedImage(descriptor, code_size, blocks) => {
                println!("descriptor: {:?}", descriptor);
                match lzw::decode_lzw(colors, code_size as usize, blocks, &mut buffer[..]) {
                  Ok(nb) => {
                    println!("decoded the image({} bytes):\n{}", nb, buffer.to_hex(8));
                    //panic!("correctly decoded")
                  },
                  _ => panic!("could not decode")
                }
              },
              _ => {
                panic!("plaintext extension");
              }
            }
          },
          e  => {
            println!("error or incomplete: {:?}", e);
            panic!("cannot parse graphic block");
          }

        }
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        panic!("cannot parse global color table");
      }
    }
  }
}
