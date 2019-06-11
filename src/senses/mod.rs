//! Module for preprocessing the data before feeding it into the model.
//! This module will eventually be extracted out and the preprocessing will
//! happen on a dedicated machine.

mod file;
mod visual;
mod auditory;

use self::file::File;
//use self::auditory::sound;
use self::visual::identify_objects;

pub fn start_data_channel() {
  identify_objects(
    File::new(
      String::from("data/debug-1/video"),
      String::from("output_0407"),
      String::from("png"),
    )
  );
}
