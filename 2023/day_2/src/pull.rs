#[derive(Debug)]
pub struct Pull {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
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
            red,
            green: self.green,
            blue: self.blue,
        }
    }
    fn green(&self, green: u32) -> Pull {
        Pull {
            red: self.red,
            green,
            blue: self.blue,
        }
    }
    fn blue(&self, blue: u32) -> Pull {
        Pull {
            red: self.red,
            green: self.green,
            blue,
        }
    }

    /*  1 red, 2 green, 6 blue */
    pub fn from(str: impl AsRef<str>) -> Pull {
        return str
            .as_ref()
            .trim()
            .split(",")
            .fold(Pull::default(), |acc, ele| {
                match ele.trim().split(" ").collect::<Vec<_>>()[..] {
                    [num, "red"] => acc.red(num.parse::<u32>().unwrap()),
                    [num, "green"] => acc.green(num.parse::<u32>().unwrap()),
                    [num, "blue"] => acc.blue(num.parse::<u32>().unwrap()),
                    _ => panic!("There should be no other option"),
                }
            });
    }
}
