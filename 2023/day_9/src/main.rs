pub fn main() {
    let filename = "day_9/src/input.txt";
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

fn part1(_lines: &Vec<impl AsRef<str>>) -> u64 {
    0
}

fn part2(_lines: &Vec<impl AsRef<str>>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 3] = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 114);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, todo!());
    }
}
