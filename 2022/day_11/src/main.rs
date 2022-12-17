use std::collections::{BTreeMap};

use monkey::MonkeyId;

use crate::monkey::Monkey;

mod monkey;
mod operation;

fn main() {
    let filename = "day_11/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Monkey Business Level: {}", part1_result);

    let part2_result = part2(lines.clone());
    println!("Part2: ??{}", part2_result);
}

fn parse_monkeys<T: AsRef<str>>(lines: Vec<T>) -> BTreeMap<MonkeyId, Monkey> {
    lines
        .chunks(7)
        .map(|chunk| {
            Monkey::from(
                chunk
                    .iter()
                    .map(|s| s.as_ref())
                    .filter(|s| !s.is_empty())
                    .collect(),
            )
        })
        .map(|monkey| (monkey.id, monkey))
        .collect()
}

fn print_monkeys(header: String, monkeys: Box<BTreeMap::<MonkeyId, Monkey>>) {
    println!("{}", header);
    for monkey in monkeys.values() {
        println!("Monkey {:?}", monkey);
    }
}

fn do_turn(id: MonkeyId, monkeys: BTreeMap::<MonkeyId, Monkey>) -> BTreeMap::<MonkeyId, Monkey> {
    let mut new_monkeys = BTreeMap::<MonkeyId, Monkey>::new();
    let monkey = monkeys.get(&id).unwrap();
    let (mk, result) = monkey.inspect();

    new_monkeys.insert(mk.id, mk);
    for (other_id, monkey) in monkeys {
        if other_id != id {
            let mut mk = monkey.clone();
            for (to, item) in result.iter() {
                if to == &other_id {
                    mk = mk.clone().catch_item(*item);
                }
            }
            new_monkeys.insert(mk.id, mk);
        }
    }
    print_monkeys(format!("After turn {:?}", id), Box::new(new_monkeys.clone()));
    new_monkeys
}

fn do_round(round: i32, ids: Vec<&MonkeyId>, monkeys: BTreeMap::<MonkeyId, Monkey>) -> BTreeMap::<MonkeyId, Monkey> {
    let mut new_monkeys = monkeys.clone();
    for id in ids {
        new_monkeys = do_turn(*id, new_monkeys);
    }
    print_monkeys(format!("After Round {}", round), Box::new(new_monkeys.clone()));
    new_monkeys
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> i32 {
    let monkeys = parse_monkeys(lines);

    print_monkeys(format!("Initial state"), Box::new(monkeys.clone()));
    let ids = monkeys.keys().collect::<Vec<_>>();

    let mut new_monkeys = monkeys.clone();
    for i in 1..=20 {
        new_monkeys = do_round(i, ids.clone(), new_monkeys);
    }
    let mut all_inspections = new_monkeys.values().map(|m| m.get_inspection_count()).collect::<Vec<i32>>();
    all_inspections.sort();
    all_inspections.reverse();

    all_inspections.iter().take(2).fold(1, |a, b| a * b)
}

fn part2<T: AsRef<str>>(_lines: Vec<T>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        let filename = "src/example.txt";
        match fileutils::safe_lines_from_file(filename) {
            Some(lines) => lines,
            // When debugging we start in root, else in day_11
            _ => fileutils::lines_from_file("day_11/".to_string() + filename),
        }
    }

    #[test]
    fn example_case_part1() {
        let result = part1(example_input());
        assert_eq!(result, 10605);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(example_input());
        assert_eq!(result, 2);
    }
}
