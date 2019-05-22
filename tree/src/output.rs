#[derive(Debug)]
pub struct Output {
  files: usize,
  directories: usize,
  starting_directories: usize,
}

impl Output {
  pub fn new(starting_directories: usize) -> Output {
    Output {
      files: 0,
      directories: 0,
      starting_directories,
    }
  }

  pub fn increment_files(&mut self) {
    self.files += 1;
  }

  pub fn increment_directories(&mut self) {
    self.directories += 1;
  }

  pub fn print_count(&self) {
    println!(
      "{} directories, {} files",
      self.directories - self.starting_directories,
      self.files
    );
  }
}
