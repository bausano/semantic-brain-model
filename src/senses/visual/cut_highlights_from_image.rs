use senses::visual::heat_map::CELL_SIZE;
use senses::visual::image::{DynamicImage};
use senses::visual::visual_object::VisualObject;

pub fn cut_highlights_from_image(
  mut highlights: Vec<VisualObject>,
  mut image: DynamicImage,
) -> Vec<DynamicImage> {
  highlights.iter_mut().filter_map(|highlight| {
    let (lower, higher) = highlight.size()?;
    let lower = lower + highlight.reference;
    let higher = higher + highlight.reference;

    Some(image.crop(
      (lower.x - 1) * CELL_SIZE / 2,
      (lower.y - 1) * CELL_SIZE / 2,
      (higher.x - lower.x + 2) * CELL_SIZE / 2,
      (higher.y - lower.y + 2) * CELL_SIZE / 2
    ))
  }).collect()
}
