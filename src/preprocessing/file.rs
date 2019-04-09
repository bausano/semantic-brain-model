
/// Tuple representing a file where first string is path, second string is file
/// name and third string is the extension.
pub struct File (String, String, String);

impl File {

  /// Joins the file strings and returns full path to the file.
  pub fn get_full_path(&self) -> String {
    self.0.clone() + "/".to_owned() + self.1.clone() + "." + self.2.clone()
  }

  /// Returns the file name.
  pub fn get_name(&self) -> String {
    self.1.clone()
  }

  /// Returns the extension of the file.
  pub fn get_extension($self) -> String {
    self.2.clone()
  }

}
