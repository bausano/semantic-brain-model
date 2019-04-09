extern crate hound;
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


pub fn get_objects_from_image(file: File) -> Vec<VisualObject> {
  let (image_color, image_gray) = open_image(file.clone());

  vec!(
    VisualObject::new(15, 15),
  )
}

fn open_image(file: File) -> (
  DynamicImage,
  ImageBuffer<Luma<u8>, Vec<u8>>
) {
  // Loads an image from given file path. The type will be automatically handled
  // by the image crate.
  let image = image::open(file.get_full_path())
    .expect("Could not open file: ".to_owned() + file.get_name());
}
