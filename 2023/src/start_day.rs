use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Config {
    url: String,
    day: u8,
}

impl Config {
    fn root_folder(&self) -> String {
        format!("day_{}", self.day)
    }

    fn src_folder(&self) -> String {
        format!("{}/src", self.root_folder())
    }

    fn input_file(&self) -> String {
        format!("{}/input.txt", self.src_folder())
    }

    fn toml_file(&self) -> String {
        format!("{}/Cargo.toml", self.root_folder())
    }

    fn main_file(&self) -> String {
        format!("{}/main.rs", self.src_folder())
    }
}

pub fn parse_day_config(input: &str) -> Result<Config, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^https:\/\/adventofcode.com\/(?<year>[0-9]{4})\/day\/(?<day>[0-9]{1,2})\/input$"
        )
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
        })
    } else {
        Err(format!(
            "Not a valid adventofcode input url {}",
            input.to_string()
        ))
    }
}

fn create_directories(config: &Config) {
    let path = config.src_folder();
    match fs::create_dir_all(&path) {
        Ok(_) => println!("Successfully created directories {}", &path),
        Err(error) => eprintln!("Error while creating directories: {}", error),
    }
}

fn download_input(config: &Config) -> Result<String, reqwest::Error> {
    let session_cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE must be set (.env supported)");
    let client = reqwest::blocking::Client::new();
    client
        .get(&config.url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()
}

fn store_input_file(config: &Config) {
    match download_input(config) {
        Ok(body) => match fs::write(config.input_file(), body) {
            Ok(_) => println!("Successfully created input file {}", config.input_file()),
            Err(e) => eprintln!("Error while writing input file {}", e),
        },
        Err(err) => {
            eprintln!("Error fetch result {}", err);
        }
    }
}

fn write_toml(config: &Config) {
    let content = format!(
        r#"
[package]
name = "day_{}"
version = "0.1.0"
edition = "2021"

[dependencies]
fileutils = {{ path = "../fileutils" }}
[dev-dependencies]
test-case = "3.3.1"
    "#,
        config.day
    )
    .trim()
    .to_string();

    match fs::write(config.toml_file(), content) {
        Ok(_) => println!("Successfully created toml file {}", config.toml_file()),
        Err(e) => eprintln!("Error while writing toml file {}", e),
    }
}

fn write_main(config: &Config) {
    let content = format!(
        r#"
    pub fn main() {{
        let filename = "day_{}/src/input.txt";
        let input = fileutils::safe_lines_from_file(filename);
        let part1_result = match input {{
            None => panic!("No input received"),
            Some(ref lines) => part1(lines),
        }};
        let part2_result = match input {{
            None => panic!("No input received"),
            Some(ref lines) => part2(lines),
        }};
        println!("Sum of games: {{}}", part1_result);
        println!("Sum of part 2: {{}}", part2_result);
    }}
    
    fn part1(_lines: &Vec<impl AsRef<str>>) -> u64 {{
        0
    }}
    
    fn part2(_lines: &Vec<impl AsRef<str>>) -> u64 {{
        0
    }}
    
    #[cfg(test)]
    mod tests {{
        use super::*;
    
        const EXAMPLE_INPUT1: [&str; 9] = [
            todo!()
        ];

    
        #[test]
        fn example_case_part1() {{
            let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
            assert_eq!(result, todo!());
        }}
    
        #[test]
        fn example_case_part2() {{
            let result = part2(&EXAMPLE_INPUT2.iter().map(|x| String::from(*x)).collect());
            assert_eq!(result, todo!());
        }}
    }}
    "#,
        config.day
    )
    .trim()
    .to_string();

    match fs::write(config.main_file(), content) {
        Ok(_) => println!("Successfully created main file {}", config.main_file()),
        Err(e) => eprintln!("Error while writing main file {}", e),
    }
}

pub fn start_day(config: Config) {
    println!("Starting new day {:?}", config);
    create_directories(&config);
    store_input_file(&config);
    write_toml(&config);
    write_main(&config);
}
