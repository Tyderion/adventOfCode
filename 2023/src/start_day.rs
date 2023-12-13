use std::fs;
use toml_edit::Document;

use lazy_static::lazy_static;
use regex::Regex;

use std::process::Command;
#[derive(Debug, Clone)]
pub struct Config {
    url: String,
    day: u8,
    year: u16,
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

    fn root_toml(&self) -> String {
        "Cargo.toml".to_string()
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
            year: captures
                .name("year")
                .unwrap()
                .as_str()
                .parse::<u16>()
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
    let session_cookie =
        std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE must be set (.env supported)");
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
execute = {{ path = "../execute" }}
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
    execute::load_and_execute("day_{}/src/input.txt", part1, part2);
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

fn add_to_workspace(config: &Config) {
    let toml_content = fs::read_to_string(config.root_toml())
        .expect(format!("Toml file not found {}", config.root_toml()).as_str());

    let mut toml = toml_content.parse::<Document>().expect("invalid doc");
    let workspace_list = toml["workspace"]["members"].as_array_mut().unwrap();

    workspace_list.push(config.root_folder());
    workspace_list.fmt();

    match fs::write(config.root_toml(), toml.to_string()) {
        Ok(_) => println!("Successfully updated toml file {}", config.main_file()),
        Err(e) => eprintln!("Error while writing main toml file {}", e),
    }
}

fn create_branch(config: &Config) {
    let branch_name = format!("{}-day{}", config.year, config.day);

    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(&branch_name)
        .status()
        .expect(format!("Coudl not create new branch {}", &branch_name).as_str());

    Command::new("git")
        .arg("add")
        .arg(".")
        .status()
        .expect("git add failed");

    Command::new("git")
        .arg("commit")
        .arg(format!("-m Initial Setup Day {}", config.day).as_str())
        .status()
        .expect("git commit failed");
}

pub fn start_day(config: Config) {
    println!("Starting new day {:?}", config);
    create_directories(&config);
    store_input_file(&config);
    write_toml(&config);
    write_main(&config);
    add_to_workspace(&config);
    create_branch(&config);
    println!(
        "Run new workspace with 'cargo run -p {}'",
        config.root_folder()
    )
}
