mod item_vector;

use dirs;
use item_vector::ItemVector;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut item_vec: ItemVector = ItemVector::new();
    let home_dir_path_buf = dirs::home_dir().expect("Couldn't get home directory");
    let home_dir_path = home_dir_path_buf.as_path();

    get_items_from_default_paths(&mut item_vec, home_dir_path);
    get_items_from_user_defined_paths(&mut item_vec, home_dir_path);

    item_vec.sort();
    item_vec.print();
}

fn get_items_from_default_paths(item_vec: &mut ItemVector, home_dir: &Path) {
    let paths = include_str!("default_paths.txt").lines();

    for path in paths {
        let assembled_path = home_dir.join(Path::new(path));

        if assembled_path.exists() {
            item_vec.add_items_in_dir(&assembled_path);
        }
    }
}

fn get_items_from_user_defined_paths(item_vec: &mut ItemVector, home_dir: &Path) {
    // Code for retrieving extra user defined paths
    let path = home_dir.join(Path::new(
        "Library/Application Support/The Lyons' Den Labs/shlogger_extra_paths.txt",
    ));

    let paths_file = File::open(path).expect("Couldn't open file or it does not exist.");

    let paths_file_buf = BufReader::new(paths_file);

    for (line_num, line_result) in paths_file_buf.lines().enumerate() {
        match line_result {
            Ok(line) => {
                let path = Path::new(&line);

                if path.exists() {
                    item_vec.add_items_in_dir(&path);
                }
            }
            Err(_) => println!(
                "Could not convert path to string on file line: {}",
                line_num
            ),
        }
    }
}
