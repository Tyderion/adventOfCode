use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Captures;

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

impl Value {
    fn from_list(nums: Vec<i32>) -> Self {
        Value::List(nums.iter().map(|n| Self::Number(*n)).collect())
    }

    fn parse_number<T: AsRef<str>>(input: T) -> Self {
        Self::from_list(
            input
                .as_ref()
                .split(',')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    fn parse<T: AsRef<str>>(input: T) -> Self {
        lazy_static! {
            static ref RE_WRAPPED_NUMBER_LIST: regex::Regex =
                regex::Regex::new(r"^(\[((?:\d+,?)+)\],?)").unwrap();
            static ref RE_UNWRAPPED_NUMBER_LIST: regex::Regex =
                regex::Regex::new(r"^(,?((?:\d+,?)+),?)").unwrap();
            static ref RE_MIXED_LIST: regex::Regex =
                regex::Regex::new(r"^(\[((?:\d+,?)+))").unwrap();
            static ref RE_NESTED_LIST: regex::Regex = regex::Regex::new(r"^\[(\[.*)\]").unwrap();
        }
        fn sanitize_input<T: AsRef<str>>(input: T) -> String {
            let mut sanitized = input.as_ref().to_string();
            while sanitized.starts_with(",") || sanitized.starts_with("]") {
                sanitized = sanitized[1..].to_string();
            }
            if sanitized == "[" {
                sanitized = "".to_string();
            }
            sanitized
        }
        fn get_rest<T: AsRef<str>>(input: T, c: Captures, index: usize) -> String {
            let mut rest = input.as_ref().to_string();
            if let Some(m) = c.get(index) {
                // remove complete match
                rest.replace_range(m.range(), "");
            }

            sanitize_input(rest)
        }
        let input = input.as_ref();
        println!("input: {}", input);
        if let Some(captures) = RE_WRAPPED_NUMBER_LIST.captures(input) {
            if let Some(c) = captures.get(2) {
                let numbers = Self::parse_number(c.as_str());
                let rest = get_rest(input, captures, 1);
                if rest.len() == 0 {
                    return numbers;
                } else {
                    return Self::List(vec![numbers, Self::parse(rest)]);
                }
            }
            panic!("Regex matched but no captured group")
        } else if let Some(captures) = RE_UNWRAPPED_NUMBER_LIST.captures(input) {
            if let Some(c) = captures.get(2) {
                println!("unwrapped number: {}", c.as_str());
                let numbers = Self::parse_number(c.as_str());
                let rest = get_rest(input, captures, 1);
                println!("unwrapped rest: {}", rest);

                if rest.len() == 0 {
                    return numbers;
                } else {
                    return Self::List(vec![numbers, Self::parse(rest)]);
                }
            }
            panic!("Regex matched but no captured group")
        } else if let Some(captures) = RE_NESTED_LIST.captures(input) {
            Value::parse(captures.get(1).unwrap().as_str())
        } else if input == "[]" {
            Value::List(vec![])
        } else if let Some(captures) = RE_MIXED_LIST.captures(input) {
            fn find_nested_group<T: AsRef<str>>(input: T) -> (String, String) {
                let mut open = 0;
                let mut close = 0;
                let mut start = 0;
                let mut end = 0;
                for (i, c) in input.as_ref().chars().enumerate() {
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
                let mut rest = input.as_ref().to_string();
                let nested = rest.drain(start..=end).collect::<String>();
                
                (nested, sanitize_input(rest))
            }

            if let Some(c) = captures.get(2) {
                let the_rest = get_rest(input, captures, 1);
                println!("left {}", c.as_str());
                println!("the rest: {}", the_rest);
                let (nested, rest) = find_nested_group(the_rest);
                println!("nested: {}", nested);
                println!("rest: {}", rest);
                let left_values = Self::parse(c.as_str());
                println!("left_values: {:?}", left_values);
                let right_values = match rest.len() {
                    0 => None,
                    _ => Some(Self::parse(rest)),
                };
                println!("right_values: {:?}", right_values);
                let nested_value = Self::parse(nested);
                println!("nested_value: {:?}", nested_value);
                let mut result = vec![];
                if let Value::List(left) = left_values {
                    result.extend(left);
                } else {
                    result.push(left_values);
                }
                result.push(nested_value);
                match right_values {
                    Some(Value::List(right)) => result.extend(right),
                    Some(Value::Number(n)) => result.push(Value::Number(n)),
                    None => (),
                }
                return Value::List(result);
            }
            Value::List(vec![])
        } else {
            todo!()
        }
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

    #[test_case("[1,1,3,1,1]", Value::from_list(vec![1,1,3,1,1]); "wrapped simple list")]
    #[test_case(",1,3,1,", Value::from_list(vec![1,3,1]); "unwrapped simple list")]
    #[test_case("[1]", Value::from_list(vec![1]); "list of one")]
    #[test_case("[[1]]", Value::List(vec![Value::from_list(vec![1])]); "simple nested list")]
    #[test_case("[[1],[2,3,4]]", Value::List(vec![Value::from_list(vec![1]),Value::from_list(vec![2,3,4])]); "complex nested list")]
    #[test_case("[]", Value::List(vec![]); "empty list")]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", Value::List(vec![Value::Number(1), Value::List(vec![Value::Number(2), Value::List(vec![Value::Number(3), Value::List(vec![Value::Number(4), Value::from_list(vec![5, 6, 7])])])]), Value::Number(8), Value::Number(9)]); "mixed list")]
    #[test_case("[1,10,[2,[3,[4,[5,6,7]]]],8,9]", Value::List(vec![Value::Number(1),Value::Number(10), Value::List(vec![Value::Number(2), Value::List(vec![Value::Number(3), Value::List(vec![Value::Number(4), Value::from_list(vec![5, 6, 7])])])]), Value::Number(8), Value::Number(9)]); "mixed list 2")]

    fn parse_value<T: AsRef<str>>(input: T, expected: Value) {
        println!("input: {}", input.as_ref());
        assert_eq!(Value::parse(input), expected)
    }

    #[test]
    fn bubu() {
        let result =
            Value::parse("[[5,[[3,0]],[0,[8,6,9],2,9],[5,6,[2,8,3],[0]],[6,2,[2,6,8],10]]]");
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
