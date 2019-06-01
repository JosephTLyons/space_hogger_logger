use dirs;
use pretty_bytes::converter::convert;
use std::fs::{read_dir, File, ReadDir};
use std::io::{BufRead, BufReader};
use std::path::Path;

struct FileObj {
    path: String,
    size_in_bytes: u64,
}

fn main() {
    let home_dir = dirs::home_dir()
        .expect("Couldn't get home directory")
        .to_str()
        .expect("Couldn't parse home directory path to &str")
        .to_string();

    let paths_file = File::open(
        home_dir + "/Library/Application Support/The Lyons' Den Labs/space_hogger_logger_paths.txt",
    )
    .expect("Couldn't open file or it does not exist.");

    let paths_file_buf = BufReader::new(paths_file);

    let mut item_vec: Vec<FileObj> = Vec::new();

    for (line_num, line_result) in paths_file_buf.lines().enumerate() {
        match line_result {
            Ok(line) => {
                if Path::new(&line).exists() {
                    recursively_get_items_in_dir(Path::new(&line), &mut item_vec);
                }
            }
            Err(_) => println!(
                "Could not convert path to string on file line: {}",
                line_num
            ),
        }
    }

    // item_vec.sort_by(|a, b| a.1.cmp(&b.1));
    // item_vec.sort_by(|(_, u1), (_, u2)| u1.cmp(u2));
    item_vec.sort_unstable_by_key(|a| a.size_in_bytes);
    print_item_vec(&item_vec);
}

fn recursively_get_items_in_dir(path: &Path, mut item_vec: &mut Vec<FileObj>) {
    let dir_file_iter: ReadDir = read_dir(path).expect("Couldn't obtain item iter.");

    for item in dir_file_iter {
        let item = item.expect("Wasn't able to obtain item");
        let item_path_buf = item.path();
        let item_path = item_path_buf.as_path();
        let item_metadata = item.metadata().expect("Couldn't get item's metadata");

        if item_metadata.is_dir() {
            recursively_get_items_in_dir(item_path, &mut item_vec);
        } else {
            item_vec.push(FileObj {
                path: item_path.to_str().expect("Could not create &str file path").to_string(),
                size_in_bytes: item_metadata.len(),
            });
        }
    }
}

fn print_item_vec(item_vec: &[FileObj]) {
    for item in item_vec {
        println!(
            "{}{}",
            format!("{:>12}", convert(item.size_in_bytes as f64) + ": "),
            item.path
        );
    }
}
