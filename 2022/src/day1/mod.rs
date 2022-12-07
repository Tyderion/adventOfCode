use std::cmp::Reverse;

use crate::fileutils;

pub fn run() {
    let filename = "./src/day1/input.txt";
    let lines = fileutils::lines_from_file(filename);
    let mut reduced = lines.unwrap().into_iter().map(|l| l.unwrap()).fold(
        vec![0],
        |mut result: Vec<i32>, r: String| {
            if r.is_empty() {
                result.push(0);
            } else {
                let prev = result.last_mut().unwrap();
                *prev += r.parse::<i32>().unwrap();
            }
            result
        },
    );
    reduced.sort_by_key(|n| Reverse(*n));
    println!("Max: {:?}", reduced.iter().max().unwrap());

    let top3 = reduced.iter().take(3).sum::<i32>();
    println!("Top 3 carry: {:?}", top3);
}
