extern crate hound;

pub fn sound() {
  let mut reader = hound::WavReader::open("data/debug-1/audio/output.wav").unwrap();

  // let samples = reader.samples::<i16>();

  println!(
    "Sound should be {} sec long.",
    (reader.len() as f64 / 44100_f64) as u32
  );
}
