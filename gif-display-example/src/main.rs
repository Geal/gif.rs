extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;
extern crate sdl2_window;
extern crate piston;

extern crate gif;
extern crate nom;

use nom::IResult;
use nom::util::*;
use gif::*;
use gif::parser::*;
use gif::lzw::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::vec::Vec;
use glium::{ Surface, Texture2d };
use glium_graphics::{ Glium2d, GliumGraphics, DrawTexture, GliumWindow };
use sdl2_window::{ Sdl2Window, OpenGL };
use piston::event::*;
use piston::window::{ WindowSettings, Size };

fn main() {
    println!("starting...");
    let opengl = OpenGL::_3_2;
    let (w, h) = (400, 300);
    let ref window = Rc::new(RefCell::new(
        Sdl2Window::new(
            opengl,
            WindowSettings::new(
                "glium_graphics: image_test".to_string(),
                Size { width: w, height: h }
            ).exit_on_esc(true)
        )
    ));
    let ref glium_window = GliumWindow::new(window).unwrap();

    println!("A");
    let rust_logo = DrawTexture::new({
        let image = image::open(&Path::new("../assets/rust.png")).unwrap();
        Texture2d::new(glium_window, image)
    });

    println!("B");
    let pixels = DrawTexture::new({
        let decoded = decode_gif();
        let mut reversed:Vec< Vec<(u8,u8,u8)> > = Vec::new();
        for el in decoded.iter().rev() {
          reversed.push(el.clone());
        }
        Texture2d::new(glium_window, reversed)
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

    println!("C");
    let mut g2d = Glium2d::new(OpenGL::_3_2, glium_window);
    println!("w: {:?}, h: {:?}", w, h);
    let transform = graphics::math::abs_transform(w as f64, h as f64);
    println!("f: w: {:?}, h: {:?}", w as f64, h as f64);

 
    for e in window.events().swap_buffers(false) {
      if let Some(_) = e.render_args() {
        let mut target = glium_window.draw();
        {
            use graphics::*;

            //println!("E");
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
            //println!("F");
            image(&rust_logo, transform.trans(0.0, 0.0), &mut g);
            image(&pixels, transform.trans(0.0, 0.0), &mut g);
            //println!("G");

        }
        target.finish();
      }
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
              match lzw::decode_lzw(colors, code_size as usize, blocks, &mut buffer[..]) {
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
