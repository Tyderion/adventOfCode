#![feature(assert_matches)]
use std::vec;

use either::Either;
use itertools::Itertools;

fn main() {
    let filename = "day_13/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());

    let part2_result = part2(lines.clone());
    println!("Monkey Business Level: {}", part1_result);
    println!("monkey business level part2 : {}", part2_result);
}

#[derive(Debug, Clone)]
enum Value {
    Number(i32),
    List(Vec<Value>),
}

fn sanitize_input<T: AsRef<str>>(input: T) -> String {
    let mut sanitized = input.as_ref().to_string();
    while sanitized.starts_with(",") {
        sanitized = sanitized[1..].to_string();
    }
    sanitized
}

enum P {
    UNWRAPPED(String),
    NUMBERS(String),
    MIXED(String),
}

fn find_numbers<T: AsRef<str>>(input: T) -> (String, String) {
    let mut input = input.as_ref().to_string();
    let mut end = None;

    for (i, c) in input.chars().enumerate() {
        if c != '[' {
            end = Some(i);
        } else {
            break;
        }
    }
    if end.is_none() {
        return ("".to_string(), input);
    }
    let left = input.drain(0..=end.unwrap()).collect::<String>();

    return (left, input);
}

fn find_first_group<T: AsRef<str>>(input: T) -> (String, String) {
    let mut input = input.as_ref().to_string();
    let mut open = 0;
    let mut close = 0;
    let mut start = 0;
    let mut end = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '[' {
            if open == 0 {
                start = i;
            }
            open += 1;
        } else if c == ']' {
            close += 1;
            if open == close {
                end = i;
                break;
            }
        }
    }

    if (open == 0 && close == 0) || (open != close) {
        return ("".to_string(), input);
    }


    let left = input.drain(start..=end).collect::<String>();

    return (left, input);
}

fn split_input<T: AsRef<str>>(input: &T) -> (String, String, String) {
    let (left, rest) = find_numbers(input);
    let (group, right) = find_first_group(rest);
    return (left, group, right);
}

fn split_into_groups<T: AsRef<str>>(
    input: T,
) -> Either<(Option<String>, Option<String>, Option<String>), Option<String>> {
    fn to_option(s: String) -> Option<String> {
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
    let mut open = 0;
    let mut close = 0;
    let mut middle = 0;
    let mut start = None;
    let mut end = 0;
    for (i, c) in input.as_ref().chars().enumerate() {
        if (c.is_numeric() || c == ',') && start.is_none() {
            middle += 1;
        }
        if c == '[' {
            if open == 0 {
                start = Some(i);
            }
            open += 1;
        } else if c == ']' {
            close += 1;
            if open == close {
                end = i;
                break;
            }
        }
    }
    let mut right = input.as_ref().to_string();

    let left = right.drain(0..middle).collect::<String>();
    let middle = if start.is_some() && start.unwrap() >= middle {
        right
            .drain(start.unwrap() - middle..=end - middle)
            .collect::<String>()
    } else {
        "".to_string()
    };
    if left == "" && right == "" {
        return Either::Right(to_option(middle[1..middle.len() - 1].to_string()));
    }

    Either::Left((
        to_option(left),
        to_option(middle),
        to_option(sanitize_input(right)),
    ))
}

impl Value {
    fn parse<T: AsRef<str>>(input: T) -> Option<Self> {
        println!("-------unwrap_line-------");
        if input.as_ref() == "" {
            return None;
        }
        let (left, group, right) = split_input(&input);
        println!("left: {}, group: {}, right: {}", left, group, right);
        let input = input.as_ref().to_string();
        if group.len() == input.len() {
            return Some(Self::List(Self::parse_vec(group[1..group.len() - 1].to_string())));
        }
        let left = Self::parse_numbers(left);
        Some(Self::List(left))
    }

    fn parse_vec<T: AsRef<str>>(input: T) -> Vec<Self> {
        println!("-------parse_vec-------");
        if input.as_ref() == "" {
            return vec![];
        }
        let (left, group, right) = split_input(&input);
        println!("left: {}, group: {}, right: {}", left, group, right);
        let mut result = Self::parse_numbers(left);
        if !group.is_empty() {
            let group = Self::parse(group[1..group.len() - 1].to_string());
            if let Some(group) = group {
                result.push(group);
            }
        }
        result.extend(Self::parse_vec(right));
        result
    }

    fn from_list(nums: Vec<i32>) -> Self {
        Value::List(nums.iter().map(|n| Self::Number(*n)).collect())
    }

    fn as_list(&self) -> Vec<Value> {
        match self {
            Value::Number(n) => vec![Self::Number(*n)],
            Value::List(list) => list.iter().map(|l| l.clone()).collect(),
        }
    }

    fn parse_numbers<T: AsRef<str>>(input: T) -> Vec<Self> {
        input
            .as_ref()
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .map(|n| Self::Number(n.parse::<i32>().unwrap()))
            .collect()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(lnum), Self::Number(rnum)) => lnum == rnum,
            (Self::List(lvec), Self::List(rvec)) => lvec == rvec,
            (Self::List(lvec), Self::Number(rnum)) => match lvec.first() {
                Some(Value::Number(lnum)) => lnum == rnum,
                _ => false,
            },
            (Self::Number(lnum), Self::List(rvec)) => match rvec.first() {
                Some(Value::Number(rnum)) => lnum == rnum,
                _ => false,
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                // println!("Comparing {:?} and {:?}", self, other);
                a.partial_cmp(b)
            }
            (Value::List(a), Value::List(b)) => {
                a.iter().zip_longest(b.iter()).fold(None, |acc, elements| {
                    // println!("Acc: {:?}, elements: {:?}", acc, elements);
                    match acc {
                        Some(std::cmp::Ordering::Equal) | None => match elements {
                            itertools::EitherOrBoth::Both(a, b) => a.partial_cmp(b),
                            itertools::EitherOrBoth::Left(_) => Some(std::cmp::Ordering::Greater),
                            itertools::EitherOrBoth::Right(_) => Some(std::cmp::Ordering::Less),
                        },
                        _ => acc,
                    }
                })
            }
            (Value::List(_), Value::Number(b)) => self.partial_cmp(&Value::from_list(vec![*b])),
            (Value::Number(a), Value::List(_)) => Value::from_list(vec![*a]).partial_cmp(other),
        }
    }
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> usize {
    lines
        .chunks(3)
        .enumerate()
        .map(|(index, chunk)| {
            (
                index,
                chunk[0..2]
                    .iter()
                    .map(|x| x.as_ref())
                    .collect::<Vec<&str>>(),
            )
        })
        .map(|(index, parts)| (index, Value::parse(parts[0]), Value::parse(parts[1])))
        .filter_map(|(index, left, right)| match left.partial_cmp(&right) {
            Some(std::cmp::Ordering::Equal) | Some(std::cmp::Ordering::Greater) => None,
            _ => Some(index),
        })
        .sum()
}

fn part2<T: AsRef<str>>(_lines: Vec<T>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::assert_matches::assert_matches;
    use test_case::test_case;

    fn example_input() -> Vec<String> {
        let filename = "src/example.txt";
        match fileutils::safe_lines_from_file(filename) {
            Some(lines) => lines,
            // When debugging we start in root, else in day_11
            _ => fileutils::lines_from_file("day_13/".to_string() + filename),
        }
    }

    #[test_case(Value::Number(1), Value::Number(1), true; "number eq")]
    #[test_case(Value::from_list(vec![1, 2, 3]), Value::from_list(vec![1, 2, 3]), true; "list eq")]
    #[test_case(Value::from_list(vec![1, 2, 3]), Value::Number(1), true; "number list eq")]
    #[test_case(Value::Number(1), Value::from_list(vec![1, 2, 3]), true; "number list 2 eq")]
    #[test_case(Value::Number(1), Value::Number(2), false; "number neq")]
    #[test_case(Value::from_list(vec![1, 2, 3]), Value::from_list(vec![2, 2, 3]), false; "list neq")]
    #[test_case(Value::from_list(vec![1, 2, 3]), Value::Number(2), false; "number list neq")]
    #[test_case(Value::Number(2), Value::from_list(vec![1, 2, 3]), false; "number list 2 neq")]
    fn value_eq(left: Value, right: Value, expected: bool) {
        assert!((left == right) == expected)
    }

    // #[test_case(
    //     Value::from_list(vec![1,1,3,1,1]),
    //     Value::from_list(vec![1,1,5,1,1]),
    //     std::cmp::Ordering::Less;
    //     "example1")]
    // #[test_case(
    //     Value::List(vec![Value::from_list(vec![1]),Value::from_list(vec![2,3,4])]),
    //     Value::List(vec![Value::from_list(vec![1]),Value::Number(4)]),
    //     std::cmp::Ordering::Less;
    //     "example2")]
    // #[test_case(
    //     Value::from_list(vec![9]),
    //     Value::List(vec![Value::from_list(vec![8,7,6])]),
    //     std::cmp::Ordering::Greater;
    //     "example3")]
    // #[test_case(
    //     Value::List(vec![Value::from_list(vec![4,4]),Value::Number(4), Value::Number(4)]),
    //     Value::List(vec![Value::from_list(vec![4,4]),Value::Number(4), Value::Number(4), Value::Number(4)]),
    //     std::cmp::Ordering::Less;
    //     "example4")]
    // #[test_case(
    //     Value::from_list(vec![7,7,7,7]),
    //     Value::from_list(vec![7,7,7]),
    //     std::cmp::Ordering::Greater;
    //     "example5")]
    // #[test_case(
    //     Value::from_list(vec![]),
    //     Value::from_list(vec![3]),
    //     std::cmp::Ordering::Less;
    //     "example6")]
    // #[test_case(
    //     Value::List(vec![Value::List(vec![Value::List(vec![])])]),
    //     Value::List(vec![Value::List(vec![])]),
    //     std::cmp::Ordering::Greater;
    //     "example7")]
    // #[test_case(
    //     //[1,[2,[3,[4,[5,6,7]]]],8,9]
    //     Value::List(vec![Value::Number(1), Value::List(vec![Value::Number(2), Value::List(vec![Value::Number(3), Value::List(vec![Value::Number(4), Value::from_list(vec![5, 6, 7])])])]), Value::Number(8), Value::Number(9)]),
    //     //[1,[2,[3,[4,[5,6,0]]]],8,9]
    //     Value::List(vec![Value::Number(1), Value::List(vec![Value::Number(2), Value::List(vec![Value::Number(3), Value::List(vec![Value::Number(4), Value::from_list(vec![5, 6, 0])])])]), Value::Number(8), Value::Number(9)]),
    //     std::cmp::Ordering::Greater;
    //     "example8")]
    // fn value_partial_ord_example(left: Value, right: Value, expected: std::cmp::Ordering) {
    //     assert_eq!(left.partial_cmp(&right), Some(expected))
    // }

    #[test_case("[]", ("", "[]", ""); "empty list")]
    #[test_case("[[]]", ("", "[[]]", ""); "nested empty list")]
    #[test_case("1", ("1", "", ""); "number")]
    #[test_case("[1]", ("", "[1]", ""); "nested number")]
    #[test_case("1,2,3", ("1,2,3", "", ""); "number list")]
    #[test_case("[1,2,3]", ("", "[1,2,3]", ""); "nested number list")]
    #[test_case("1,[1]", ("1,", "[1]", ""); "mixed number")]
    #[test_case("1,2,[1]", ("1,2,", "[1]", ""); "mixed multi number")]
    #[test_case("1,[]", ("1,", "[]", ""); "mixed number and empty")]
    #[test_case("1,2,[]", ("1,2,", "[]", ""); "mixed multi number and empty")]
    #[test_case("[1,2,[]]", ("", "[1,2,[]]", ""); "nested mixed multi number and empty")]
    #[test_case("[1],1", ("", "[1]", ",1"); "mixed with right part number")]
    #[test_case("[1],1,2", ("", "[1]", ",1,2"); "mixed with right part multi number")]
    #[test_case("[1],1,2,[]", ("", "[1]", ",1,2,[]"); "mixed with right part mixed")]
    #[test_case("1,2,[1],1,2,[]", ("1,2,", "[1]", ",1,2,[]"); "3 part mixed")]
    #[test_case("[1,2,[3],[4],5]", ("", "[1,2,[3],[4],5]", ""); "complex nested list")]
    #[test_case("1,2,[3],[4],5", ("1,2,", "[3]", ",[4],5"); "complex unwrapped list")]
    fn test_split_input(input: &str, expected: (&str, &str, &str)) {
        let result = split_input(&input);
        assert_eq!(
            result,
            (
                expected.0.to_string(),
                expected.1.to_string(),
                expected.2.to_string()
            )
        )
    }

    
    // #[test_case("[1,1,3,1,1]", Value::from_list(vec![1,1,3,1,1]); "wrapped simple list")]
    // #[test_case(",1,3,1,", Value::from_list(vec![1,3,1]); "unwrapped simple list")]
    // #[test_case("[1]", Value::from_list(vec![1]); "list of one")]
    // #[test_case("[[1]]", Value::List(vec![Value::from_list(vec![1])]); "simple nested list")]
    // #[test_case("[[1],[2,3,4]]", Value::List(vec![Value::from_list(vec![1]),Value::from_list(vec![2,3,4])]); "complex nested list")]
    // #[test_case("[]", Value::List(vec![]); "empty list")]
    #[test_case("[1,[2]]", Value::List(vec![Value::Number(1), Value::List(vec![Value::Number(2)])]); "simple mixed nested list")]
    #[test_case("[1,[2],3]", Value::List(vec![Value::Number(1), Value::List(vec![Value::Number(2)]), Value::Number(3)]); "simple contained list")]
    #[test_case("[1,2,[3]]", Value::List(vec![Value::Number(1), Value::Number(2), Value::List(vec![Value::Number(3)])]); "combined list")]
    #[test_case("[1,2,[3],[4]]", Value::List(vec![Value::Number(1), Value::Number(2), Value::List(vec![Value::Number(3)]), Value::List(vec![Value::Number(4)])]); "combined list2")]
    #[test_case("[1,2,[3],4]", Value::List(vec![Value::Number(1), Value::Number(2), Value::List(vec![Value::Number(3)]), Value::Number(4)]); "combined list3")]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", Value::List(vec![Value::Number(1), Value::List(vec![Value::Number(2), Value::List(vec![Value::Number(3), Value::List(vec![Value::Number(4), Value::from_list(vec![5, 6, 7])])])]), Value::Number(8), Value::Number(9)]); "mixed list")]
    #[test_case("[1,10,[2,[3,[4,[5,6,7]]]],8,9]", Value::List(vec![Value::Number(1),Value::Number(10), Value::List(vec![Value::Number(2), Value::List(vec![Value::Number(3), Value::List(vec![Value::Number(4), Value::from_list(vec![5, 6, 7])])])]), Value::Number(8), Value::Number(9)]); "mixed list 2")]

    fn parse_value<T: AsRef<str>>(input: T, expected: Value) {
        let result = Value::parse(&input).unwrap();
        println!("input: {} -> {:?}", input.as_ref(), result);
        assert_eq!(result, expected)
    }

    #[test]
    fn bubu() {
        let result =
            Value::parse("[[5,[[3,0]],[0,[8,6,9],2,9],[5,6,[2,8,3],[0]],[6,2,[2,6,8],10]]]");
        println!("{:?}", result);
        assert!(false)
    }

    #[test]
    fn example_case_part1() {
        let result = part1(example_input());
        assert_eq!(result, 13);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(example_input());
        assert_eq!(result, 2713310158);
    }
}
