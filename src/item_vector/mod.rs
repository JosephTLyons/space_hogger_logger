mod file_obj;

use file_obj::FileObj;
use pretty_bytes::converter::convert;
use std::fs::{read_dir, ReadDir};
use std::path::Path;

pub struct ItemVector {
    item_vec: Vec<FileObj>,
}

impl ItemVector {
    pub fn new() -> Self {
        ItemVector {
            item_vec: Vec::new(),
        }
    }

    pub fn add_items_in_dir(&mut self, file_path: &Path) {
        self.recursively_get_items_in_dir(file_path);
    }

    fn recursively_get_items_in_dir(&mut self, path: &Path) {
        let dir_file_iter: ReadDir = read_dir(path).expect("Couldn't obtain item iter.");

        for item in dir_file_iter {
            let item = item.expect("Wasn't able to obtain item");
            let item_path_buf = item.path();
            let item_path = item_path_buf.as_path();
            let item_metadata = item.metadata().expect("Couldn't get item's metadata");

            if item_metadata.is_dir() {
                self.recursively_get_items_in_dir(item_path);
            } else {
                self.item_vec.push(FileObj {
                    path: item_path
                        .to_str()
                        .expect("Could not create &str file path")
                        .to_string(),
                    size_in_bytes: item_metadata.len(),
                });
            }
        }
    }

    pub fn sort(&mut self) {
        // item_vec.sort_by(|a, b| a.1.cmp(&b.1));
        // item_vec.sort_by(|(_, u1), (_, u2)| u1.cmp(u2));
        self.item_vec.sort_unstable_by_key(|a| a.size_in_bytes);
    }

    pub fn print(&self) {
        for item in &self.item_vec {
            println!(
                "{}{}",
                format!("{:>12}", convert(item.size_in_bytes as f64) + ": "),
                item.path
            );
        }
    }
}
