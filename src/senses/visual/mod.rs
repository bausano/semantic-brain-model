extern crate image;

mod point;
mod helpers;
mod heat_map;
mod find_edges;
mod visual_object;
mod cellular_automaton;
mod extract_highlights;

use senses::file::File;
use senses::visual::point::Point;
use senses::visual::heat_map::heat_map;
use senses::visual::heat_map::CELL_SIZE;
use senses::visual::find_edges::find_edges;
use senses::visual::visual_object::VisualObject;
use senses::visual::extract_highlights::extract_highlights;
use senses::visual::cellular_automaton::cellular_automaton;

use self::image::{GenericImage, ImageBuffer, Rgb, RgbImage};

pub fn identify_objects(file: File) {
  let image = image::open(file.full_path())
    .expect("Could not open image.");

  // Converts the image to grayscale and finds edges within the picture. Works
  // only with bright images. Resulting image has white background with dark
  // edges highlighted.
  let edge_detector = find_edges(&image);

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

  let ref mut img: RgbImage = ImageBuffer::new(640 / (CELL_SIZE / 2), 360 / (CELL_SIZE / 2));
  let len = (255 / highlights.len() + 4) / 2;

  img.pixels_mut().for_each(|mut pixel| *pixel = Rgb([255, 255, 255]));

  for (i, highlight) in highlights.iter_mut().enumerate() {
    for point in highlight.points.iter() {
      img.put_pixel(
        point.x + highlight.reference.x,
        point.y + highlight.reference.y,
        Rgb([
          (3 * len + (i % 3) * i * len) as u8,
          (3 * len + (i + 1 % 3) * i * len) as u8,
          (3 * len + (i + 2 % 3) * i * len) as u8,
        ]),
      )
    }
  }

  img.save("output/test/highlighted.png").unwrap();

  println!("{:?}", highlights.len());

  // TODO: Match each object back to original image.
}
