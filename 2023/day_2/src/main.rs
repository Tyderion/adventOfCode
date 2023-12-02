pub fn main() {
    let filename = "day_2/src/input.txt";
    let input = fileutils::safe_lines_from_file(filename);
    let part1_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part1(lines),
    };
    let _part2_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part2(lines),
    };
    println!("Sum of games: {}", part1_result);
    // println!("Sum of part 2: {}", part2_result);
}

#[derive(Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pull {
    pub fn default() -> Pull {
        return Pull {
            red: 0,
            green: 0,
            blue: 0,
        };
    }

    fn red(&self, red: u32) -> Pull {
        Pull {
            red: red,
            green: self.green,
            blue: self.blue,
        }
    }
    fn green(&self, green: u32) -> Pull {
        Pull {
            red: self.red,
            green: green,
            blue: self.blue,
        }
    }
    fn blue(&self, blue: u32) -> Pull {
        Pull {
            red: self.red,
            green: self.green,
            blue: blue,
        }
    }

    /*  1 red, 2 green, 6 blue */
    pub fn from(str: impl AsRef<str>) -> Pull {
        return str
            .as_ref()
            .trim()
            .split(",")
            .fold(Pull::default(), |pull, single_color| {
                match single_color.trim().split(" ").collect::<Vec<_>>()[..] {
                    [num, "red"] => pull.red(num.parse::<u32>().unwrap()),
                    [num, "green"] => pull.green(num.parse::<u32>().unwrap()),
                    [num, "blue"] => pull.blue(num.parse::<u32>().unwrap()),
                    _ => panic!("There should be no other option"),
                }
            });
    }

    pub fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        return self.red <= red && self.green <= green && self.blue <= blue;
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

fn parse_game_id(str: impl AsRef<str>) -> Option<u32> {
    return match str.as_ref().split(" ").collect::<Vec<_>>()[..] {
        [_, num] => num.parse::<u32>().ok(),
        _ => None,
    };
}

fn parse_pulls(str: impl AsRef<str>) -> Vec<Pull> {
    return str.as_ref().split(";").map(Pull::from).collect::<Vec<_>>();
}

impl Game {
    pub fn from(str: impl AsRef<str>) -> Option<Game> {
        return match str.as_ref().split(":").collect::<Vec<_>>()[..] {
            [game, data] => match parse_game_id(game) {
                Some(id) => Some(Game {
                    id,
                    pulls: parse_pulls(data),
                }),
                None => None,
            },
            _ => None,
        };
    }
}

fn get_games(lines: &Vec<String>) -> Vec<Game> {
    return lines.iter().map(Game::from).filter_map(|v| v).collect();
}

fn part1(lines: &Vec<String>) -> u32 {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;
    return get_games(lines)
        .iter()
        .filter(|g| {
            g.pulls
                .iter()
                .all(|p| p.is_valid(RED_LIMIT, GREEN_LIMIT, BLUE_LIMIT))
        })
        .map(|g| g.id)
        .sum();
}

fn part2(_lines: &Vec<String>) -> u32 {
    return 0;
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
}
