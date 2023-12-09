use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    let filename = "day_8/src/input.txt";
    let input = fileutils::safe_lines_from_file(filename);
    let part1_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part1(lines),
    };
    let part2_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part2(lines),
    };
    println!("Sum of games: {}", part1_result);
    println!("Sum of part 2: {}", part2_result);
}

fn parse(lines: &Vec<impl AsRef<str>>) -> (Vec<char>, HashMap<String, Vec<String>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"( =)|[(,)]").unwrap();
    }
    let (instruction_list, rest) = lines.split_first().unwrap();
    let map = rest.iter().filter(|l| !l.as_ref().is_empty()).fold(
        HashMap::new() as HashMap<String, Vec<String>>,
        |mut acc, ele| {
            let replaced = RE.replace_all(ele.as_ref(), "");
            let [key, l, r] = replaced
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            acc.insert(key.to_string(), vec![l.to_string(), r.to_string()]);
            acc
        },
    );
    let instructions = instruction_list.as_ref().chars().collect::<Vec<_>>();

    (instructions, map)
}

fn access_index(instructions: &Vec<char>, index: usize) -> usize {
    if instructions[index] == 'L' {
        0
    } else {
        1
    }
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u64 {
    let (instructions, map) = parse(lines);
    let mut index = 0;
    let mut steps = 0;
    let mut key = "AAA";
    loop {
        key = &map.get(key).unwrap()[access_index(&instructions, index)];
        index = (index + 1) % instructions.len();
        steps += 1;
        if key == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(_lines: &Vec<impl AsRef<str>>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 9] = [
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    const EXAMPLE_INPUT2: [&str; 5] = [
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    const EXAMPLE_INPUT1_PART2: [&str; 10] = [
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    #[test]
    fn example_case1_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 2);
    }

    #[test]
    fn example_case2_part1() {
        let result = part1(&EXAMPLE_INPUT2.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 6);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(
            &EXAMPLE_INPUT1_PART2
                .iter()
                .map(|x| String::from(*x))
                .collect(),
        );
        assert_eq!(result, 6);
    }
}
