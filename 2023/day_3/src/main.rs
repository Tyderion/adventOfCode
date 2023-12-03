mod engine;

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

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    return Engine::parse(lines)
        .get_part_list()
        .iter()
        .flat_map(|(_, part_numbers)| part_numbers.iter().map(|p| p.id))
        .sum();
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u32 {
    return Engine::parse(lines)
        .get_part_list()
        .iter()
        .filter(|(part, part_numbers)| part.symbol == '*' && part_numbers.len() == 2)
        .map(|(_, parts)| parts.iter().map(|p| p.id).product::<u32>())
        .sum();
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
        "12.......*..",
        "+.........34",
        ".......-12..",
        "..78........",
        "..*....60...",
        "78..........",
        ".......23...",
        "....90*12...",
        "............",
        "2.2......12.",
        ".*.........*",
        "1.1.......56",
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
