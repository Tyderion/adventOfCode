use std::collections::HashSet;

fn main() {
    let filename = "day_4/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_sum = part1(lines.clone());
    println!("Part1 Sum: {}", part1_sum);

    let part2_sum = part2(lines);
    println!("Part2 Sum: {}", part2_sum);
}

#[derive(Debug)]
struct Pair {
    left: HashSet<i32>,
    right: HashSet<i32>,
}

fn part1(lines: Vec<String>) -> usize {
    ranges_from_strings(lines)
        .iter()
        .filter(|pair| is_subset(pair))
        .count()
}

fn part2(lines: Vec<String>) -> usize {
    ranges_from_strings(lines)
        .iter()
        .filter(|pair| has_common_elements(pair))
        .count()
}

fn ranges_from_strings(lines: Vec<String>) -> Vec<Pair> {
    lines
        .iter()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|parts| match parts.as_slice() {
            [left, right] => Pair {
                left: parse_range(left),
                right: parse_range(right),
            },
            a => panic!("Invalid input {:?}", a),
        })
        .collect()
}

fn parse_range(range: &str) -> HashSet<i32> {
    let parts = range
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (parts[0]..=parts[1]).collect::<HashSet<i32>>()
}

fn is_subset(pair: &Pair) -> bool {
    let Pair { left, right } = pair;
    let intersect = left & right;
    &intersect == left || &intersect == right
}

fn has_common_elements(pair: &Pair) -> bool {
    let Pair { left, right } = pair;
    (left & right).len() > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn example_case_part1() {
        let result = part1(
            [
                "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
            ]
            .map(String::from)
            .to_vec(),
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(
            [
                "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
            ]
            .map(String::from)
            .to_vec(),
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn parse_range_into_hashset() {
        let result = parse_range("2-4");
        assert_eq!(result, HashSet::from([2, 3, 4]));
    }

    #[test_case(&[1], &[1, 2], true; "[1] is subset of [1, 2]")]
    #[test_case(&[1, 2], &[1], true; "[1, 2] fully contains [1]")]
    #[test_case(&[1, 2, 3, 4], &[5, 6], false; "[1, 2, 3, 4] does not contain [5, 6]")]
    #[test_case(&[5, 6], &[1, 2, 3, 4],  false; "[5, 6] is not subset of [1, 2, 3, 4]")]
    fn fully_contains(left: &[i32], right: &[i32], expected: bool) {
        let result = is_subset(&Pair {
            left: HashSet::from_iter(left.iter().map(|x| *x)),
            right: HashSet::from_iter(right.iter().map(|x| *x)),
        });
        assert_eq!(result, expected);
    }
}
