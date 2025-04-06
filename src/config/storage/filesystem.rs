use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn create_file(&self, path: &Path) -> io::Result<()>;
    fn write_file(&self, path: &Path, content: &str) -> io::Result<()>;
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    fn exists(&self, path: &Path) -> bool;
}

pub struct OsFileSystem;

impl OsFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for OsFileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn create_file(&self, path: &Path) -> io::Result<()> {
        File::create(path).map(|_| ())
    }

    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        fs::write(path, content)
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}
