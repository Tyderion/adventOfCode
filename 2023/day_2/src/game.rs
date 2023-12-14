use crate::pull::Pull;

fn parse_game_id(str: impl AsRef<str>) -> Option<u32> {
    return match str.as_ref().split(" ").collect::<Vec<_>>()[..] {
        [_, num] => num.parse::<u32>().ok(),
        _ => None,
    };
}

fn parse_pulls(str: impl AsRef<str>) -> Vec<Pull> {
    return str.as_ref().split(";").map(Pull::from).collect::<Vec<_>>();
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub pulls: Vec<Pull>,
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