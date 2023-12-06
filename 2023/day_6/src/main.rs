pub fn main() {
    let filename = "day_6/src/input.txt";
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

fn parse_p1(lines: &Vec<impl AsRef<str>>) -> Vec<(u32, u32)> {
    let times = lines[0].as_ref().split(" ").filter_map(|t| t.parse::<u32>().ok());
    let distances = lines[1].as_ref().split(" ").filter_map(|t| t.parse::<u32>().ok());
    times.zip(distances).collect()
}

fn part1(lines: &Vec<impl AsRef<str>>) -> usize {
    let races = parse_p1(lines);
    println!("{:?}", races);
    races.iter().map(|(time, distance)| {
        (1..*time).map(|t| t * (time -t)).filter(|d| d > distance).collect::<Vec<_>>().len()
    }).product()
}
}

fn part2(_lines: &Vec<impl AsRef<str>>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 2] = ["Time:      7  15   30", "Distance:  9  40  200"];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 288);
    }
}
