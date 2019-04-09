extern crate image;

mod helpers;
mod heat_map;
mod find_edges;
mod visual_object;
mod cellular_automaton;

use senses::file::File;
use senses::visual::heat_map::heat_map;
use senses::visual::find_edges::find_edges;
use senses::visual::visual_object::VisualObject;
use senses::visual::cellular_automaton::cellular_automaton;

use self::image::{
  Luma,
  imageops,
  GrayImage,
  ImageBuffer,
};

type GrayImageRaw = Vec<Vec<u32>>;

pub fn get_objects_from_image(file: File) -> Vec<VisualObject> {
  let image = image::open(file.get_full_path())
    .expect("Could not open image.");

  let edge_detector = find_edges(&image);

  // TODO: Remove
  edge_detector.save("output/test/".to_owned() + &file.get_name() + "_edge." + &file.get_extension()).unwrap();

  let _objects_detector = highlight_objects(&edge_detector);

  vec!(
    VisualObject::new(15, 15),
  )
}

/// Highlights zones with crucial objects in the image. Does so by creating a
/// heat map (the more dense the pixels, the larger heat) and the runs the heat
/// map though a cellular automaton with set rules that stabilize all cells into
/// two states: alive (255) or dead (0).
fn highlight_objects(image: &GrayImage) -> GrayImageRaw {
  let (width, height) = image.dimensions();
  let mut object_detector: GrayImage = ImageBuffer::new(width, height);

  // From the bricked heat map creates more detailed one where each cell is half
  // of the size of those in the bricked heat map. This multi-dimensional vector
  // represents density of edges in the original image.
  // Also returns maximum heat observed in the map and an average heat. This is
  // used for calculating the rules of the cellular automaton.
  let (heat_map, heat_max, heat_mean) = heat_map(&image);

  // Stabilizes each cell into one of two states.
  let highlight = cellular_automaton(heat_map, heat_max, heat_mean);

  // TODO: Remove
  let unit: f64 = 255_f64 / heat_max as f64;
  for (x, y, pixel) in object_detector.enumerate_pixels_mut() {
    let col = (x / (10 / 2)) as usize;
    let row = (y / (10 / 2)) as usize;
    let heat = highlight[row][col];
    let mut heat: u8 = (unit * heat as f64) as u8;
    *pixel = Luma([heat]);
  }
  imageops::colorops::invert(&mut object_detector);
  object_detector.save("output/test/objects.png").unwrap();

  highlight
}
