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

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    let engine = Engine::parse(lines);
    println!("{:?}", engine);
    println!("----------------");
    let mut valid_parts: HashSet<engine::PartNumber> = HashSet::new();
    engine.parts.iter().for_each(|p| {
        let row_range = lower_bound(p.row)..=p.row + 1;
        let col_range = lower_bound(p.col)..=p.col+1;
        println!(
            "{:?} looking in rows {:?} and in cols {:?}",
            p, row_range, col_range
        );
        for row in row_range {
            for col in lower_bound(p.col)..=p.col+1 {
                if let Some(possible_nums) = engine.part_numbers.get(&row) {
                    let valid_nums = possible_nums
                        .iter()
                        .filter(|num| (num.start..=num.end).contains(&col))
                        .collect::<Vec<_>>();
                    if valid_nums.len() > 0 {
                        println!("{:?} has numbers: {:?}", p, valid_nums);
                    }
                    valid_nums.iter().for_each(|p| {
                        valid_parts.insert(**p);
                    });
                }
            }
        }
    });
    println!("----------------");
    println!(
        "parts {:?}",
        valid_parts.iter().map(|p| p.id).collect::<Vec<_>>()
    );
    return valid_parts.iter().map(|e| e.id).sum();
}

fn part2(_lines: &Vec<impl AsRef<str>>) -> u32 {
    return 0;
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

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 4361);
    }
}
