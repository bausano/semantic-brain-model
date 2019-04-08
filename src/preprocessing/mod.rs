//! Module for preprocessing the data before feeding it into the model.
//! This module will eventually be extracted out and the preprocessing will
//! happen on a dedicated machine.

extern crate hound;
extern crate image;

use self::image::Luma;
use self::image::GenericImageView;
use self::image::imageops::{grayscale, filter3x3};

pub fn start_data_channel() {
  let image = image::open("data/debug-1/video/output_0081.png").unwrap();

  let (width, height) = image.dimensions();

  let mut imgbuf = image::ImageBuffer::new(width, height);

  let horizontal_edges = filter3x3(&grayscale(&image), &[
    10_f32, 10_f32, 10_f32,
    1_f32, 1_f32, 1_f32,
    -10_f32, -10_f32, -10_f32,
  ]);

  let vertical_edges = filter3x3(&grayscale(&image), &[
    10_f32, 1_f32, -10_f32,
    10_f32, 1_f32, -10_f32,
    10_f32, 1_f32, -10_f32,
  ]);

  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let vertical_edge = vertical_edges.get_pixel(x, y).data[0];
    let horizontal_edge = horizontal_edges.get_pixel(x, y).data[0];
    let max = vertical_edge.max(horizontal_edge);
    let min = vertical_edge.min(horizontal_edge);

    *pixel = if max == 255 || min == 0 {
      Luma([0])
    } else {
      Luma([255])
    };
  }

  imgbuf.save("test.png").unwrap();
}

fn sound() {
  let mut reader = hound::WavReader::open("data/debug-1/audio/output.wav").unwrap();
  let sqr_sum = reader.samples::<i16>()
                      .fold(0.0, |sqr_sum, s| {
      let sample = s.unwrap() as f64;
      (sqr_sum as f64).min(sample)
  });
  println!("RMS is {}, len is {}", sqr_sum, reader.len());
}
