use std::fmt;
use senses::visual::point::Point;

pub struct VisualObject {

  /// Maps the point to the original picture.
  pub reference: Point,

  /// Vector off all points the object contains.
  points: Vec<Point>,

  /// First point is the left most point with the lowest y value.
  /// Second point the right most point with the highest y value.
  size: Option<(Point, Point)>,

}

impl VisualObject {

  /// Factory function initializing new object from given dimensions.
  pub fn new(reference: Point) -> VisualObject {
    VisualObject {
      points: Vec::new(),
      size: None,
      reference,
    }
  }

  /// Pushes new point to the object and refreshes size cache.
  pub fn push(&mut self, point: Point) {
    self.points.push(point);
    self.size = None;
  }

  /// Returns a tuple with points defining the smallest encapsulating rectangle,
  /// meaning all points are within this rectangle.
  pub fn size(&mut self) -> Option<(Point, Point)> {
    if self.points.len() == 0 {
      return None;
    }

    if self.size.is_some() {
      return self.size;
    }

    let mut lowest: Option<Point> = None;
    let mut highest: Option<Point> = None;

    // We iterate over all points and find the lowest and highest combination.
    // The result will be a rectangle that fits the whole object.
    for point in self.points.iter() {
      lowest = match lowest {
        None => Some(*point),
        Some(lowest) => {
          Some(Point::new(lowest.x.min(point.x), lowest.y.min(point.y)))
        }
      };

      highest = match highest {
        None => Some(*point),
        Some(highest) => {
          Some(Point::new(highest.x.max(point.x), highest.y.max(point.y)))
        }
      };
    }

    // We made sure that there is at least one point in the object, therefore
    // lowest and highest will never be None.
    self.size = Some((lowest.unwrap(), highest.unwrap()));

    self.size
  }

  pub fn point_map(&mut self) -> Option<Vec<Vec<bool>>> {
    let size = self.size();

    if size.is_none() {
      return None;
    }

    let (lower, higher) = size.unwrap();

    let mut map: Vec<Vec<bool>> = Vec::new();

    for y in 0..(higher.y - lower.y) {
      map.push(vec!(false; (higher.x - lower.x) as usize));
    }

    for point in self.points.iter() {
      let y: usize = (point.y - lower.y) as usize;
      let x: usize = (point.x - lower.x) as usize;
      map[y][x] = true;
    }

    Some(map)
  }

}

impl fmt::Debug for VisualObject {

  // Implements debug message for VisualObject.
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_fmt(
      format_args!("VisualObject | {} points", self.points.len())
    )
  }

}
