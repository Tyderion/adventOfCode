use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let filename = "day_5/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Part1 Top of Stacks: {}", part1_result);

    let part2_result = part2(lines);
    println!("Part2: {}", part2_result);
}

fn part1(lines: Vec<String>) -> String {
    String::from("")
}

fn part2(_lines: Vec<String>) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 9] = [
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn verify_case() {
        let result = part1(EXAMPLE_INPUT.map(String::from).to_vec());
        assert_eq!(result, "CMZ");
    }
}
