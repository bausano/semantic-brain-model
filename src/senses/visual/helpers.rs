type GrayImageRaw = Vec<Vec<u32>>;

/// Helper function for accessing values at given address in vector. If the
/// address is out of bounds, it delivers the default value instead.
pub fn pixel_value(vec: &GrayImageRaw, x: isize, y: isize, default: u32) -> u32 {
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
