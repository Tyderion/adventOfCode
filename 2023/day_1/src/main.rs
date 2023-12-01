pub fn main() {
    let filename = "day_1/src/input.txt";
    let input = fileutils::safe_lines_from_file(filename);
    let part1_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part1(lines),
    };
    let part2_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part2(lines),
    };
    println!("Sum of nubmers: {}", part1_result);
    println!("Sum of nubmers: {}", part2_result);
}

fn part1(_lines: &Vec<String>) -> u32 {
    return 0
}

fn part2(_lines: &Vec<String>) -> u32 {
    return 0;
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
}
