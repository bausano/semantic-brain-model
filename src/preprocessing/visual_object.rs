use std::fmt;

pub struct VisualObject {

  /// Width and height of the square the object is extracted in.
  size: (u8, u8)

}

impl VisualObject {

  /// Factory function initializing new object from given dimensions.
  pub fn new(width: u8, height: u8) -> VisualObject {
    // TODO: Replace with 2 dimensional vector and derive the dimensions
    //       from that. Also check that all rows have same length.
    VisualObject {
      size: (width, height),
    }
  }

}

impl fmt::Debug for VisualObject {

  // Implements debug message for VisualObject.
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_fmt(
      format_args!("VisualObject ({}x{})", self.size.0, self.size.1)
    )
  }

}
