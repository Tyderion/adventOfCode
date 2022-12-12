use std::{
    collections::{vec_deque, VecDeque},
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
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

fn recursive_walk(
    lines: Vec<String>,
    mut current_dir: Directory,
    dirs: &mut Vec<Directory>,
) -> u32 {
    match lines.as_slice() {
        [line, rest @ ..] => match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "ls"] => recursive_walk(rest.to_vec(), current_dir, dirs),
            ["$", "cd", name] => match name {
                &".." => {
                    dirs.push(current_dir);
                    0
                }
                name => {
                    let dir = Directory {
                        name: name.to_string(),
                        children: vec![],
                    };
                    dirs.push(current_dir);
                    recursive_walk(rest.to_vec(), dir, dirs)
                }
            },
            ["dir", _] => recursive_walk(rest.to_vec(), current_dir, dirs),
            [size, name] => {
                current_dir.children.push(File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                });
                recursive_walk(rest.to_vec(), current_dir, dirs)
            }
            _ => panic!("Invalid line"),
        },
        _ => 0,
    }
}

fn part1(mut lines: Vec<String>) -> u32 {
    let mut dirs: Vec<Directory> = Vec::new();
    let first = lines.remove(0);
    if first.ne("$ cd /") {
        panic!("Does not start with /");
    }
    let current_dir: Directory = Directory {
        name: "/".to_string(),
        children: vec![],
    };
    recursive_walk(lines, current_dir, &mut dirs);

    println!("{:?}", dirs);
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

    #[test]
    fn example_cases_part1() {
        let result = part1(EXAMPLE.iter().map(|l| String::from(*l)).collect());
        assert_eq!(result, 95437);
    }

    #[test]
    fn example_cases_part2() {
        let result = part2(EXAMPLE.to_vec());
        assert_eq!(result, todo!());
    }
}
