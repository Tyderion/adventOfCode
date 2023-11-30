use std::collections::HashSet;
use bounded_vec_deque::BoundedVecDeque;
fn main() {
    let filename = "day_6/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.first().unwrap());
    println!("Part1 position of key: {}", part1_result);

    let part2_result = part2(lines.first().unwrap());
    println!("Part2 position of message: {}", part2_result);
}

fn part1(line: &str) -> u32 {
    find_position_of_first_n_unique_chars(line, 4)
}

fn part2(line: &str) -> u32 {
    find_position_of_first_n_unique_chars(line, 14)
}

fn find_position_of_first_n_unique_chars(line: &str, n: usize) -> u32 {
    let mut last_elements: BoundedVecDeque<char> = BoundedVecDeque::new(n-1);
    for (index, c) in line.char_indices() {
        if last_elements.contains(&c) {
            last_elements.push_front(c);
        } else if HashSet::<_>::from_iter(last_elements.iter()).len() == n-1 {
            return (index + 1) as u32;
        } else {
            last_elements.push_front(c);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test_case(EXAMPLE_1,  7; "example 1")]
    #[test_case(EXAMPLE_2,  5; "example 2")]
    #[test_case(EXAMPLE_3,  6; "example 3")]
    #[test_case(EXAMPLE_4,  10; "example 4")]
    #[test_case(EXAMPLE_5,  11; "example 5")]
    fn example_cases_part1(input: &str, expected: u32) {
        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test_case(EXAMPLE_1,  19; "example 1")]
    #[test_case(EXAMPLE_2,  23; "example 2")]
    #[test_case(EXAMPLE_3,  23; "example 3")]
    #[test_case(EXAMPLE_4,  29; "example 4")]
    #[test_case(EXAMPLE_5,  26; "example 5")]
    fn example_cases_part2(input: &str, expected: u32) {
        let result = part2(input);
        assert_eq!(result, expected);
    }
}
