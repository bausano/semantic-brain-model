extern crate image;

use visual_object::VisualObject;
use file::File;
use self::image::imageops::colorops::invert;
use self::image::imageops::{grayscale, filter3x3};
use self::image::{
  Rgb,
  Luma,
  ImageBuffer,
  DynamicImage,
};

/// Replaces all pixels that are darker than this threshold by this threshold
/// giving the edge detection extra space to highlight edges.
const DARKEST_GREYSCALE_VALUE: u8 = 5;

/// How strongly should edges be favored in edge detection algorithm. The larger
/// the value, the more dense the resulting image becomes.
const EDGE_COEF: f32 = 10_f32;

pub fn get_objects_from_image(file: File) -> Vec<VisualObject> {
  // Opens the image with original colours and in chromo.
  let (image_color, image_gray) = open_image(file.clone());

  // Highlights edges.
  let edge_detector = find_edges(&image_gray);
  // TODO: Remove
  edge_detector.save("output/test/".to_owned() + file.get_name() + "_edge" + file.get_extension()).unwrap();

  vec!(
    VisualObject::new(15, 15),
  )
}

/// Opens given file and converts the image into greyscale, returning both the
/// original image and the new one. Also makes sure greyscale does not contain
/// too dark pixels.
fn open_image(file: File) -> (
  DynamicImage,
  ImageBuffer<Luma<u8>, Vec<u8>>
) {
  // Loads an image from given file path. The type will be automatically handled
  // by the image crate.
  let image = image::open(file.get_full_path())
    .expect("Could not open file: ".to_owned() + file.get_name());

  // Copies the image with all colours converted to Luma.
  let mut image_gray = grayscale(&image);

  // Removes pixels that are too dark so that the edge detection works better.
  // This is a hacky solution that works mostly for bright images.
  for pixel in image_gray.pixels_mut() {
    if pixel.data[0] < DARKEST_GREYSCALE_VALUE {
      *pixel = Luma([DARKEST_GREYSCALE_VALUE]);
    }
  }

  (image, image_gray)
}

/// Finds edges in given grayscale picture by using two 3x3 matrixes. First one
/// detects horizontal edges, the second one vertical.
fn find_edges<T: ImageBuffer<Luma<u8>, Vec<u8>>>(image: &T) ->  T {
  let (width, height) = image.dimensions();
  let mut edge_detector: T = ImageBuffer::new(width, height);

  // Highlights horizontal edges.
  let horizontal_edges = filter3x3(&image, &[
    EDGE_COEF, EDGE_COEF, EDGE_COEF,
    1_f32, 1_f32, 1_f32,
    -EDGE_COEF, -EDGE_COEF, -EDGE_COEF,
  ]);

  // Highlights vertical edges.
  let vertical_edges = filter3x3(&image, &[
    EDGE_COEF, 1_f32, -EDGE_COEF,
    EDGE_COEF, 1_f32, -EDGE_COEF,
    EDGE_COEF, 1_f32, -EDGE_COEF,
  ]);

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
