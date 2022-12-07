
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    BufReader::new(file).lines().into_iter().map(|l| l.unwrap()).collect()
}
