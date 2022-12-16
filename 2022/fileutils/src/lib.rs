use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(&filename).expect(
        format!(
            "current dir: {}, no such path: {} ",
            env::current_dir().unwrap().display(),
            filename.as_ref().to_str().unwrap()
        )
        .as_str(),
    );
    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .collect()
}

pub fn safe_lines_from_file(filename: impl AsRef<Path>) -> Option<Vec<String>> {
    match File::open(&filename) {
        Ok(file) => Some(
            BufReader::new(file)
                .lines()
                .into_iter()
                .map(|l| l.unwrap())
                .collect::<Vec<String>>(),
        ),
        _ => None,
    }
}
