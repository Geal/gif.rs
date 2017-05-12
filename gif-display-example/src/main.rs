#[macro_use]
extern crate glium;
extern crate gif;
extern crate nom;


use glium::{DisplayBuild, Surface};

use glium::glutin;

use nom::{IResult,HexDisplay,Offset};
//use nom::util::*;
use gif::*;
use gif::parser::*;
use gif::lzw::*;

fn main() {
  // building the display, ie. the main object
  let display = glutin::WindowBuilder::new()
    .with_dimensions(400, 300)
    .with_vsync()
    .build_glium()
    .unwrap();

  let decoded = decode_gif();
  let mut reversed:Vec< Vec<(u8,u8,u8)> > = Vec::new();
  for el in decoded.iter().rev() {
    reversed.push(el.clone());
  }
  let dest_texture = glium::Texture2d::new(&display, reversed).unwrap();

  /*
  let dest_texture = glium::Texture2d::new(&display,
  vec![
  vec![(0u8, 1u8, 2u8), (4u8, 8u8, 16u8), (4u8, 8u8, 16u8), (4u8, 8u8, 16u8)],
  vec![(32u8, 64u8, 128u8), (32u8, 16u8, 4u8), (4u8, 8u8, 16u8), (4u8, 8u8, 16u8)],
  vec![(32u8, 64u8, 128u8), (32u8, 16u8, 4u8), (4u8, 8u8, 16u8), (4u8, 8u8, 16u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)],
  vec![(255u8, 64u8, 2u8), (10u8, 255u8, 255u8), (128u8, 8u8, 255u8), (4u8, 8u8, 255u8)]
  ]).unwrap();
  */

  // the main loop
  loop {
    // drawing a frame
    let target = display.draw();
    dest_texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
    target.finish().unwrap();

    // polling and handling the events received by the window
    for event in display.poll_events() {
      match event {
        glutin::Event::Closed => return,
        _ => ()
      }
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
  }
}

pub fn buf_to_colors(buffer: &[u8], row_size: usize) -> Vec< Vec<(u8,u8,u8)> > {
  let mut res: Vec< Vec<(u8,u8,u8)> > = Vec::new();
  println!("chunking by {} bytes", row_size * 3);
  for row in buffer.chunks(row_size as usize * 3) {
    if row.len() != row_size as usize * 3 || res.len() == 300 {
      break;
    }
    let mut v: Vec<(u8,u8,u8)> = Vec::new();
    for pixel in row.chunks(3) {
      v.push( ( pixel[0], pixel[1], pixel[2]) );
    }
    assert_eq!(row_size as usize, v.len());
    //println!("adding {:?}", v);
    res.push(v);
  }
  println!("{} rows of {} pixels", res.len(), row_size);
  res
}

pub fn decode_gif () -> Vec< Vec<(u8,u8,u8)> > {
  let d = include_bytes!("../../assets/axolotl-piano.gif");
  //let d = include_bytes!("../../assets/test.gif");
  let data = &d[13..];
  println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

  // we know the color table size
  match color_table(data, 256) {
    IResult::Done(_, colors) => {
      //println!("parsed: {:?}", colors);
      // allocate the image
      let mut buffer: Vec<u8> = Vec::with_capacity(400 * 300 * 3);
      unsafe { buffer.set_len(400 * 300 * 3); }

      let data = &d[801..];
      //println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

      match graphic_block(data) {
        IResult::Done(_, Block::GraphicBlock(opt_control, rendering)) => {
          //let (opt_control, rendering) = grb;
          match rendering {
            GraphicRenderingBlock::TableBasedImage(descriptor, code_size, blocks) => {
              match lzw::decode_lzw(&colors, code_size as usize, blocks, &mut buffer[..]) {
                Ok(nb) => {
                  println!("decoded the image({} bytes):\n", nb);//, buffer.to_hex(8));
                  return buf_to_colors(&mut buffer[..], 400);
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
