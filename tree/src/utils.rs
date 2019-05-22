use std::path::PathBuf;

pub fn is_hidden(path: &PathBuf) -> bool {
  let mut is_valid_filename = true;
  let filename = match path.file_name() {
    Some(filename) => filename.to_string_lossy(),
    None => {
      is_valid_filename = false;
      path.to_string_lossy()
    }
  };

  if !is_valid_filename {
    return false;
  }

  match filename.chars().next() {
    Some('.') => true,
    _ => false,
  }
}
