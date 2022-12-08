#![feature(custom_test_frameworks)]

use std::collections::HashSet;

fn main() {
    let filename = "day_3/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_sum = part1(lines.clone());
    println!("Part1 Sum: {}", part1_sum);

    let part2_sum = part2(lines);
    println!("Part2 Sum: {}", part2_sum);
}

fn part1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            (
                left.chars().collect::<HashSet<char>>(),
                right.chars().collect::<HashSet<char>>(),
            )
        })
        .flat_map(|(left, right)| &left & &right)
        .map(|c| map_to_value(c).unwrap())
        .map(Into::<u32>::into)
        .sum::<u32>()
}

fn hs(a: &String) -> HashSet<char> {
    a.chars().collect::<HashSet<char>>()
}

fn part2_grouping(lines: &[String], collected: Vec<char>) -> Vec<char> {
    match lines {
        [a, b, c, rest @ ..] => part2_grouping(
            rest,
            (&(&hs(a) & &hs(b)) & &hs(c))
                .iter()
                .map(|c| c.to_owned())
                .chain(collected)
                .collect::<Vec<char>>(),
        ),
        _ => collected,
    }
}

fn part2(lines: Vec<String>) -> u32 {
    part2_grouping(lines.as_slice(), vec![])
        .iter()
        .map(|c| map_to_value(*c).unwrap())
        .map(Into::<u32>::into)
        .sum::<u32>()
}

fn map_to_value(c: char) -> Option<u8> {
    const START_VALUE_LOWERCASE: u8 = 1;
    const START_VALUE_UPPERCASE: u8 = 27;
    match c {
        c if c.is_alphabetic() && c.is_ascii_uppercase() => {
            Some(c as u8 - 'A' as u8 + START_VALUE_UPPERCASE)
        }
        c if c.is_alphabetic() && c.is_ascii_lowercase() => {
            Some(c as u8 - 'a' as u8 + START_VALUE_LOWERCASE)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case('a', Some(1); "lowercase a")]
    #[test_case('b', Some(2); "lowercase b")]
    #[test_case('z', Some(26); "lowercase z")]
    #[test_case('A', Some(27); "uppercase A")]
    #[test_case('Z', Some(52); "uppercase Z")]
    fn values_are_correct(c: char, expected: Option<u8>) {
        let result = map_to_value(c);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let result = part2(
            [
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]
            .map(String::from)
            .to_vec(),
        );

        assert_eq!(result, 70)
    }
}
