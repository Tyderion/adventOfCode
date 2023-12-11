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

#[derive(Debug)]
struct Reading(Vec<i32>);

impl From<&str> for Reading {
    fn from(value: &str) -> Self {
        Reading(
            value
                .split(" ")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect(),
        )
    }
}

fn calculate_diff_tree(values: &Vec<i32>, acc: &mut Vec<Vec<i32>>) {
    let (_, levels) = values.iter().fold(
        (None as Option<i32>, vec![] as Vec<i32>),
        |(prev, mut list), ele| match prev {
            None => (Some(*ele), vec![]),
            Some(p) => {
                list.push(ele - p);
                (Some(*ele), list)
            }
        },
    );
    acc.push(levels.clone());
    if levels.iter().any(|ele| ele != &0) {
        calculate_diff_tree(&levels, acc)
    }
}

impl Reading {
    pub fn next_value(&self) -> i32 {
        let mut levels = vec![self.0.clone()] as Vec<Vec<i32>>;
        calculate_diff_tree(&self.0, &mut levels);
        let next = levels
            .iter()
            .rev()
            .fold(0, |acc, prev| acc + prev.last().unwrap());
        next
    }
}

fn part1(lines: &Vec<impl AsRef<str>>) -> i32 {
    lines
        .iter()
        .map(|s| Reading::from(s.as_ref()))
        .map(|r| r.next_value())
        .sum()
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
        assert_eq!(result, 114 + 1);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, todo!());
    }
}
