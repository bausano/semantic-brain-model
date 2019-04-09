
/// Tuple representing a file where first string is path, second string is file
/// name and third string is the extension.
pub struct File (String, String, String);

impl File {

  /// Factory method for file.
  pub fn new(path: String, name: String, extension: String) -> File {
    File(path, name, extension)
  }

  /// Joins the file strings and returns full path to the file.
  pub fn full_path(&self) -> String {
    self.0.clone() + "/" + &self.1 + "." + &self.2
  }

}
