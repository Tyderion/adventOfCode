use std::{fmt::Debug, str::FromStr};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

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
        let crate_indices: Vec<usize> = state
            .last()
            .unwrap()
            .char_indices()
            .filter_map(|(index, value)| match value {
                ' ' => None,
                _ => Some(index),
            })
            .collect();

        let mut stacks = vec![Vec::<char>::new(); crate_indices.len()];

        while let Some(row) = state.pop() {
            for (stack_index, crate_index) in crate_indices.iter().enumerate() {
                match row.chars().nth(*crate_index) {
                    Some(c) if c.is_alphabetic() => stacks[stack_index].push(c),
                    _ => (),
                }
            }
        }

        stacks
    }

    fn parse_moves(moves: &Vec<String>) -> Vec<Move> {
        moves.iter().map(Self::parse_move).collect()
    }

    fn parse_move<T: AsRef<str>>(mv: T) -> Move {
        fn get_value_from_group<S: FromStr>(name: &str, capture: &Captures) -> S
        where
            <S as FromStr>::Err: Debug,
        {
            capture.name(name).unwrap().as_str().parse().unwrap()
        }

        if let Some(capture) = RE_MOVE.captures(mv.as_ref()) {
            Move {
                amount: get_value_from_group("amount", &capture),
                from: get_value_from_group("from", &capture),
                to: get_value_from_group("to", &capture),
            }
        } else {
            panic!("Invalid move: {}", mv.as_ref());
        }
    }

    pub fn from(lines: Vec<String>) -> UnloadingPlan {
        let split = lines.iter().position(|l| l.is_empty()).unwrap();

        return UnloadingPlan {
            stacks: Self::parse_stacks(&mut lines[0..split].to_vec()),
            moves: Self::parse_moves(&lines[split + 1..].to_vec()),
        };
    }

    pub fn execute_moves(&mut self, move_strategy: fn(&mut UnloadingPlan, Move)) {
        for mv in self.moves.clone() {
            move_strategy(self, mv);
        }
    }

    pub fn move_strategy_part1(&mut self, mv: Move) {
        for _ in 0..mv.amount {
            let moved = self.stacks[mv.from - 1].pop().unwrap();
            self.stacks[mv.to - 1].push(moved)
        }
    }

    pub fn move_strategy_part2(&mut self, mv: Move) {
        let insert_start = self.stacks[mv.to - 1].len();
        for _ in 0..mv.amount {
            let element = self.stacks[mv.from - 1].pop().unwrap();
            self.stacks[mv.to - 1].insert(insert_start, element);
        }
    }

    pub fn result(&mut self) -> String {
        self.stacks
            .iter()
            .filter_map(|s| s.last().to_owned())
            .into_iter()
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

fn part1(lines: Vec<String>) -> String {
    let mut plan = UnloadingPlan::from(lines);
    plan.execute_moves(UnloadingPlan::move_strategy_part1);
    String::from(plan.result())
}

fn part2(lines: Vec<String>) -> String {
    let mut plan = UnloadingPlan::from(lines);
    plan.execute_moves(UnloadingPlan::move_strategy_part2);
    String::from(plan.result())
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

    #[test]
    fn verify_case_part2() {
        let result = part2(EXAMPLE_INPUT.map(String::from).to_vec());
        assert_eq!(result, "MCD");
    }
}
