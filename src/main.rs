mod file_finder;

use directories::{BaseDirs, UserDirs};
use file_finder::FileFinder;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut file_finder: FileFinder = FileFinder::new();

    get_files_from_default_paths(&mut file_finder);
    get_files_from_user_defined_paths(&mut file_finder);

    file_finder.sort_by_file_size();
    println!("{}", file_finder);
}

fn get_files_from_default_paths(file_finder: &mut FileFinder) {
    let user_dirs: UserDirs = UserDirs::new().expect("Couldn't create a UserDirs object.");

    file_finder.add_files_in_dir(
        user_dirs
            .audio_dir()
            .expect("Couldn't obtain audio folder."),
    );

    file_finder.add_files_in_dir(
        user_dirs
            .desktop_dir()
            .expect("Couldn't obtain desktop folder."),
    );

    file_finder.add_files_in_dir(
        user_dirs
            .document_dir()
            .expect("Couldn't obtain documents folder."),
    );

    file_finder.add_files_in_dir(
        user_dirs
            .download_dir()
            .expect("Couldn't obtain download folder."),
    );

    file_finder.add_files_in_dir(
        user_dirs
            .picture_dir()
            .expect("Couldn't obtain picture folder."),
    );

    file_finder.add_files_in_dir(
        user_dirs
            .public_dir()
            .expect("Couldn't obtain public folder."),
    );

    file_finder.add_files_in_dir(
        user_dirs
            .video_dir()
            .expect("Couldn't obtain video folder."),
    );
}

fn get_files_from_user_defined_paths(file_finder: &mut FileFinder) {
    let base_dirs: BaseDirs = BaseDirs::new().expect("Couldn't create a BaseDirs object");
    let lyons_den_dir = base_dirs.data_dir().join("The Lyons' Den Labs");

    create_dir_all(&lyons_den_dir)
        .expect("Could not create the path for user-defined paths");

    let shlogger_exta_paths_path = lyons_den_dir.join(Path::new("shlogger_extra_paths.txt"));

    let shlogger_exta_paths_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(shlogger_exta_paths_path)
        .expect("Couldn't open user-defined paths file");

    let paths_file_buf = BufReader::new(shlogger_exta_paths_file);

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
