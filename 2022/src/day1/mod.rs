use std::cmp::Reverse;

use crate::fileutils;

pub fn run() {
    let filename = "./src/day1/input.txt";
    let lines = fileutils::lines_from_file(filename);
    let mut grouped_calories = lines.unwrap().into_iter().map(|l| l.unwrap()).fold(
        vec![0],
        |mut result, current| {
            match current {
                _ if current.is_empty() => result.push(0),
                weight => *result.last_mut().unwrap() += weight.parse::<i32>().unwrap(),
            }
            result
        },
    );
    grouped_calories.sort_by_key(|n| Reverse(*n));

    println!("Max: {:?}", grouped_calories.iter().max().unwrap());

    let top3 = grouped_calories.iter().take(3).sum::<i32>();
    println!("Top 3 carry: {:?}", top3);
}
