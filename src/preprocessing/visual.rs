extern crate image;

use preprocessing::file::File;
use preprocessing::visual_object::VisualObject;

use self::image::{
  Luma,
  imageops,
  ImageBuffer,
  DynamicImage,
};

/// Replaces all pixels that are darker than this threshold by this threshold
/// giving the edge detection extra space to highlight edges.
const DARKEST_GREYSCALE_VALUE: u8 = 5;

/// How strongly should edges be favored in edge detection algorithm. The larger
/// the value, the more dense the resulting image becomes.
const EDGE_COEF: f32 = 10_f32;

/// Cell is a square that represents size*size pixels of the original image with
/// a single number. It is used to track density of edges. The larger the cell
/// size the lower the resolution of the heat map. The lower the cell size the
/// less abstract the heat map becomes. It has to be a number that is divides
/// both image width and image hight without a rest.
const CELL_SIZE: u32 = 20;

type GrayImageRaw = Vec<Vec<u32>>;
type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;

pub fn get_objects_from_image(file: File) -> Vec<VisualObject> {
  // Opens the image with original colours and in chromo.
  let (image_color, image_gray) = open_image(&file);

  // Highlights edges.
  let edge_detector = find_edges(&image_gray);
  // TODO: Remove
  edge_detector.save("output/test/".to_owned() + &file.get_name() + "_edge" + &file.get_extension()).unwrap();

  let objects_detector = highlight_objects(&edge_detector);

  vec!(
    VisualObject::new(15, 15),
  )
}

/// Opens given file and converts the image into greyscale, returning both the
/// original image and the new one. Also makes sure greyscale does not contain
/// too dark pixels.
fn open_image(file: &File) -> (
  DynamicImage,
  ImageBuffer<Luma<u8>, Vec<u8>>
) {
  // Loads an image from given file path. The type will be automatically handled
  // by the image crate.
  let image = image::open(file.get_full_path())
    .expect("Could not open image.");

  // Copies the image with all colours converted to Luma.
  let mut image_gray = imageops::grayscale(&image);

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
fn find_edges(image: &GrayImage) -> GrayImage {
  let (width, height) = image.dimensions();
  let mut edge_detector = ImageBuffer::new(width, height);

  // Highlights horizontal edges.
  let horizontal_edges = imageops::filter3x3(&image, &[
    EDGE_COEF, EDGE_COEF, EDGE_COEF,
    1_f32, 1_f32, 1_f32,
    -EDGE_COEF, -EDGE_COEF, -EDGE_COEF,
  ]);

  // Highlights vertical edges.
  let vertical_edges = imageops::filter3x3(&image, &[
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

/// Highlights zones with crucial objects in the image. Does so by creating a
/// heat map (the more dense the pixels, the larger heat) and the runs the heat
/// map though a cellular automaton with set rules that stabilize all cells into
/// two states: alive (255) or dead (0).
fn highlight_objects(image: &GrayImage) -> GrayImageRaw {
  let (width, height) = image.dimensions();
  let mut heat_detector: GrayImage = ImageBuffer::new(width, height);

  // From the bricked heat map creates more detailed one where each cell is half
  // of the size of those in the bricked heat map. This multi-dimensional vector
  // represents density of edges in the original image.
  // Also returns maximum heat observed in the map and an average heat. This is
  // used for calculating the rules of the cellular automaton.
  let (heat_map, heat_max, heat_mean) = get_crisp_heat_map(
    // Sketches cells over the image that overlay and calculates their heat.
    get_bricked_heat_map(&image),
    width,
    height,
  );

  // Stabilizes each cell into one of two states.
  let highlight = cellular_automaton(heat_map, heat_max, heat_mean);

  // TODO: Remove
  let unit: f64 = 255_f64 / heat_max as f64;
  for (x, y, pixel) in heat_detector.enumerate_pixels_mut() {
    let col = (x / (CELL_SIZE / 2)) as usize;
    let row = (y / (CELL_SIZE / 2)) as usize;
    let heat = highlight[row][col];
    let mut heat: u8 = (unit * heat as f64) as u8;
    *pixel = Luma([heat]);
  }
  imageops::colorops::invert(&mut heat_detector);
  heat_detector.save("output/test/heat.png").unwrap();

  highlight
}

/// Calculates the heat map of overlaying cells. Most pixels therefore belong
/// to 4 cells. Pixels on the edges of the image belong to 2 cells and pixels
/// in the corners belong to one cell.
///
/// In the following diagram, there are 4 cells where each cell is of the same
/// size (e.g. cell 0x0 contains CELL_SIZE*CELL_SIZE pixels).
/// a: row 0, col 0
/// b: row 0, col 1
/// c: row 1, col 0
/// d: row 1, col 1
///
///   ____0___________1_____
/// 0 |   a    ab     b...
///   |   ac   abcd   bd...
/// 1 |   c... cd...  d...
///
fn get_bricked_heat_map(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> GrayImageRaw {
  let (width, height) = image.dimensions();

  // We want the cells to overlay one another by half of their size. Therefore
  // we can fit one full stack of cells plus one on top of it, but the second
  // one starts with padding of CELL_SIZE / 2, therefore the overlay will fit
  // one cell less.
  let rows = (2 * height / CELL_SIZE) - 1;
  let columns = (2 * width / CELL_SIZE) - 1;

  let mut heat_map: GrayImageRaw = Vec::new();

  for offset_y in 0..rows {
    let mut row: Vec<u32> = Vec::new();

    for offset_x in 0..columns {
      let mut heat: u32 = 0;

      // Counts number of black pixels (in the image the pixels are black and
      // white only) in given cell.
      for cell_y in 0..CELL_SIZE {
        for cell_x in 0..CELL_SIZE {
          // Gets the value of the pixel on position that is padded by the
          // offset plus the current cell index.
          let pixel = image.get_pixel(
            (offset_x * CELL_SIZE / 2) + cell_x,
            (offset_y * CELL_SIZE / 2) + cell_y,
          );

          if pixel.data[0] == 0 {
            heat += 1;
          }
        }
      }

      row.push(heat);
    }

    heat_map.push(row);
  }

  heat_map
}

/// Transforms the bricked heat map where the cells are of CELL_SIZE to a more
/// granular one where cells are CELL_SIZE / 2. This gives us better detail
/// while preserving relationships between all parts of the image rather than
/// cropping out a block and calculating the heat separately.
fn get_crisp_heat_map(
  bricked_heat_map: GrayImageRaw,
  width: u32,
  height: u32,
) -> (GrayImageRaw, u32, u32) {
  let mut heat_max: u32 = 1;
  let mut heat_total: u32 = 0;
  let mut heat_counter: u32 = 1;
  let mut heat_map: GrayImageRaw = Vec::new();

  for offset_y in 0..(2 * height / CELL_SIZE) {
    let mut row: Vec<u32> = vec!();

    for offset_x in 0..(2 * width / CELL_SIZE) {
      // Sums the heat of all cells that participate to given offset and divides
      // it by 4. This will result in very low heat near the edges of the image.
      let heat: u32 = {
        let x: isize = offset_x as isize;
        let y: isize = offset_y as isize;

        (pixel_value(&bricked_heat_map, x, y, 0) +
        pixel_value(&bricked_heat_map, x, y - 1, 0) +
        pixel_value(&bricked_heat_map, x - 1, y, 0) +
        pixel_value(&bricked_heat_map, x - 1, y - 1, 0)) as u32
      } / 4;

      // Updates maximum observed heat.
      heat_max = heat_max.max(heat);

      // Adds info to heat average calculations.
      if heat > 0 {
        heat_total += heat;
        heat_counter += 1;
      }

      row.push(heat);
    }

    heat_map.push(row);
  }

  (heat_map, heat_max, heat_total / heat_counter)
}

/// Runs the automaton until all cells are stabilized (positively dead or alive)
/// which corresponds to their heat values of 0 to max. The rules are based on
/// their surrounding heat within the Moore neighbourhood. The resulting vector
/// highlights important objects in the image.
fn cellular_automaton(map: GrayImageRaw, max: u32, mean: u32) -> GrayImageRaw {
  // This loop break once there has been no change in the previous cycle, which
  // means the map is stabilized.
  loop {
    // Flag for breaking the cycle.
    let mut stabilized = true;
    // New status of the map after this cycle.
    let mut step_map: GrayImageRaw = Vec::new();

    for (y, map_row) in map.iter().enumerate() {
      let mut step_map_row: Vec<u32> = Vec::new();

      for (x, heat) in map_row.iter().enumerate() {
        // If the cell is stabilized (either fully dead or alive), skip it.
        if *heat == max || *heat == 0 {
          step_map_row.push(*heat);
          continue;
        }

        stabilized = false;

        // Find the average heat in Moore neighbourhood.
        let surrounding_heat: u32 = get_neighborhood_heat(&map, x, y);

        // Rule #1:
        // If the surrounding heat is less than the smaller value of out average
        // map heat or cell heat, cell dies.
        if surrounding_heat < mean.min(*heat) {
          step_map_row.push(0);
          continue;
        }

        // Rule #2:
        // If the surrounding heat is lower than an average, the cell decreases
        // its heat by that difference.
        if surrounding_heat < mean {
          step_map_row.push(0.max(
            *heat as i32 - mean as i32 + surrounding_heat as i32
          ) as u32);
          continue;
        }

        // Rule #3:
        // If the surrounding heat is larger or equal to the average heat, the
        // cell increases its heat by that difference.
        if surrounding_heat >= mean {
          step_map_row.push(max.min(*heat + surrounding_heat - mean));
          continue;
        }

        // Otherwise the cell idles and waits for the environment.
        println!("This should not happen tho.");
        stabilized = true;
        step_map_row.push(*heat);
      }

      step_map.push(step_map_row);
    }

    if stabilized {
      break;
    }

    // Updates the map to its new evolvement.
    map = step_map;
  }

  map
}

/// Helper function for accessing values at given address in vector. If the
/// address is out of bounds, it delivers the default value instead.
fn pixel_value(vec: &GrayImageRaw, x: isize, y: isize, default: u32) -> u32 {
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

/// Calculates the mean heat in Moore neighbourhood of a cell at given location.
fn get_neighborhood_heat(map: &GrayImageRaw, x: usize, y: usize) -> u32 {
  let x: isize = x as isize;
  let y: isize = y as isize;

  (pixel_value(map, x - 1, y - 1, 0) +
  pixel_value(map, x, y - 1, 0) +
  pixel_value(map, x + 1, y - 1, 0) +
  pixel_value(map, x - 1, y, 0) +
  pixel_value(map, x + 1, y, 0) +
  pixel_value(map, x - 1, y + 1, 0) +
  pixel_value(map, x, y + 1, 0) +
  pixel_value(map, x + 1, y + 1, 0)) / 8
}
