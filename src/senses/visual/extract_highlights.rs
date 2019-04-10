use senses::visual::point::Point;
use senses::visual::helpers::pixel_value;
use senses::visual::visual_object::VisualObject;

type PointMap = Vec<Vec<bool>>;

pub fn extract_highlights(image: Vec<Vec<u32>>) -> Vec<VisualObject> {
  // Currently iterated point in the image.
  let mut c_point: Point = Point::new(0, 0);
  // Converts the u32 vector into a boolean one.
  let mut image: PointMap = to_point_map(image);
  // Instantiates the return vector.
  let mut objects: Vec<VisualObject> = Vec::new();
  // Servers as image dimensions.
  let last_point: Point = Point::new(
    image[0].len() as u32,
    image.len() as u32
  );

  // As long as the currently iterated point is not the last one, run the cycle.
  while c_point != last_point {
    // If the value at currently iterated point is positive, flood fill the
    // object and remove it from the original map.
    if pixel_value(&image, c_point.x as isize, c_point.y as isize, false) {
      objects.push(flood_fill(&c_point, &mut image))
    }

    // Increments the row starting from 0 if c_point reached the end of the line
    // otherwise moves to the pixel to the right.
    if c_point.x == last_point.x {
      c_point.x = 0;
      c_point.y += 1;
    } else {
      c_point.x += 1;
    }
  }

  objects
}

fn flood_fill(starting_point: &Point, image: &mut PointMap) -> VisualObject {
  VisualObject::new(0, 0)
}

fn to_point_map(input: Vec<Vec<u32>>) -> PointMap {
  input.iter().map(
    |row| row.iter().map(|point| *point != 0).collect()
  ).collect()
}

