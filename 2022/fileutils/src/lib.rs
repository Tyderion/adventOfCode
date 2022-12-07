
use std::{
    fs::{File},
    io::{BufRead, BufReader},
    path::Path, env,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(&filename).expect(format!("current dir: {}, no such path: {} ", env::current_dir().unwrap().display() , filename.as_ref().to_str().unwrap()).as_str());
    BufReader::new(file).lines().into_iter().map(|l| l.unwrap()).collect()
}
