use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn main() {
    execute::load_and_execute("day_1/src/input.txt", part1, part2);
}

fn part1(lines: &Vec<String>) -> u32 {
    return lines
        .iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .filter(|c| match c {
                    '0'..='9' => true,
                    _ => false,
                })
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .map(|nums| nums.first().unwrap() * 10 + nums.last().unwrap())
        .sum::<u32>();
}

fn part2(lines: &Vec<String>) -> u32 {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    }

    let replacement = |caps: &Captures| match caps.get(0).unwrap().as_str() {
        "one" => "1e",
        "two" => "2o",
        "three" => "3e",
        "four" => "4r",
        "five" => "5e",
        "six" => "6x",
        "seven" => "7n",
        "eight" => "8t",
        "nine" => "9e",
        _ => "",
    };

    let replaced_numbers = lines
        .iter()
        .map(|l| RE.replace_all(l.as_ref(), &replacement).to_string())
        // Do it twice to handle overlapping cases
        .map(|l| RE.replace_all(l.as_ref(), &replacement).to_string())
        .collect::<Vec<_>>();
    return part1(replaced_numbers.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 4] = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    const EXAMPLE_INPUT2: [&str; 7] = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    const TEST: [&str; 1] = ["eightwo"];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 142);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT2.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 281);
    }

    #[test]
    fn custom_test() {
        let result = part2(&TEST.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 82);
    }
}
