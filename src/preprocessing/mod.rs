//! Module for preprocessing the data before feeding it into the model.
//! This module will eventually be extracted out and the preprocessing will
//! happen on a dedicated machine.

pub fn start_data_channel() {
  let unit: f64 = 255_f64 / heat_max as f64;

  for (x, y, pixel) in heat_detector.enumerate_pixels_mut() {
    let col = (x / (cell_size / 2)) as usize;
    let row = (y / (cell_size / 2)) as usize;
    let heat = final_heat_map[row][col];

    let mut heat: u8 = (unit * heat as f64) as u8;

    *pixel = Luma([heat]);
  }

  invert(&mut heat_detector);

  heat_detector.save("output/test/".to_owned() + file + "_heat.png").unwrap();

}
