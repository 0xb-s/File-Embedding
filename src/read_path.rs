
use std::path::Path;
use std::path::PathBuf;

pub struct File {
    pub path: PathBuf,
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path }
    }
    pub fn read_directory<P: AsRef<Path>>(path: P) {
        std::fs::read_dir(path).unwrap();
    }
}
