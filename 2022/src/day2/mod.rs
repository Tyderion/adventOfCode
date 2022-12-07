use crate::fileutils;
const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;
const LOSS_POINTS: i32 = 0;

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

fn compute_results(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|pair| pair.split(' ').map(to_points).collect::<Vec<i32>>())
        .fold(0, |result, current_pair| match current_pair.as_slice() {
            [theirs, mine] if theirs == mine => result + mine + DRAW_POINTS,
            [theirs, mine] if (theirs - 1) % 3 == mine % 3 => result + mine + LOSS_POINTS,
            [theirs, mine] if (theirs + 1) % 3 == mine % 3 => result + mine + WIN_POINTS,
            _ => result,
        })
}

fn convert_part1(lines: Vec<String>) -> Vec<String> {
    const MY_ROCK: &str = "X";
    const MY_PAPER: &str = "Y";
    const MY_SCISSORS: &str = "Z";
    lines
        .iter()
        .map(|l| {
            l.replace(MY_ROCK, ROCK)
                .replace(MY_PAPER, PAPER)
                .replace(MY_SCISSORS, SCISSORS)
        })
        .collect()
}

fn to_points(value: &str) -> i32 {
    match value {
        ROCK => ROCK_POINTS,
        PAPER => PAPER_POINTS,
        SCISSORS => SCISSORS_POINTS,
        _ => 0,
    }
}

fn from_points(value: i32) -> Option<String> {
    match value {
        ROCK_POINTS => Some(String::from(ROCK)),
        PAPER_POINTS => Some(String::from(PAPER)),
        SCISSORS_POINTS | 0 => Some(String::from(SCISSORS)),
        _ => None,
    }
}

fn convert_part2(lines: Vec<String>) -> Vec<String> {
    const LOSS: &str = "X";
    const DRAW: &str = "Y";
    const WIN: &str = "Z";

    lines.iter().fold(vec![], |mut result, current_pair| {
        let new_pair = match current_pair.split(' ').collect::<Vec<&str>>().as_slice() {
            [theirs, result] => match result {
                &LOSS => Some([
                    String::from(*theirs),
                    from_points((to_points(theirs) - 1) % 3).unwrap(),
                ]),
                &DRAW => Some([String::from(*theirs), String::from(*theirs)]),
                &WIN => Some([
                    String::from(*theirs),
                    from_points((to_points(theirs) + 1) % 3).unwrap(),
                ]),

                _ => None,
            },
            _ => None,
        };

        match new_pair {
            Some(selections) => result.push(selections.join(" ")),
            None => (),
        };

        result
    })
}

fn part1(lines: Vec<String>) -> i32 {
    compute_results(convert_part1(lines))
}

fn part2(lines: Vec<String>) -> i32 {
    compute_results(convert_part2(lines))
}

#[allow(dead_code)]
pub fn run() {
    let filename = "./src/day2/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let result_part1 = part1(lines.clone());
    println!("Result: {:?} Points", result_part1);

    let result_part2 = part2(lines);
    println!("Result Part 2: {:?} Points", result_part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps_part1() {
        let result = part1(["A Y", "B X", "C Z"].map(String::from).to_vec());
        assert_eq!(result, 15);
    }

    #[test]
    fn rps_part2() {
        let result = part2(["A Y", "B X", "C Z"].map(String::from).to_vec());
        assert_eq!(result, 12);
    }

    #[test]
    fn test_convert2_a_y() {
        let result = convert_part2(["A Y"].map(String::from).to_vec());
        assert_eq!(result, ["A A"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_a_x() {
        let result = convert_part2(["A X"].map(String::from).to_vec());
        assert_eq!(result, ["A C"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_a_z() {
        let result = convert_part2(["A Z"].map(String::from).to_vec());
        assert_eq!(result, ["A B"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_b_x() {
        let result = convert_part2(["B X"].map(String::from).to_vec());
        assert_eq!(result, ["B A"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_b_y() {
        let result = convert_part2(["B Y"].map(String::from).to_vec());
        assert_eq!(result, ["B B"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_b_z() {
        let result = convert_part2(["B Z"].map(String::from).to_vec());
        assert_eq!(result, ["B C"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_c_z() {
        let result = convert_part2(["C Z"].map(String::from).to_vec());
        assert_eq!(result, ["C A"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_c_y() {
        let result = convert_part2(["C Y"].map(String::from).to_vec());
        assert_eq!(result, ["C C"].map(String::from).to_vec());
    }

    #[test]
    fn test_convert2_c_x() {
        let result = convert_part2(["C X"].map(String::from).to_vec());
        assert_eq!(result, ["C B"].map(String::from).to_vec());
    }

    #[test]
    fn test_scissors_paper() {
        let result = compute_results([format!("{SCISSORS} {PAPER}")].map(String::from).to_vec());
        assert_eq!(result, PAPER_POINTS + LOSS_POINTS);
    }

    #[test]
    fn test_scissors_scissors() {
        let result = compute_results(
            [format!("{SCISSORS} {SCISSORS}")]
                .map(String::from)
                .to_vec(),
        );
        assert_eq!(result, SCISSORS_POINTS + DRAW_POINTS);
    }
    #[test]
    fn test_scissors_rock() {
        let result = compute_results([format!("{SCISSORS} {ROCK}")].map(String::from).to_vec());
        assert_eq!(result, ROCK_POINTS + WIN_POINTS);
    }

    #[test]
    fn test_paper_scissors() {
        let result = compute_results([format!("{PAPER} {SCISSORS}")].map(String::from).to_vec());
        assert_eq!(result, SCISSORS_POINTS + WIN_POINTS);
    }

    #[test]
    fn test_rock_scissors() {
        let result = compute_results([format!("{ROCK} {SCISSORS}")].map(String::from).to_vec());
        assert_eq!(result, SCISSORS_POINTS + LOSS_POINTS);
    }
}
