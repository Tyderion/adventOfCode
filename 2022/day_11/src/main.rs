use std::{
    collections::{BTreeMap, HashMap},
    iter::Map,
};

use monkey::MonkeyId;

use crate::monkey::Monkey;

mod monkey;
mod operation;

fn main() {
    let filename = "day_10/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Monkey Business Level: {}", part1_result);

    let part2_result = part2(lines.clone());
    println!("Part2: ??{}", part2_result);
}

fn parse_monkeys<T: AsRef<str>>(lines: Vec<T>) -> BTreeMap<MonkeyId, Monkey> {
    lines
        .chunks(7)
        .map(|chunk| {
            Monkey::from(
                chunk
                    .iter()
                    .map(|s| s.as_ref())
                    .filter(|s| !s.is_empty())
                    .collect(),
            )
        })
        .map(|monkey| (monkey.id, monkey))
        .collect()
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> i32 {
    let monkeys = parse_monkeys(lines);
    0
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        let filename = "src/example.txt";
        match fileutils::safe_lines_from_file(filename) {
            Some(lines) => lines,
            // When debugging we start in root, else in day_11
            _ => fileutils::lines_from_file("day_11/".to_string() + filename),
        }
    }

    #[test]
    fn example_case_part1() {
        let result = part1(example_input());
        assert_eq!(result, 10605);
    }

    #[test]
    fn example_case_part2() {
        // Cannot be tested as it draws letters on the command line
        let result = part2(example_input());
        assert_eq!(result, 2);
    }
}
