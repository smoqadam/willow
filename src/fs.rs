use log::info;
use std::fs::Metadata;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io};

pub trait Fs: Send + Sync {
    fn metadata(&self, path: &Path) -> io::Result<Metadata>;
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
}

pub struct StdFs;

impl Default for StdFs {
    fn default() -> Self {
        Self::new()
    }
}

impl StdFs {
    pub fn new() -> Self {
        Self
    }
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

pub struct DryRunFs {
    inner: Arc<dyn Fs>,
}

impl DryRunFs {
    pub fn new(inner: Arc<dyn Fs>) -> Self {
        Self { inner }
    }
}

impl Fs for DryRunFs {
    fn metadata(&self, path: &Path) -> io::Result<Metadata> {
        self.inner.metadata(path)
    }
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        info!("[dry-run] create_dir_all {path:?}");
        Ok(())
    }
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        info!("[dry-run] move {from:?} -> {to:?}");
        Ok(())
    }
    fn exists(&self, path: &Path) -> bool {
        self.inner.exists(path)
    }
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.inner.read_to_string(path)
    }
}
