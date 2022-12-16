fn main() {
    let filename = "day_9/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Planck Rope Part 1: {} Positions", part1_result);

    let part2_result = part2(lines);
    println!("Part2 max viewable trees: {}", part2_result);
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> usize {
    todo!("Part 1")
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> usize {
    todo!("Part 2")
}


#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        let filename = "day_9/src/example.txt";
        fileutils::lines_from_file(filename)
    }

    #[test]
    fn example_case_part1() {
        let result = part1(example_input());
        assert_eq!(result, 13140);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(example_input());
        assert_eq!(result, 1);
    }
}
