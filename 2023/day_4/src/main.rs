use std::collections::{hash_map::RandomState, HashMap, HashSet};

pub fn main() {
    let filename = "day_4/src/input.txt";
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

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

fn parse_cards(lines: &Vec<impl AsRef<str>>) -> Vec<Card> {
    lines
        .iter()
        .map(|l| {
            let card_parts = l.as_ref().split(':').collect::<Vec<_>>();

            let id = card_parts[0]
                .split(" ")
                .filter_map(|p| p.parse::<u32>().ok())
                .sum();

            let parts = card_parts[1].split("|").collect::<Vec<_>>();
            Card {
                id,
                winning_numbers: HashSet::from_iter(
                    parts[0]
                        .split(" ")
                        .filter_map(|num| num.trim().parse::<u32>().ok()),
                ),
                numbers: parts[1]
                    .split(" ")
                    .filter_map(|num| num.parse::<u32>().ok())
                    .collect(),
            }
        })
        .collect()
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    parse_cards(lines)
        .iter()
        .map(|card| {
            card.numbers
                .iter()
                .fold(0u32, |acc, num| match card.winning_numbers.contains(num) {
                    true => acc + 1,
                    false => acc,
                })
        })
        .filter(|n| *n > 0)
        .map(|count| 2u32.pow(count - 1))
        .sum()
}

fn count_cards(cards: &HashMap<u32, Vec<u32>, RandomState>) -> u32 {
    let mut storage: HashMap<u32, u32, RandomState> = HashMap::new();
    let max = *cards.keys().max().unwrap();

    (1..=max).rev().for_each(|id| {
        let card = cards.get(&id).unwrap();
        let card_amount = card.iter().filter_map(|c| storage.get(c)).sum::<u32>() + 1;
        storage.insert(id, card_amount);
    });

    storage.values().sum()
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u32 {
    let cards: HashMap<u32, Vec<u32>, RandomState> = HashMap::from_iter(
        parse_cards(lines)
            .iter()
            .map(|card| {
                (
                    card.id,
                    card.numbers.iter().fold(0u32, |acc, num| {
                        match card.winning_numbers.contains(num) {
                            true => acc + 1,
                            false => acc,
                        }
                    }),
                )
            })
            .map(|(id, count)| {
                (
                    id,
                    if count > 0 {
                        (id + 1..=id + count).collect()
                    } else {
                        vec![]
                    },
                )
            }),
    );

    count_cards(&cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 6] = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[ignore]
    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 13);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 30);
    }
}
