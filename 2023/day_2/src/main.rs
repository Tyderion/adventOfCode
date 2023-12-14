mod game;
mod pull;

use std::cmp;

use game::Game;

pub fn main() {
    execute::load_and_execute( "day_2/src/input.txt", part1, part2);
}

fn get_games(lines: &Vec<String>) -> Vec<Game> {
    return lines.iter().map(Game::from).filter_map(|v| v).collect();
}

fn part1(lines: &Vec<String>) -> u32 {
    return get_games(lines)
        .iter()
        .filter(|g| {
            g.pulls
                .iter()
                .all(|pull| pull.red <= 12 && pull.green <= 13 && pull.blue <= 14)
        })
        .map(|g| g.id)
        .sum();
}

fn part2(lines: &Vec<String>) -> u32 {
    return get_games(lines)
        .iter()
        .map(|g| {
            g.pulls.iter().fold([0u32; 3], |acc, ele| {
                [
                    cmp::max(acc[0], ele.red),
                    cmp::max(acc[1], ele.green),
                    cmp::max(acc[2], ele.blue),
                ]
            })
        })
        .map(|p| p.iter().product::<u32>())
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 5] = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 8);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 2286);
    }
}
