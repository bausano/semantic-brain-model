/// Helper function for accessing values at given address in vector. If the
/// address is out of bounds, it delivers the default value instead.
pub fn pixel_value<T: Copy>(vec: &Vec<Vec<T>>, x: isize, y: isize, default: T) -> T {
  if x < 0 || y < 0 {
    return default;
  }

  match vec.get(y as usize) {
    None => default,
    Some(row) => match row.get(x as usize) {
      None => default,
      Some(value) => *value,
    },
  }
}
