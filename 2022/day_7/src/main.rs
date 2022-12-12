use std::{
    collections::{vec_deque, VecDeque},
    fmt::Display,
    panic,
    thread::current,
    vec,
};

fn main() {
    let filename = "day_7/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Part1 Total Size {}", part1_result);

    let part2_result = part2(lines);
    println!("Part2 ??: {}", part2_result);
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    children: Vec<File>,
    subdirs: Vec<Directory>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn print(&self, level: usize) -> String {
        format!(
            "{}- {} (file, size={})",
            " ".repeat(level),
            self.name,
            self.size
        )
    }
}

impl Directory {
    fn print(&self, level: usize) -> String {
        let indent = "  ".repeat(level + 1);
        let dirs = match self.subdirs.len() {
            0 => "".to_string(),
            _ => format!(
                "\n{}{}",
                indent,
                self.subdirs
                    .iter()
                    .map(|child| child.print(level + 1))
                    .collect::<Vec<String>>()
                    .join(&format!("\n{}", indent))
            ),
        };
        let children = match self.children.len() {
            0 => "".to_string(),
            _ => format!(
                "\n{}{}",
                indent,
                self.children
                    .iter()
                    .map(|child| child.print(level + 1))
                    .collect::<Vec<String>>()
                    .join(&format!("\n{}", indent))
            ),
        };
        format!("- {} (dir) {} {}", self.name, children, dirs)
    }
}

fn recursive_walk(lines: Vec<String>, current_dir: &mut Directory) -> usize {
    match lines.as_slice() {
        [line, rest @ ..] => match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "ls"] => recursive_walk(rest.to_vec(), current_dir) + 1,
            ["$", "cd", name] => match name {
                &".." => {
                    println!("going back up from {}", current_dir.name);
                    1
                }
                name => {
                    let mut dir = Directory {
                        name: name.to_string(),
                        children: vec![],
                        subdirs: vec![],
                    };
                    println!("In {}, Going down to {}", current_dir.name, dir.name);
                    let mut eaten = recursive_walk(rest.to_vec(), &mut dir);
                    loop {
                        eaten += recursive_walk(rest[eaten..].to_vec(), current_dir);
                        if eaten == rest.len() {
                            current_dir.subdirs.push(dir);
                            return eaten + 1;
                        }
                    }
                }
            },
            ["dir", _] => recursive_walk(rest.to_vec(), current_dir) + 1,
            [size, name] => {
                current_dir.children.push(File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                });
                println!("In {}, adding file {}", current_dir.name, name);
                1 + recursive_walk(rest.to_vec(), current_dir)
            }
            _ => panic!("Invalid line"),
        },
        _ => 0,
    }
}

fn part1(mut lines: Vec<String>) -> u32 {
    let first = lines.remove(0);
    if first.ne("$ cd /") {
        panic!("Does not start with /");
    }
    let mut root: Directory = Directory {
        name: "/".to_string(),
        children: vec![],
        subdirs: vec![],
    };
    recursive_walk(lines, &mut root);

    println!("{}", root.print(0));

    println!("{:?}", root);
    0
}

fn part2<T: AsRef<str>>(_lines: Vec<T>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE: [&str; 23] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    static EXAMPLE2: [&str; 6] = [
        "$ cd /",
        "$ cd a",
        "$ cd b",
        "222 b.txt",
        "$ cd ..",
        "333 a.txt",
    ];

    #[test]
    fn example_cases_part1() {
        let result = part1(EXAMPLE.iter().map(|l| String::from(*l)).collect());
        assert_eq!(result, 95437);
    }

    #[test]
    fn example_cases_part1_2() {
        let result = part1(EXAMPLE2.iter().map(|l| String::from(*l)).collect());
        assert_eq!(result, 0);
    }

    #[test]
    fn example_cases_part2() {
        let result = part2(EXAMPLE.to_vec());
        assert_eq!(result, todo!());
    }
}
