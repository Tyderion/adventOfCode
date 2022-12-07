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

fn compute_results(lines: Vec<String>) -> i32 {
    lines.iter().fold(vec![], |mut result, current_pair| {

        let current_score = match current_pair.split(' ').collect::<Vec<&str>>().as_slice() {
            [ROCK, ROCK] => Some(DRAW_POINTS + ROCK_POINTS),
            [ROCK, PAPER] => Some(WIN_POINTS + PAPER_POINTS),
            [ROCK, SCISSORS] => Some(LOSS_POINTS + SCISSORS_POINTS),
            [PAPER, ROCK] => Some(LOSS_POINTS + ROCK_POINTS),
            [PAPER, PAPER] =>Some(DRAW_POINTS + PAPER_POINTS),
            [PAPER, SCISSORS] => Some(WIN_POINTS + SCISSORS_POINTS),
            [SCISSORS, ROCK] => Some(WIN_POINTS + ROCK_POINTS),
            [SCISSORS, PAPER] => Some(LOSS_POINTS + PAPER_POINTS),
            [SCISSORS, SCISSORS] => Some(DRAW_POINTS + SCISSORS_POINTS),
            _ => None
        };

        match current_score {
            Some(score) => result.push(score),
            None => (),
        }

        result
    }).iter().sum()
}

fn convert_part1(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|l| l.replace(MY_ROCK, ROCK).replace(MY_PAPER, PAPER).replace(MY_SCISSORS, SCISSORS)).collect()
}

fn convert_part2(lines: Vec<String>) -> Vec<String> {

    const LOSS: &str = "X";
    const DRAW: &str = "Y";
    const WIN: &str = "Z";

    lines.iter().fold(vec![], |mut result, current_pair| {
        let new_pair = match current_pair.split(' ').collect::<Vec<&str>>().as_slice() {
            [ROCK, LOSS] => Some(format!("{ROCK} {SCISSORS}")),
            [ROCK, DRAW] => Some(format!("{ROCK} {ROCK}")),
            [ROCK, WIN] => Some(format!("{ROCK} {PAPER}")),
            [PAPER, LOSS] => Some(format!("{PAPER} {ROCK}")),
            [PAPER, DRAW] => Some(format!("{PAPER} {PAPER}")),
            [PAPER, WIN] => Some(format!("{PAPER} {SCISSORS}")),
            [SCISSORS, LOSS] => Some(format!("{SCISSORS} {PAPER}")),
            [SCISSORS, DRAW] => Some(format!("{SCISSORS} {SCISSORS}")),
            [SCISSORS, WIN] => Some(format!("{SCISSORS} {ROCK}")),
            _ => None
        };

        match new_pair {
            Some(score) => result.push(String::from(score)),
            None => (),
        }

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
}
