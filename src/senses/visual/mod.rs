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

pub fn find_objects(file: File) -> Vec<VisualObject> {
  let image = image::open(file.full_path())
    .expect("Could not open image.");

  let edge_detector = find_edges(&image);

  // From the bricked heat map creates more detailed one where each cell is half
  // of the size of those in the bricked heat map. This multi-dimensional vector
  // represents density of edges in the original image.
  // Also returns maximum heat observed in the map and an average heat. This is
  // used for calculating the rules of the cellular automaton.
  let (heat_map, heat_max, heat_mean) = heat_map(&edge_detector);

  // Stabilizes each cell into one of two states.
  let _highlight = cellular_automaton(heat_map, heat_max, heat_mean);

  vec!(
    VisualObject::new(15, 15),
  )
}
