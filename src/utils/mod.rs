use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn file_to_vec(path: &str) -> Vec<Result<String, io::Error>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().collect()
}
