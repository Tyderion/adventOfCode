use std::{collections::VecDeque, fmt::Display, panic, vec};

fn main() {
    let filename = "day_7/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let root = get_root_directory(lines);

    let part1_result = part1(root.clone());
    println!("Part1 Total Size {}", part1_result);

    let part2_result = part2(root);
    println!("Part2 Directory to delete size: {}", part2_result);
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
            "\n{}- {} (file, size={})",
            "  ".repeat(level),
            self.name,
            self.size
        )
    }
}

impl Directory {
    fn print(&self, level: usize) -> String {
        let indent = "  ".repeat(level);
        let dirs = match self.subdirs.len() {
            0 => "".to_string(),
            _ => format!(
                "{}",
                self.subdirs
                    .iter()
                    .map(|child| child.print(level + 1))
                    .collect::<Vec<String>>()
                    .join(&indent)
            ),
        };
        let children = match self.children.len() {
            0 => "".to_string(),
            _ => format!(
                "{}",
                self.children
                    .iter()
                    .map(|child| child.print(level + 1))
                    .collect::<Vec<String>>()
                    .join(&indent)
            ),
        };
        format!("\n{}- {} (dir){}{}", indent, self.name, children, dirs)
    }

    fn size(&self) -> u32 {
        self.children
            .iter()
            .map(|f| f.size)
            .chain(self.subdirs.iter().map(|d| d.size()))
            .sum::<u32>()
    }

    fn collect_subdirs(&self) -> Vec<Directory> {
        let mut child_dirs = self
            .subdirs
            .iter()
            .flat_map(|d| d.collect_subdirs())
            .collect::<Vec<Directory>>();
        child_dirs.push(self.clone());
        child_dirs
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print(0))
    }
}

fn get_root_directory(mut lines: Vec<String>) -> Directory {
    let first = lines.remove(0);
    if first.ne("$ cd /") {
        panic!("Does not start with /");
    }
    let mut root: Directory = Directory {
        name: "/".to_string(),
        children: vec![],
        subdirs: vec![],
    };
    read_directory_contents(
        VecDeque::from_iter(lines.iter().map(|s| s.as_str())),
        &mut root,
    );
    root
}

fn read_directory_contents<'a>(
    mut lines: VecDeque<&'a str>,
    current_dir: &mut Directory,
) -> VecDeque<&'a str> {
    while let Some(line) = lines.pop_front() {
        if line.starts_with("$ cd ..") {
            return lines.clone();
        }
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }
        if let ["$", "cd", name] = line.split(" ").collect::<Vec<&str>>().as_slice() {
            let mut dir = Directory {
                name: name.to_string(),
                children: vec![],
                subdirs: vec![],
            };
            lines = read_directory_contents(lines.clone(), &mut dir);
            current_dir.subdirs.push(dir);
        }
        if let [size, name] = line.split(" ").collect::<Vec<&str>>().as_slice() {
            current_dir.children.push(File {
                name: name.to_string(),
                size: size.parse().unwrap(),
            });
        }
    }
    lines.clone()
}

fn part1(root: Directory) -> u32 {
    root.collect_subdirs()
        .iter()
        .map(|d| d.size())
        .filter(|s| *s < 100000 as u32)
        .sum::<u32>()
}

fn part2(root: Directory) -> u32 {
    const AVAILABLE_SPACE: u32 = 70000000 as u32;
    const NEEDED_SPACE: u32 = 30000000 as u32;

    let used = root.size();
    let to_delete = NEEDED_SPACE - (AVAILABLE_SPACE - used);
    let mut directories_big_enough = root
        .collect_subdirs()
        .iter()
        .map(|d| d.size())
        .filter(|s| *s > to_delete)
        .collect::<Vec<u32>>();
    directories_big_enough.sort();
    *directories_big_enough.first().unwrap()
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
        let result = part1(get_root_directory(
            EXAMPLE.iter().map(|l| String::from(*l)).collect(),
        ));
        assert_eq!(result, 95437);
    }

    #[test]
    fn print() {
        let mut root = Directory {
            subdirs: vec![],
            children: vec![],
            name: "/".to_string(),
        };
        root.children.push(File {
            name: "a.txt".to_string(),
            size: 111,
        });
        root.children.push(File {
            name: "b.txt".to_string(),
            size: 222,
        });

        let mut subdir = Directory {
            subdirs: vec![],
            children: vec![],
            name: "dirname".to_string(),
        };
        subdir.children.push(File {
            name: "c.txt".to_string(),
            size: 333,
        });

        root.subdirs.push(subdir);

        println!("{}", root.print(0));
        assert_eq!(
            r"
- / (dir)
  - a.txt (file, size=111)
  - b.txt (file, size=222)
  - dirname (dir)
    - c.txt (file, size=333)",
            root.print(0)
        );
    }

    #[test]
    fn example_cases_part2() {
        let result = part2(get_root_directory(
            EXAMPLE.iter().map(|l| String::from(*l)).collect(),
        ));
        assert_eq!(result, 24933642);
    }
}
