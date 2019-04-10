
pub struct Point {
  pub x: u32,
  pub y: u32,
}

impl Point {

  pub fn new(x: u32, y: u32) -> Point {
    Point { x, y }
  }

}

impl PartialEq for Point {

  fn eq(&self, other: &Point) -> bool {
    self.x == other.x && self.y == other.y
  }

}
