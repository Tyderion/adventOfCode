fn main() {
    let filename = "day_9/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Part1 visible trees: {}", part1_result);

    let part2_result = part2(lines);
    println!("Part2 max viewable trees: {}", part2_result);
}

fn part1<T: AsRef<str>>(_lines: Vec<T>) -> u32 {
    0
}

fn part2<T: AsRef<str>>(_lines: Vec<T>) -> u32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE: [&str; 8] = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];

    #[test]
    fn example_case_part1() {
        let result = part1(EXAMPLE.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 13);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(EXAMPLE.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 0);
    }
}
