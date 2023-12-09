use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Config {
    url: String,
    day: u8,
    year: u16,
}

pub fn parse_day_config(input: &str) ->   Result<Config, String> {
    lazy_static! {
        
        static ref RE: Regex =
            Regex::new(r"^https:\/\/adventofcode.com\/(?<year>[0-9]{4})\/day\/(?<day>[0-9]{1,2})\/input$")
                .unwrap();
    }

    if let Some(captures) = RE.captures(input) {
        Ok(Config {
            url: input.to_string(),
            day: captures
                .name("day")
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap(),
            year: captures
                .name("year")
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap(),
        })
    } else {
        Err(format!("Not a valid adventofcode input url {}", input.to_string()))
    }
}


pub fn start_day(config: Config) {
    println!("Starting new day {:?}", config);
}
