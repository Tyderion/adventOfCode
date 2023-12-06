use std::ops::Range;

pub fn main() {
    let filename = "day_5/src/input.txt";
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

#[derive(Debug)]
struct DependencyMapEntry {
    destination: u64,
    source: Range<u64>,
}

impl DependencyMapEntry {
    pub fn from(line: impl AsRef<str>) -> Option<DependencyMapEntry> {
        let parts = line
            .as_ref()
            .split(" ")
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<_>>();

        match parts.len() {
            3 => Some(DependencyMapEntry {
                destination: parts[0],
                source: parts[1]..(parts[1] + parts[2]),
            }),
            _ => None,
        }
    }

    fn map(&self, value: u64) -> Option<u64> {
        match self.source.contains(&value) {
            true => Some(value - self.source.start + self.destination),
            false => None,
        }
    }
}

#[derive(Debug)]
struct GardenInstructions {
    seeds: Vec<u64>,
    dependencies: Vec<Vec<DependencyMapEntry>>,
}

fn parse_input(
    lines: &Vec<impl AsRef<str>>,
    parse_seeds: fn(line: &str) -> Vec<u64>,
) -> GardenInstructions {
    lines.iter().fold(
        GardenInstructions {
            seeds: vec![],
            dependencies: vec![],
        },
        |mut acc, ele| {
            let line = ele.as_ref();
            if line.starts_with("seeds: ") {
                acc.seeds = parse_seeds(line)
            }
            if line.is_empty() {
                acc.dependencies.push(vec![]);
            } else if !line.contains(":") {
                if let (Some(map), Some(list)) =
                    (DependencyMapEntry::from(line), acc.dependencies.last_mut())
                {
                    list.push(map)
                }
            }
            acc
        },
    )
}

fn find_min_mapping(instructions: GardenInstructions) -> Option<u64> {
    instructions
        .seeds
        .iter()
        .map(|seed| {
            instructions.dependencies.iter().fold(*seed, |prev, deps| {
                deps.iter().find_map(|dep| dep.map(prev)).unwrap_or(prev)
            })
        })
        .min()
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u64 {
    let instructions = parse_input(lines, |l| {
        l.split(" ").filter_map(|n| n.parse::<u64>().ok()).collect()
    });
    find_min_mapping(instructions).unwrap()
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u64 {
    let instructions = parse_input(lines, |l| {
        l.split(" ").filter_map(|n| n.parse::<u64>().ok()).collect::<Vec<_>>().chunks_exact(2)
        .flat_map(|chunk| {
            let start = *chunk.first().unwrap();
            start..(start+chunk.last().unwrap())
        })
        .collect()
    });
    find_min_mapping(instructions).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 33] = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 35);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 46);
    }
}
