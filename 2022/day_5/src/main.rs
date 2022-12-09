use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let filename = "day_5/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Part1 Top of Stacks: {}", part1_result);

    let part2_result = part2(lines);
    println!("Part2: {}", part2_result);
}

// Vectors are stacks (push/pop) -> map crates into vec and then push/pop the moves
type Stack = Vec<char>;

#[derive(Debug)]
struct UnloadingPlan {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

lazy_static! {
    static ref RE_MOVE: Regex =
        Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
}

impl UnloadingPlan {
    fn parse_stacks(state: &mut Vec<String>) -> Vec<Stack> {
        let crate_indices =
            state
                .pop()
                .unwrap()
                .char_indices()
                .fold(vec![], |mut result, (index, value)| match value {
                    ' ' => result,
                    _ => {
                        result.push(index);
                        result
                    }
                });
        let number_of_stacks = crate_indices.len();
        let mut stacks = vec![Vec::<char>::new(); number_of_stacks];

        while let Some(row) = state.pop() {
            crate_indices
                .iter()
                .enumerate()
                .for_each(
                    |(stack_index, crate_index)| match row.chars().nth(*crate_index) {
                        None => (),
                        Some(c) if c.is_alphabetic() => stacks[stack_index].push(c),
                        Some(_) => (),
                    },
                );
        }

        stacks
    }

    fn parse_moves(moves: &Vec<String>) -> Vec<Move> {
        moves.iter().map(|move_str| Self::parse_move(move_str)).collect()
    }

    fn parse_move(mv: &str) -> Move {
        match RE_MOVE.captures(mv) {
            Some(capture) => Move {
                amount: capture.name("amount").unwrap().as_str().parse::<i32>().unwrap(),
                from: capture.name("from").unwrap().as_str().parse::<i32>().unwrap(),
                to: capture.name("to").unwrap().as_str().parse::<i32>().unwrap(),
            },
            None => panic!("Invalid move: {}", mv),
        }
    }

    pub fn from(lines: Vec<String>) -> UnloadingPlan {
        let split = lines.iter().position(|l| l.is_empty()).unwrap();

        return UnloadingPlan {
            stacks: Self::parse_stacks(&mut lines[0..split].to_vec()),
            moves: Self::parse_moves(&lines[split + 1..lines.len()].to_vec()),
        };
    }

}

#[derive(Debug)]
struct Move {
    amount: i32,
    from: i32,
    to: i32,
}

fn part1(lines: Vec<String>) -> String {
    let plan = UnloadingPlan::from(lines);
    println!("{:?}", plan);
    String::from("")
}

fn part2(_lines: Vec<String>) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 9] = [
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn verify_case() {
        let result = part1(EXAMPLE_INPUT.map(String::from).to_vec());
        assert_eq!(result, "CMZ");
    }
}
