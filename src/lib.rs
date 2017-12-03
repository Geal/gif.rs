#[macro_use]
extern crate nom;

extern crate byteorder;

pub mod lzw;
pub mod parser;

use nom::HexDisplay;
use nom::Offset;

enum State {
  Header,
  Images,
  Ended
}

pub struct Decoder<'a> {
  data:       &'a[u8],
  position:   usize,
  state:      State,
  descriptor: parser::LogicalScreenDescriptor,
  images:     Vec<usize>
}

impl<'a> Decoder<'a> {
  /// Creates a new GIF decoder
  pub fn initialize(d: &'a[u8]) -> Option<Decoder<'a>> {
    if let Ok((remaining, descriptor)) = parser::header_and_logical_screen_descriptor(d) {
      Some(Decoder {
        data:       d,
        position:   d.offset(remaining),
        state:      State::Header,
        descriptor: descriptor,
        images:     Vec::new()
      })
    } else {
      None
    }
  }

  pub fn buffer_size(&self) -> usize {
    self.descriptor.width  as usize * self.descriptor.height  as usize * 3
  }

  pub fn next_image(&mut self, buffer: &mut [u8]) -> Option<&mut [u8]> {
    None
  }


}
/*
  fn read_header(&mut self) -> ImageResult<()> {
  }

  fn read_logical_screen_descriptor(&mut self) -> ImageResult<()> {
  }

  fn read_extension(&mut self) -> ImageResult<()> {
  }

  fn read_control_extension(&mut self) -> ImageResult<()> {
  }

  /// Skips an unknown extension
  fn skip_extension(&mut self) -> ImageResult<()> {
  }

  /// Reads data blocks
  fn read_data(&mut self) -> ImageResult<Vec<u8>> {
  }

  fn read_frame(&mut self) -> ImageResult<Frame> {
  }

  fn next_frame(&mut self) -> ImageResult<Option<Frame>> {
  }


  fn dimensions(&mut self) -> ImageResult<(u32, u32)> {
  }

  fn colortype(&mut self) -> ImageResult<color::ColorType> {
  }

  fn row_len(&mut self) -> ImageResult<usize> {
  }

  fn read_scanline(&mut self, _: &mut [u8]) -> ImageResult<u32> {
  }

  fn read_image(&mut self) -> ImageResult<DecodingResult> {
  }
  */
//}

/*
pub struct GIFDecoder<R: Read> {
    r: R,
    state: State,

    width: u16,
    height: u16,
    global_table: Vec<(u8, u8, u8)>,
    global_background_index: Option<u8>,
    delay: u16,
    local_transparent_index: Option<u8>,
}
*/

mod tests {
  use super::*;

  #[test]
  fn parse_test() {
    let d = include_bytes!("../assets/axolotl-piano.gif");
  }

}
