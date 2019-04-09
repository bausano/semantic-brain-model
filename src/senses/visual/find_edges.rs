use senses::visual::image::{
  Luma,
  GrayImage,
  ImageLuma8,
  ImageBuffer,
  DynamicImage,
};

/// Replaces all pixels that are darker/brighter than these thresholds, giving
/// the edge detection extra space to highlight edges.
const DARKEST_GREYSCALE_VALUE: u8 = 5;
const BRIGHTEST_GREYSCALE_VALUE: u8 = 250;

/// How strongly should edges be favored in edge detection algorithm. The larger
/// the value, the more dense the resulting image becomes.
const EDGE_COEF: f32 = 7.5_f32;

/// Finds edges in given grayscale picture by using two 3x3 matrixes. First one
/// detects horizontal edges, the second one vertical.
pub fn find_edges(
  image: &DynamicImage
) -> GrayImage {
  let image = smooth_out_polarized_pixels(&image);

  // Highlights horizontal edges.
  let horizontal_edges = image.filter3x3(&[
    EDGE_COEF, EDGE_COEF, EDGE_COEF,
    1_f32, 1_f32, 1_f32,
    -EDGE_COEF, -EDGE_COEF, -EDGE_COEF,
  ]).to_luma();

  // Highlights vertical edges.
  let vertical_edges = image.filter3x3(&[
    EDGE_COEF, 1_f32, -EDGE_COEF,
    EDGE_COEF, 1_f32, -EDGE_COEF,
    EDGE_COEF, 1_f32, -EDGE_COEF,
  ]).to_luma();

  let (width, height) = horizontal_edges.dimensions();
  let mut edge_detector = ImageBuffer::new(width, height);

  // Merges the two edge highlighters together into a single image.
  for (x, y, pixel) in edge_detector.enumerate_pixels_mut() {
    // Finds the vertical and horizontal edge values at given pixel.
    let vertical_edge = vertical_edges.get_pixel(x, y).data[0];
    let horizontal_edge = horizontal_edges.get_pixel(x, y).data[0];

    // If larger from both values equals max value (255 for white) or the lower
    // equals the min value (0 for black), this pixel has been recognized as
    // clear edge and will be coloured (therefore we do Luma([0]) for black).
    // Otherwise the pixel value is white as the edge in this pixel was not
    // that prevalent. We have to check for both max and min values (0 and 255)
    // because the 3x3 kernels work in one direction. Should we only check for
    // black, we would end up with edges where the darker colour was on
    // top or right to the brighter one.
    let max = vertical_edge.max(horizontal_edge);
    let min = vertical_edge.min(horizontal_edge);

    *pixel = if max == 255 || min == 0 {
      Luma([0])
    } else {
      Luma([255])
    };
  }

  edge_detector
}

/// Removes pixels that are too dark or bright so that the edge detection works
/// better. This is a hacky solution that works mostly for bright images.
fn smooth_out_polarized_pixels(image: &DynamicImage) -> DynamicImage {
  // Copies the image with all colours converted to Luma.
  let mut image_gray: GrayImage = image.grayscale().to_luma();

  for pixel in image_gray.pixels_mut() {
    if pixel.data[0] < DARKEST_GREYSCALE_VALUE {
      *pixel = Luma([DARKEST_GREYSCALE_VALUE]);
    } else if pixel.data[0] > BRIGHTEST_GREYSCALE_VALUE {
      *pixel = Luma([BRIGHTEST_GREYSCALE_VALUE]);
    }
  }

  ImageLuma8(image_gray)
}
