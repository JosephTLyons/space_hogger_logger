mod item_vector;

use dirs;
use item_vector::ItemVector;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn main() {
    let default_paths = include_str!("default_paths.txt").lines();

    let home_dir = dirs::home_dir()
        .expect("Couldn't get home directory")
        .to_str()
        .expect("Couldn't parse home directory path to &str")
        .to_string();

    let mut item_vec: ItemVector = ItemVector::new();

    for default_path in default_paths {
        let assembled_path = Path::new(&home_dir).join(Path::new(default_path));

        if assembled_path.exists() {
            item_vec.add_items_in_dir(&assembled_path);
        }
    }

    // Code for retrieving extra user defined paths
    let paths_file = File::open(
        home_dir + "/Library/Application Support/The Lyons' Den Labs/shlogger_extra_paths.txt",
    )
    .expect("Couldn't open file or it does not exist.");

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

    item_vec.sort();
    item_vec.print();
}
