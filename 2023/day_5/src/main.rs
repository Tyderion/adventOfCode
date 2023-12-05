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
    destination_start: u64,
    source_start: u64,
    length: u64,
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
                destination_start: parts[0],
                source_start: parts[1],
                length: parts[2],
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct GardenInstructions {
    seeds: Vec<u64>,
    dependencies: Vec<Vec<DependencyMapEntry>>,
}

fn parse_input(lines: &Vec<impl AsRef<str>>) -> GardenInstructions {
    lines.iter().fold(
        GardenInstructions {
            seeds: vec![],
            dependencies: vec![],
        },
        |mut acc, ele| {
            let line = ele.as_ref();
            if line.starts_with("seeds: ") {
                acc.seeds = line
                    .split(" ")
                    .filter_map(|n| n.parse::<u64>().ok())
                    .collect()
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

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    let instructions = parse_input(lines);
    println!("{:#?}", instructions);
    0
}

fn part2(_lines: &Vec<impl AsRef<str>>) -> u32 {
    0
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
}
