use std::{
    collections::{BTreeMap, HashMap},
    iter::{Map, Rev},
};

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
    new_monkeys
}

fn do_round(round: i32, ids: Vec<&MonkeyId>, monkeys: BTreeMap::<MonkeyId, Monkey>) -> BTreeMap::<MonkeyId, Monkey> {
    let mut new_monkeys = monkeys.clone();
    for id in ids {
        new_monkeys = do_turn(*id, new_monkeys);
    }
    // println!("--- after round {} ---", round);
    // for monkey in new_monkeys.values() {
    //     println!("Monkey {:?}", monkey);
    // }
    new_monkeys
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> i32 {
    let monkeys = parse_monkeys(lines);
    // for monkey in monkeys.values() {
    //     println!("Monkey {:?}", monkey);
    // }
    let ids = monkeys.keys().collect::<Vec<_>>();

    let mut new_monkeys = monkeys.clone();
    for i in 1..=20 {
        new_monkeys = do_round(i, ids.clone(), new_monkeys);
    }
    let mut all_inspections = new_monkeys.values().map(|m| m.get_inspection_count()).collect::<Vec<i32>>();
    all_inspections.sort();
    all_inspections.reverse();

    // println!("all_inspections: {:?}", all_inspections);

    all_inspections.iter().take(2).fold(1, |a, b| a * b)
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> i32 {
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

    // #[test]
    // fn example_monkey0_turn1() {
    //     let mut monkeys = parse_monkeys(example_input());
    //     let monkey0 = monkeys.get_mut(&MonkeyId::new(0)).unwrap();
    //     let result = monkey0.inspect();
    //     for (id, item) in result.iter() {
    //         monkeys.get_mut(id).unwrap().catch_item(*item);
    //     }
    //     assert_eq!(monkey0.get_items()[..], []);
    //     for (id, monkey) in monkeys {
    //         if id == MonkeyId::new(3) {
    //             assert_eq!(monkey.get_items()[..], [74, 500, 620]);
    //         }
    //     }
    // }

    #[test]
    fn example_case_part1() {
        let result = part1(example_input());
        assert_eq!(result, 10605);
    }

    #[test]
    fn example_case_part2() {
        // Cannot be tested as it draws letters on the command line
        let result = part2(example_input());
        assert_eq!(result, 2);
    }
}
