mod engine;

use std::collections::HashSet;

use engine::Engine;

pub fn main() {
    let filename = "day_3/src/input.txt";
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

fn lower_bound(value: usize) -> usize {
    if value > 0 {
        value - 1
    } else {
        value
    }
}

fn get_valid_parts(engine: &Engine) -> Vec<(char, HashSet<engine::PartNumber>)> {
    let mut valid_parts: Vec<(char, HashSet<engine::PartNumber>)> = vec![];
    engine.parts.iter().for_each(|p| {
        let mut part_numbers: HashSet<engine::PartNumber> = HashSet::new();
        for row in lower_bound(p.row)..=p.row + 1 {
            for col in lower_bound(p.col)..=p.col + 1 {
                if let Some(possible_nums) = engine.part_numbers.get(&row) {
                    possible_nums
                        .iter()
                        .filter(|num| (num.start..=num.end).contains(&col))
                        .for_each(|p| {
                            part_numbers.insert(*p);
                        });
                }
            }
        }
        valid_parts.push((p.symbol, part_numbers));
    });
    valid_parts
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    let engine = Engine::parse(lines);

    let valid_parts = get_valid_parts(&engine);

    return valid_parts
        .iter()
        .flat_map(|(_, parts)| parts.iter().map(|p| p.id))
        .sum();
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u32 {
    let engine = Engine::parse(lines);

    let valid_parts = get_valid_parts(&engine);

    return valid_parts
        .iter()
        .filter(|(symbol, parts)| *symbol == '*' && parts.len() == 2)
        .map(|(_, parts)| parts.iter().map(|p| p.id).product::<u32>())
        .sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    const EXAMPLE_INPUT_PART2: [&str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    const REDDIT_EXAMPLE_INPUT1: [&str; 12] = [
        "12.......*..", // ok
        "+.........34", // miss 34
        ".......-12..", // ok
        "..78........", // ok
        "..*....60...", // ok
        "78..........", // ok
        ".......23...", // ok
        "....90*12...", // ok
        "............", // ok
        "2.2......12.", // ok
        ".*.........*", // ok
        "1.1.......56", // miss 56
    ];

    const REDDIT_EXAMPLE_INPUT2: [&str; 12] = [
        "12.......*..",
        "+.........34",
        ".......-12..",
        "..78........",
        "..*....60...",
        "78.........9",
        ".5.....23..$",
        "8...90*12...",
        "............",
        "2.2......12.",
        ".*.........*",
        "1.1..503+.56",
    ];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 4361);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(
            &EXAMPLE_INPUT_PART2
                .iter()
                .map(|x| String::from(*x))
                .collect(),
        );
        assert_eq!(result, 467835);
    }

    #[test]
    fn reddit_example_case_part1() {
        let result = part1(
            &REDDIT_EXAMPLE_INPUT1
                .iter()
                .map(|x| String::from(*x))
                .collect(),
        );
        assert_eq!(result, 413);
    }

    #[test]
    fn reddit_example_case_part2() {
        let result = part1(
            &REDDIT_EXAMPLE_INPUT2
                .iter()
                .map(|x| String::from(*x))
                .collect(),
        );
        assert_eq!(result, 925);
    }

    #[test]
    fn test_parsing1() {
        let engine = Engine::parse(&vec![String::from("............409..........784...578...802......64..............................486.248..............177....................369...............")]);
        let numbers = engine
            .part_numbers
            .get(&0)
            .unwrap()
            .iter()
            .map(|p| p.id)
            .collect::<Vec<_>>();
        assert_eq!(numbers, vec![409, 784, 578, 802, 64, 486, 248, 177, 369]);
    }
}
