use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub fn main() {
    execute::load_and_execute("day_8/src/input.txt", part1, part2);
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

fn compute_steps(
    instructions: &Vec<char>,
    map: &HashMap<String, Vec<String>>,
    start_key: String,
    has_ended: fn(key: String) -> bool,
) -> u64 {
    let mut index = 0;
    let mut steps = 0;
    let mut key = &start_key;
    loop {
        key = &map.get(key).unwrap()[if instructions[index] == 'L' { 0 } else { 1 }];
        index = (index + 1) % instructions.len();
        steps += 1;
        if has_ended(key.to_string()) {
            break;
        }
    }

    steps
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u64 {
    let (instructions, map) = parse(lines);
    compute_steps(&instructions, &map, "AAA".to_string(), |key| key == "ZZZ")
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u64 {
    let (instructions, map) = parse(lines);
    map.keys()
        .filter(|key| key.ends_with("A"))
        .map(|starting_key| {
            compute_steps(&instructions, &map, starting_key.to_string(), |key| {
                key.ends_with("Z")
            })
        })
        .reduce(|a, b| num_integer::lcm(a, b))
        .unwrap()
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
