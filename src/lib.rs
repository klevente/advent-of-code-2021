use std::convert::TryInto;
use std::io::Read;
use std::{fs::File, path::Path};

pub fn read_file_to_string(path: impl AsRef<Path>) -> String {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn read_file_lines(path: impl AsRef<Path>) -> Vec<String> {
    read_file_lines_as(path, str::to_string)
}

pub fn read_file_lines_as<T>(path: impl AsRef<Path>, f: fn(&str) -> T) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().map(f).collect()
}

pub fn read_file_lines_filter_as<T>(path: impl AsRef<Path>, f: fn(&str) -> Option<T>) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().filter_map(f).collect()
}

#[allow(dead_code)]
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
