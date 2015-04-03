#![feature(std_misc, thread_sleep)]

extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;

extern crate gif;
use gif::*;
use gif::lzw::*;

use std::path::Path;
use std::thread::sleep;
use std::time::duration::Duration;
use glium::{ DisplayBuild, Surface, Texture2d };
use glium_graphics::{ Glium2d, GliumGraphics, DrawTexture, OpenGL };

fn main() {
    let builder = glutin::WindowBuilder::new();
    let window = builder
        .with_dimensions(400, 300)
        .with_title("glium_graphics: image_test".to_string())
        .build_glium().unwrap();


    let rust_logo = DrawTexture::new({
        let image = image::open(&Path::new("assets/rust.png")).unwrap();
        Texture2d::new(&window, image)
    });

    let pixels = DrawTexture::new({
        Texture2d::new(&window, decode_gif())
    });
    /*let pixels = DrawTexture::new({
        Texture2d::new(&window,
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
        ])
    });*/

    let mut g2d = Glium2d::new(OpenGL::_3_2, &window);
    let (w, h) = window.get_framebuffer_dimensions();
    println!("w: {:?}, h: {:?}", w, h);
    let transform = graphics::abs_transform(w as f64, h as f64);
    println!("f: w: {:?}, h: {:?}", w as f64, h as f64);

    loop {
        let mut target = window.draw();
        {
            use graphics::*;

            let mut g = GliumGraphics::new(&mut g2d, &mut target);

            clear(color::WHITE, &mut g);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      transform,
                      &mut g);
            rectangle([0.0, 1.0, 0.0, 0.3],
                      [50.0, 50.0, 100.0, 100.0],
                      transform,
                      &mut g);
            image(&rust_logo, transform.trans(0.0, 0.0), &mut g);
            image(&pixels, transform.trans(0.0, 0.0), &mut g);

        }
        target.finish();

        window.poll_events().last();
        if window.is_closed() {
            break
        }
        sleep(Duration::milliseconds(15));
    }
}

pub fn buf_to_colors(input: &buffer[u8], row_size: u8) -> Vec< Vec<u8> > {
  let mut res: Vec< Vec<u8> > = Vec::new();
  for row in buffer.chunks(row_size * 3) {
    let mut v: Vec<u8> = Vec::new();
    for pixel in row.chunks(3) {
      v.push( ( pixel[0], pixel[1], pixel[2]) );
    }
    row.push(v);
  }
  res
}

pub fn decode_gif () -> Vec< Vec<u8> > {
  let d = include_bytes!("../axolotl-piano.gif");
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
              match lzw::decode_lzw(colors, code_size, blocks, &mut buffer[..]) {
                Some(nb) => {
                  println!("decoded the image({} bytes):\n{}", nb, buffer.to_hex(8));
                  return buf_to_colors(buffer, 400);
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
