extern crate image;

mod point;
mod helpers;
mod heat_map;
mod find_edges;
mod visual_object;
mod cellular_automaton;
mod extract_highlights;
mod cut_highlights_from_image;

use senses::file::File;
use senses::visual::point::Point;
use senses::visual::heat_map::heat_map;
use senses::visual::heat_map::CELL_SIZE;
use senses::visual::find_edges::find_edges;
use senses::visual::visual_object::VisualObject;
use senses::visual::extract_highlights::extract_highlights;
use senses::visual::cellular_automaton::cellular_automaton;
use senses::visual::cut_highlights_from_image::cut_highlights_from_image;

use self::image::{ImageBuffer, Rgb, RgbImage};

pub fn identify_objects(file: File) {
  let image = image::open(file.full_path())
    .expect("Could not open image.");

  // Converts the image to grayscale and finds edges within the picture. Works
  // only with bright images. Resulting image has white background with dark
  // edges highlighted.
  let edge_detector = find_edges(&image);

  edge_detector.save("output/test/edges.png").unwrap();

  // From the bricked heat map creates more detailed one where each cell is half
  // of the size of those in the bricked heat map. This multi-dimensional vector
  // represents density of edges in the original image.
  // Also returns maximum heat observed in the map and an average heat. This is
  // used for calculating the rules of the cellular automaton.
  let (heat_map, heat_max, heat_mean) = heat_map(&edge_detector);

  // Stabilizes each cell into one of two states.
  let point_map = cellular_automaton(heat_map, heat_max, heat_mean);

  // Finds objects using a recursive flood fill method.
  let mut highlights: Vec<VisualObject> = Vec::new();
  extract_highlights(
    point_map,
    Point::new(0, 0),
    &mut highlights,
  );

  for (i, highlight) in cut_highlights_from_image(highlights, image).iter().enumerate() {
    highlight.save("output/test/highlight_".to_owned() + &i.to_string() + ".png").unwrap();
  }
}
