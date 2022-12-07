use crate::fileutils;
const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;
const LOSS_POINTS: i32 = 0;

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";
const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

fn compute_rps_results_by_pair(lines: Vec<String>) -> Vec<i32> {
    lines.iter().fold(vec![], |mut result, current_pair| {

        let current_score = match current_pair.split(' ').collect::<Vec<&str>>().as_slice() {
            [ROCK, MY_ROCK] => Some(DRAW_POINTS + ROCK_POINTS),
            [ROCK, MY_PAPER] => Some(WIN_POINTS + PAPER_POINTS),
            [ROCK, MY_SCISSORS] => Some(LOSS_POINTS + SCISSORS_POINTS),
            [PAPER, MY_ROCK] => Some(LOSS_POINTS + ROCK_POINTS),
            [PAPER, MY_PAPER] =>Some(DRAW_POINTS + PAPER_POINTS),
            [PAPER, MY_SCISSORS] => Some(WIN_POINTS + SCISSORS_POINTS),
            [SCISSORS, MY_ROCK] => Some(WIN_POINTS + ROCK_POINTS),
            [SCISSORS, MY_PAPER] => Some(LOSS_POINTS + PAPER_POINTS),
            [SCISSORS, MY_SCISSORS] => Some(DRAW_POINTS + SCISSORS_POINTS),
            _ => None
        };

        match current_score {
            Some(score) => result.push(score),
            None => (),
        }

        result
    })
}

fn compute_rps_results_by_pair_part2(lines: Vec<String>) -> Vec<String> {

    const LOSS: &str = "X";
    const DRAW: &str = "Y";
    const WIN: &str = "Z";

    lines.iter().fold(vec![], |mut result, current_pair| {
        let new_pair = match current_pair.split(' ').collect::<Vec<&str>>().as_slice() {
            [ROCK, LOSS] => Some(format!("{ROCK} {MY_SCISSORS}")),
            [ROCK, DRAW] => Some(format!("{ROCK} {MY_ROCK}")),
            [ROCK, WIN] => Some(format!("{ROCK} {MY_PAPER}")),
            [PAPER, LOSS] => Some(format!("{PAPER} {MY_ROCK}")),
            [PAPER, DRAW] => Some(format!("{PAPER} {MY_PAPER}")),
            [PAPER, WIN] => Some(format!("{PAPER} {MY_SCISSORS}")),
            [SCISSORS, LOSS] => Some(format!("{SCISSORS} {MY_PAPER}")),
            [SCISSORS, DRAW] => Some(format!("{SCISSORS} {MY_SCISSORS}")),
            [SCISSORS, WIN] => Some(format!("{SCISSORS} {MY_ROCK}")),
            _ => None
        };

        match new_pair {
            Some(score) => result.push(String::from(score)),
            None => (),
        }

        result
    })
}


fn compute_rps_results(by_line: Vec<i32>) -> i32 {
    by_line.iter().sum()
}

fn part1(lines: Vec<String>) -> i32 {
    let by_pair = compute_rps_results_by_pair(lines);
    compute_rps_results(by_pair)
}

fn part2(lines: Vec<String>) -> i32 {
    let converted = compute_rps_results_by_pair_part2(lines);
    compute_rps_results(compute_rps_results_by_pair(converted))
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
    fn rps_results_by_line_are_computed_correctly() {
        let result = compute_rps_results_by_pair(["A Y", "B X", "C Z"].map(String::from).to_vec());
        assert_eq!(result, vec![8, 1, 6]);
    }

    #[test]
    fn rps_results_are_computed_correctly() {
        let result = compute_rps_results(vec![8, 1, 6]);
        assert_eq!(result, 15);
    }

    #[test]
    fn rps_results_by_line_are_computed_correctly_part2() {
        let result =
            compute_rps_results_by_pair_part2(["A Y", "B X", "C Z"].map(String::from).to_vec());
        assert_eq!(result, ["A X", "B X", "C X"].map(String::from).to_vec());
    }
}
