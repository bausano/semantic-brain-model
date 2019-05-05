use senses::visual::point::Point;
use senses::visual::helpers::pixel_value;
use senses::visual::visual_object::VisualObject;

type PointMap = Vec<Vec<bool>>;

/// Finds objects within given image heatmap. Uses flood fill algorithm which,
/// after finding any highlighted unvisited point within the image, selects all
/// highlighted other points in the neighbourhood.
pub fn extract_highlights(image: Vec<Vec<u32>>) -> Vec<VisualObject> {
  // Currently iterated point in the image.
  let mut current_point: Point = Point::new(0, 0);
  // Converts the u32 vector into a boolean one.
  let mut image: PointMap = to_point_map(image);
  // Instantiates the return vector.
  let mut objects: Vec<VisualObject> = Vec::new();
  // Servers as image dimensions.
  let last_point: Point = Point::new(
    image[0].len() as u32 - 1,
    image.len() as u32 - 1,
  );

  // As long as the currently iterated point is not the last one, run the cycle.
  while current_point != last_point {
    // If the value at currently iterated point is positive, flood fill the
    // object and remove it from the original map.
    if pixel_value(&image, current_point.x as isize, current_point.y as isize, false) {
      let mut object: VisualObject = VisualObject::new();
      flood_fill(current_point, &mut object, &mut image);
      objects.push(object);
    }

    // Increments the row starting from 0 if current_point reached the end of
    // the line otherwise moves to the pixel to the right.
    if current_point.x == last_point.x {
      current_point.x = 0;
      current_point.y += 1;
    } else {
      current_point.x += 1;
    }
  }

  objects
}

/// Recursively finds a single object within given image. It calls this function
/// for every new highlighted point.
fn flood_fill(point: Point, object: &mut VisualObject, image: &mut PointMap) {
  // Adds currently iterated point to the object and set that point to no
  // highlighted.
  object.push(point);
  image[point.y as usize][point.x as usize] = false;

  // Iterates over the Moore neighbourhood of currently iterated point.
  for y in (point.y as isize - 1)..(point.y as isize + 2) {
    if y < 0 {
      continue;
    }

    for x in (point.x as isize - 1)..(point.x as isize + 2) {
      // If the Moore's point is not highlighted, skips.
      if x < 0 || !pixel_value(image, x, y, false) {
        continue;
      }

      // Visits the Moore's point.
      flood_fill(Point::new(x as u32, y as u32), object, image);
    }
  }
}

/// Converts multi dimensional vector of integers that represent colour into
/// a multi dimensional vector of booleans where true means given point is
/// highlighted.
fn to_point_map(input: Vec<Vec<u32>>) -> PointMap {
  input.iter().map(
    |row| row.iter().map(|point| *point != 0).collect()
  ).collect()
}

