mod file_finder;

use dirs;
use file_finder::FileFinder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut file_finder: FileFinder = FileFinder::new();
    let home_dir_path_buf = dirs::home_dir().expect("Couldn't obtain home directory.");
    let home_dir_path = home_dir_path_buf.as_path();

    get_files_from_default_paths(&mut file_finder, home_dir_path);
    get_files_from_user_defined_paths(&mut file_finder, home_dir_path);

    file_finder.sort_by_file_size();
    println!("{}", file_finder);
}

fn get_files_from_default_paths(file_finder: &mut FileFinder, home_dir: &Path) {
    let paths_in_home_folder: Vec<&str> = vec![
        "Applications",
        "Desktop",
        "Documents",
        "Downloads",
        "Movies",
        "Music",
        "Pictures",
    ];

    for path in paths_in_home_folder {
        let assembled_path = home_dir.join(Path::new(path));

        if assembled_path.exists() {
            file_finder.add_files_in_dir(&assembled_path);
        }
    }
}

fn get_files_from_user_defined_paths(file_finder: &mut FileFinder, home_dir: &Path) {
    // Code for retrieving extra user-defined paths
    let path = home_dir.join(Path::new(
        "Library/Application Support/The Lyons' Den Labs/shlogger_extra_paths.txt",
    ));

    let paths_file = File::open(path).expect("Error opening file.");
    let paths_file_buf = BufReader::new(paths_file);

    for (line_num, line_result) in paths_file_buf.lines().enumerate() {
        match line_result {
            Ok(line) => {
                let path = Path::new(&line);

                if path.exists() {
                    file_finder.add_files_in_dir(&path);
                }
            }
            Err(_) => println!(
                "Could not convert path to string on file line: {}",
                line_num
            ),
        }
    }
}
