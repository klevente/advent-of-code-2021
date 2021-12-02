use std::io::Read;
use std::{fs::File, path::Path};

pub fn read_file_to_string(path: impl AsRef<Path>) -> String {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn read_file_lines(path: impl AsRef<Path>) -> Vec<String> {
    let contents = read_file_to_string(path);
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn read_file_lines_as<T>(path: impl AsRef<Path>, f: fn(&str) -> T) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().map(f).collect()
}
