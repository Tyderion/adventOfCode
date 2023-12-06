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

    fn map_range(&self, b: Range<u64>) -> (Option<Range<u64>>, Option<Range<u64>>) {
        if self.source.contains(&b.start) {
            if self.source.contains(&b.end) {
                let new_start = b.start - self.source.start + self.destination;
                let new_end = b.end - self.source.start + self.destination;
                return (None, Some(new_start..new_end));
            }
            let new_start = b.start - self.source.start + self.destination;
            let new_end = self.source.end - self.source.start + self.destination;

            let lower_start = b.start + b.end - self.source.end;
            return (Some(lower_start..b.end), Some(new_start..new_end));
            // return vec![(b.start - self.start)..self.end, self.end+1..b.end]
        } else if self.source.contains(&b.end) {
            let new_end = b.end - self.source.start + self.destination;
            return (
                Some(b.start..self.source.start),
                Some(self.destination..new_end),
            );
        }
        (Some(b), None)
    }
}

#[derive(Debug)]
struct GardenInstructions {
    seeds: Vec<Range<u64>>,
    dependencies: Vec<Vec<DependencyMapEntry>>,
}

fn parse_input(
    lines: &Vec<impl AsRef<str>>,
    parse_seeds: fn(line: &str) -> Vec<Range<u64>>,
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
    let mapping = instructions
        .seeds
        .iter()
        // .take(1)
        .map(|seed| {
            let init: (Vec<Range<u64>>, Vec<Range<u64>>) = (vec![seed.clone()], vec![]);
            instructions
                .dependencies
                .iter()
                // .take(3)
                .fold(init, |(unmapped, mut mapped), deps| {
                    // println!("DependencyMaps: {:?}", deps);
                    let newly_mapped = unmapped
                        .iter()
                        .map(|u| {
                            let init: (Option<Range<u64>>, Option<Range<u64>>) =
                                (Some(u.clone()), None);
                            let after_dependency_maps =
                                deps.iter().fold(init, |(unmapped, mapped), dep| {
                                    if let Some(s) = unmapped {
                                        return dep.map_range(s);
                                    }
                                    (None, mapped)
                                });

                            // println!("after_dependency_maps: {:#?}", after_dependency_maps);
                            after_dependency_maps
                        })
                        .fold(
                            (vec![], vec![]) as (Vec<Range<u64>>, Vec<Range<u64>>),
                            |(mut u, mut m), ele| {
                                match ele {
                                    (None, None) => (),
                                    (None, Some(r)) => m.push(r),
                                    (Some(r), None) => u.push(r),
                                    (Some(r), Some(s)) => {
                                        u.push(r);
                                        m.push(s);
                                    }
                                }
                                (u, m)
                            },
                        );

                    mapped.extend(newly_mapped.0);
                    mapped.extend(newly_mapped.1);
                    (mapped, vec![])
                })
                .0
        })
        .filter_map(|rs| rs.iter().map(|r| r.start).min())
        .min();

    mapping
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u64 {
    let instructions = parse_input(lines, |l| {
        l.split(" ")
            .filter_map(|n| n.parse::<u64>().ok())
            .map(|seed| seed..seed)
            .collect()
    });
    // should return 31599214
    find_min_mapping(instructions).unwrap()
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u64 {
    let instructions = parse_input(lines, |l| {
        let x = l.split(" ")
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .map(|chunk| {
                let start = *chunk.first().unwrap();
                start..(start + chunk.last().unwrap())
            })
            .collect::<Vec<_>>();
        x
    });
    // println!("{:#?}", instructions);
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

    #[test]
    fn map_range_fully_contained() {
        let dep = DependencyMapEntry {
            source: 0..10,
            destination: 20,
        };
        let result = dep.map_range(3..4);
        assert_eq!(result, (None, Some(23..24)));
    }

    #[test]
    fn map_range_fully_contained2() {
        let dep = DependencyMapEntry {
            source: 2..10,
            destination: 20,
        };
        let result = dep.map_range(3..4);
        assert_eq!(result, (None, Some(21..22)));
    }

    #[test]
    fn map_range_partially_contained_below() {
        let dep = DependencyMapEntry {
            source: 10..20,
            destination: 30,
        };
        let result = dep.map_range(5..15);
        assert_eq!(result, (Some(5..10), Some(30..35)));
    }

    #[test]
    fn map_range_partially_contained_above() {
        let dep = DependencyMapEntry {
            source: 10..20,
            destination: 40,
        };
        let result = dep.map_range(15..25);
        assert_eq!(result, (Some(20..25), Some(45..50)));
    }

    #[test]
    fn map_range_not_contained() {
        let dep = DependencyMapEntry {
            source: 0..10,
            destination: 20,
        };
        let result = dep.map_range(30..34);
        assert_eq!(result, (Some(30..34), None));
    }

    #[test]
    fn map_contained_1() {
        let dep = DependencyMapEntry {
            source: 0..10,
            destination: 20,
        };
        let result = dep.map(3);
        assert_eq!(result, Some(23));
    }

    #[test]
    fn map_contained_2() {
        let dep = DependencyMapEntry {
            source: 2..4,
            destination: 20,
        };
        let result = dep.map(3);
        assert_eq!(result, Some(21));
    }
}
