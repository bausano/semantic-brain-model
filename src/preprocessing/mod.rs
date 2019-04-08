//! Module for preprocessing the data before feeding it into the model.
//! This module will eventually be extracted out and the preprocessing will
//! happen on a dedicated machine.

extern crate hound;
extern crate image;

use self::image::imageops::{grayscale, filter3x3};
use self::image::{Luma, ImageBuffer, GenericImageView};
use self::image::imageops::colorops::invert;

pub fn start_data_channel() {
  let file = "output_0331";
  let image = image::open("data/debug-1/video/".to_owned() + file.clone() + ".png").unwrap();

  let (width, height) = image.dimensions();

  let mut heat_detector: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
  let mut edge_detector = ImageBuffer::new(width, height);

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

  for (x, y, pixel) in edge_detector.enumerate_pixels_mut() {
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

  edge_detector.save("output/test/".to_owned() + file + "_edge.png").unwrap();

  let cell_size = 20;
  let heat_map_cols = height / cell_size;
  let heat_map_rows = width / cell_size;

  let mut heat_map: Vec<Vec<u32>> = Vec::new();
  let mut heat_max: u32 = 0;

  for offset_y in 0..heat_map_cols {
    let mut row: Vec<u32> = Vec::new();

    for offset_x in 0..heat_map_rows {
      let mut heat: u32 = 0;

      for cell_y in 0..cell_size {
        for cell_x in 0..cell_size {
          let pixel = edge_detector.get_pixel(
            (offset_x * cell_size) + cell_x,
            (offset_y * cell_size) + cell_y,
          );

          if pixel.data[0] == 0 {
            heat += 1;
          }
        }
      }

      heat_max = heat_max.max(heat.clone());

      row.push(heat);
    }

    heat_map.push(row);
  }

  let unit: f64 = 255_f64 / heat_max as f64;

  for (x, y, pixel) in heat_detector.enumerate_pixels_mut() {
    let col = (x / cell_size) as usize;
    let row = (y / cell_size) as usize;
    let mut heat: u8 = (unit * heat_map[row][col] as f64) as u8;

    *pixel = Luma([heat]);
  }

  invert(&mut heat_detector);

  heat_detector.save("output/test/".to_owned() + file + "_heat.png").unwrap();

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
