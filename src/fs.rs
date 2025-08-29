use std::{fs, io};
use std::fs::Metadata;
use std::path::Path;

pub trait Fs: Send + Sync {
    fn metadata(&self, path: &Path) -> io::Result<Metadata>;
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
}

pub struct StdFs;

impl StdFs {
    pub fn new() -> Self { Self }
}

impl Fs for StdFs {
    fn metadata(&self, path: &Path) -> io::Result<Metadata> {
        fs::metadata(path)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        fs::rename(from, to)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
}
