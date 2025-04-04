use std::fs::File;
use std::io;
use std::fs;
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

// #[cfg(test)]
// pub struct MockFileSystem {
//     default_content: String,
//     pub create_dir_all_called: RefCell<Vec<PathBuf>>,
//     pub create_file_called: RefCell<Vec<PathBuf>>,
//     pub exists: bool,
// }

// #[cfg(test)]
// impl MockFileSystem {
//     fn new_with_content(content: String) -> Self {
//         Self {
//             default_content: content,
//             create_dir_all_called: RefCell::new(Vec::new()),
//             create_file_called: RefCell::new(Vec::new()),
//             exists: true,
//         }
//     }

//     pub fn new() -> Self {
//         Self::new_with_content(String::new())
//     }

//     pub fn with_content(content: &str) -> Self {
//         Self::new_with_content(content.to_string())
//     }
// }

// #[cfg(test)]
// impl FileSystem for MockFileSystem {
//     fn create_dir_all(&self, path: &Path) -> io::Result<()> {
//         if path.to_str().unwrap() == "error" {
//             return Err(io::Error::new(io::ErrorKind::Other, "Mock error"));
//         }
//         self.create_dir_all_called.borrow_mut().push(path.to_path_buf());
//         Ok(())
//     }

//     fn create_file(&self, path: &Path) -> io::Result<()> {
//         if path.to_str().unwrap() == "error" {
//             return Err(io::Error::new(io::ErrorKind::Other, "Mock error"));
//         }
//         self.create_file_called.borrow_mut().push(path.to_path_buf());
//         Ok(())
//     }

//     fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
//         if path.to_str().unwrap() == "error" {
//             return Err(io::Error::new(io::ErrorKind::Other, "Mock error"));
//         }
//         if content != self.default_content {
//             return Err(io::Error::new(io::ErrorKind::Other, "Mock error"));
//         }
//         Ok(())
//     }
//     fn read_to_string(&self, path: &Path) -> io::Result<String> {
//         if path.to_str().unwrap() == "error" {
//             return Err(io::Error::new(io::ErrorKind::Other, "Mock error"));
//         }
//         Ok(self.default_content.clone())
//     }

//     fn exists(&self, _path: &Path) -> bool {
//         // if path.to_str().unwrap() == "error" {
//         //     return false;
//         // }
//         self.exists
//     }
// }
