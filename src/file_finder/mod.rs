mod file_obj;

use file_obj::FileObject;
use pretty_bytes::converter::convert as bytes_to_units;
use std::fmt;
use std::fs::{read_dir, ReadDir};
use std::path::Path;

pub struct FileFinder {
    file_vec: Vec<FileObject>,
}

impl FileFinder {
    pub fn new() -> Self {
        FileFinder {
            file_vec: Vec::new(),
        }
    }

    pub fn add_files_in_dir(&mut self, file_path: &Path) {
        self.recursively_get_files_in_dir(file_path);
    }

    fn recursively_get_files_in_dir(&mut self, path: &Path) {
        let dir_file_iter: ReadDir = read_dir(path).expect("Couldn't obtain file iter.");

        for file in dir_file_iter {
            let file = file.expect("Wasn't able to obtain file.");
            let file_path_buf = file.path();
            let file_path = file_path_buf.as_path();
            let file_metadata = file.metadata().expect("Couldn't get file's metadata.");

            if file_metadata.is_dir() {
                self.recursively_get_files_in_dir(file_path);
            } else {
                self.file_vec.push(FileObject {
                    path: file_path
                        .to_str()
                        .expect("Could not create &str file path.")
                        .to_string(),
                    size_in_bytes: file_metadata.len(),
                });
            }
        }
    }

    pub fn sort_by_file_size(&mut self) {
        self.file_vec.sort_by_key(|element| element.size_in_bytes);
    }
}

impl fmt::Display for FileFinder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for file in &self.file_vec {
            writeln!(
                f,
                "{:>12}: {}",
                bytes_to_units(file.size_in_bytes as f64),
                file.path
            )?;
        }

        Ok(())
    }
}
