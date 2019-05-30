use std::fs::{read_dir, ReadDir};
use std::path::Path;

fn main() {
    let mut item_vec: Vec<(String, u64)> = Vec::new();
    let starting_loc = Path::new("/Users/josephlyons/Books");
    recursively_get_items_in_dir(starting_loc, &mut item_vec);
    item_vec.sort_by(|a, b| b.1.cmp(&a.1));
    print_item_vec(&item_vec);
}

fn recursively_get_items_in_dir(path: &Path, mut item_vec: &mut Vec<(String, u64)>) {
    let dir_file_iter: ReadDir = read_dir(path).expect("Couldn't obtain iter.");

    for item in dir_file_iter {
        let item = item.expect("Wasn't able to obtain item");
        let item_path_buf = item.path();
        let item_path = item_path_buf.as_path();
        let item_metadata = item.metadata().expect("Couldn't get item's metadata");

        if item_metadata.is_dir() {
            recursively_get_items_in_dir(item_path, &mut item_vec);
        }

        else {
            let item_path_str = item_path.to_str().expect("Could not create &str file path");
            item_vec.push((String::from(item_path_str), item_metadata.len()));
        }
    }
}

fn print_item_vec(item_vec: &[(String, u64)]) {
    for item in item_vec {
        println!("{}: {} bytes", item.0, item.1);
    }
}
