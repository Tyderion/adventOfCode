pub fn main() {
    execute::load_and_execute("day_11/src/input.txt", part1, part2);
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

    const EXAMPLE_INPUT1: [&str; 9] = [
        todo!()
    ];


    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, todo!());
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT2.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, todo!());
    }
}