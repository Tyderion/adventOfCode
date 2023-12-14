pub fn main() {
    execute::load_and_execute("day_6/src/input.txt", part1, part2);
}

fn parse_p1(lines: &Vec<impl AsRef<str>>) -> Vec<(u64, u64)> {
    let times = lines[0]
        .as_ref()
        .split(" ")
        .filter_map(|t| t.parse::<u64>().ok());
    let distances = lines[1]
        .as_ref()
        .split(" ")
        .filter_map(|t| t.parse::<u64>().ok());
    times.zip(distances).collect()
}

fn compute_options(time: u64, distance: u64) -> u64 {
    let find_valid_chargin_time = |t: u64| -> Option<u64> {
        if t * (time - t) > distance {
            Some(t)
        } else {
            None
        }
    };

    let min_time = (1..time).find_map(find_valid_chargin_time).unwrap();
    let max_time = (1..time).rev().find_map(find_valid_chargin_time).unwrap();

    max_time - min_time + 1
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u64 {
    let races = parse_p1(lines);
    races
        .iter()
        .map(|(time, distance)| compute_options(*time, *distance))
        .product()
}

fn parse_p2(lines: &Vec<impl AsRef<str>>) -> (u64, u64) {
    let time = lines[0]
        .as_ref()
        .replace(" ", "")
        .split(":")
        .filter_map(|t| t.parse::<u64>().ok())
        .sum();

    let distance = lines[1]
        .as_ref()
        .replace(" ", "")
        .to_string()
        .split(":")
        .filter_map(|t| t.parse::<u64>().ok())
        .sum();

    (time, distance)
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u64 {
    let (time, distance) = parse_p2(lines);
    compute_options(time, distance)
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

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 71503);
    }
}
