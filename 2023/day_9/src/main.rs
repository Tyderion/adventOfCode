pub fn main() {
    execute::load_and_execute("day_9/src/input.txt", part1, part2);
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
    fn extrapolate_value(&self, accumulate: fn(i32, &Vec<i32>) -> i32) -> i32 {
        let mut levels = vec![self.0.clone()] as Vec<Vec<i32>>;
        calculate_diff_tree(&self.0, &mut levels);
        levels.iter().rev().fold(0, accumulate)
    }
    pub fn next_value(&self) -> i32 {
        self.extrapolate_value(|acc, prev| acc + prev.last().unwrap())
    }

    pub fn prev_value(&self) -> i32 {
        self.extrapolate_value(|acc, prev| prev.first().unwrap() - acc)
    }
}

fn part1(lines: &Vec<impl AsRef<str>>) -> i32 {
    lines
        .iter()
        .map(|s| Reading::from(s.as_ref()))
        .map(|r| r.next_value())
        .sum()
}

fn part2(lines: &Vec<impl AsRef<str>>) -> i32 {
    lines
        .iter()
        .map(|s| Reading::from(s.as_ref()))
        .map(|r| r.prev_value())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 3] = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    const EXAMPLE_INPUT1_3: [&str; 1] = ["10 13 16 21 30 45"];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 114);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1_3.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 5);
    }
}