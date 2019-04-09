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

  let mut image_grey = grayscale(&image);

  for pixel in image_grey.pixels_mut() {
    if pixel.data[0] < 5 {
      *pixel = Luma([5]);
    }
  }

  let horizontal_edges = filter3x3(&image_grey, &[
    10_f32, 10_f32, 10_f32,
    1_f32, 1_f32, 1_f32,
    -10_f32, -10_f32, -10_f32,
  ]);

  let vertical_edges = filter3x3(&image_grey, &[
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

  let cell_size = 10;
  let heat_map_cols = (2 * width / cell_size) - 1;
  let heat_map_rows = (2 * height / cell_size) - 1;

  let mut heat_map: Vec<Vec<u32>> = Vec::new();

  for offset_y in 0..heat_map_rows {
    let mut row: Vec<u32> = Vec::new();

    for offset_x in 0..heat_map_cols {
      let mut heat: u32 = 0;

      for cell_y in 0..cell_size {
        for cell_x in 0..cell_size {
          let pixel = edge_detector.get_pixel(
            (offset_x * cell_size / 2) + cell_x,
            (offset_y * cell_size / 2) + cell_y,
          );

          if pixel.data[0] == 0 {
            heat += 1;
          }
        }
      }

      row.push(heat);
    }

    heat_map.push(row);
  }

  let heat_map_cols = heat_map_cols + 1;
  let heat_map_rows = heat_map_rows + 1;
  let mut final_heat_map: Vec<Vec<u32>> = Vec::new();
  let mut heat_max: u32 = 1;
  let mut heat_mean: u32 = 0;
  let mut heat_counter: u32 = 0;

  for offset_y in 0..heat_map_rows {
    let mut row: Vec<u32> = vec!(0);

    for offset_x in 1..(heat_map_cols - 1) {
      if offset_y == 0 || offset_y == heat_map_rows - 1 {
        row.push(0);
        continue;
      }

      let mut heat: u32 = 0;
      heat += heat_map[offset_y as usize][offset_x as usize];
      heat += heat_map[offset_y as usize][(offset_x - 1) as usize];
      heat += heat_map[(offset_y - 1) as usize][offset_x as usize];
      heat += heat_map[(offset_y - 1) as usize][(offset_x - 1) as usize];
      let heat: u32 = heat / 4;

      heat_max = heat_max.max(heat.clone());

      if heat > 0 {
        heat_mean += heat;
        heat_counter += 1;
      }

      row.push(heat);
    }

    row.push(0);

    final_heat_map.push(row);
  }

  heat_mean /= heat_counter;

  loop {
    let mut step: Vec<Vec<u32>> = Vec::new();
    let mut stabilized = true;

    for (y, row) in final_heat_map.iter().enumerate() {
      let mut row_clone: Vec<u32> = Vec::new();
      for (x, heat) in row.iter().enumerate() {
        if *heat == heat_max || *heat == 0 {
          row_clone.push(*heat);
          continue;
        }

        let rating: u32 = calculate_mean_heat_around_cell(x, y, &final_heat_map);

        if rating < heat_mean.min(*heat) {
          row_clone.push(0);
          stabilized = false;
          continue;
        }

        if rating < heat_mean {
          row_clone.push(0.max(*heat as i32 - heat_mean as i32 + rating as i32) as u32);
          stabilized = false;
          continue;
        }

        if rating > heat_mean {
          row_clone.push(heat_max.min(*heat + rating - heat_mean));
          stabilized = false;
          continue;
        }

        row_clone.push(*heat);
      }

      step.push(row_clone);
    }

    if stabilized {
      break;
    }

    final_heat_map = step;
  }

  let unit: f64 = 255_f64 / heat_max as f64;

  for (x, y, pixel) in heat_detector.enumerate_pixels_mut() {
    let col = (x / (cell_size / 2)) as usize;
    let row = (y / (cell_size / 2)) as usize;
    let heat = final_heat_map[row][col];

    let mut heat: u8 = (unit * heat as f64) as u8;

    *pixel = Luma([heat]);
  }

  invert(&mut heat_detector);

  heat_detector.save("output/test/".to_owned() + file + "_heat.png").unwrap();

}

fn calculate_mean_heat_around_cell(x: usize, y: usize, map: &Vec<Vec<u32>>) -> u32 {
  let x: isize = x as isize;
  let y: isize = y as isize;

  (get_value_at(map, x - 1, y - 1, 0) +
  get_value_at(map, x, y - 1, 0) +
  get_value_at(map, x + 1, y - 1, 0) +
  get_value_at(map, x - 1, y, 0) +
  get_value_at(map, x + 1, y, 0) +
  get_value_at(map, x - 1, y + 1, 0) +
  get_value_at(map, x, y + 1, 0) +
  get_value_at(map, x + 1, y + 1, 0)) / 8
}

fn get_value_at(vec: &Vec<Vec<u32>>, x: isize, y: isize, default: u32) -> u32 {
  if x < 0 || y < 0 {
    return default;
  }

  match vec.get(y as usize) {
    None => default,
    Some(row) => match row.get(x as usize) {
      None => default,
      Some(value) => value.clone(),
    },
  }
}
