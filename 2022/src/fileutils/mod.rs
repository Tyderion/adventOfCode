
use std::{
    fs::File,
    io::{BufRead, BufReader, Result, Lines},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename).expect("no such file");
    Ok(BufReader::new(file).lines())
}
