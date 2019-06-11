extern crate hound;

use std::fs;
use std::collections::HashMap;

pub fn sound() {
  let mut reader = hound::WavReader::open("data/debug-1/audio/note-e.wav").unwrap();

  let samples = reader.samples::<i16>();
  let mut sponge: HashMap<i16, u32> = HashMap::new();

  for sample in samples {
    let key = sample.unwrap();
    let value = sponge.get(&key).unwrap_or(&0) + 1;
    sponge.insert(key, value);
  }

  let mut csv: String = String::new();

  for (key, value) in sponge.iter() {
    csv.push_str(&format!("{},{}\n", key, value));
  }

  fs::write("test.csv", csv).expect("Unable to write file");
}

/*
let mut csv = String::new();

for sample in samples {
  csv.push_str(&("\n".to_owned() + &sample.unwrap().to_string()));
}

fs::write("test.csv", csv).expect("Unable to write file");
*/

