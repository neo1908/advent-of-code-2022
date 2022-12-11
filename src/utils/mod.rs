use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn file_to_vec(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Dirty read func
    reader
        .lines()
        .map(|x| match x {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to read file {}", e);
                panic!();
            }
        })
        .collect()
}
