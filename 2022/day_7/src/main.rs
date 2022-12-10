fn main() {
    let filename = "day_7/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Part1 Total Size {}", part1_result);

    let part2_result = part2(lines);
    println!("Part2 ??: {}", part2_result);
}

fn part1<T: AsRef<str>>(_lines: Vec<T>) -> u32 {
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
        let result = part1(EXAMPLE.to_vec());
        assert_eq!(result, 95437);
    }

    #[test]
    fn example_cases_part2() {
        let result = part2(EXAMPLE.to_vec());
        assert_eq!(result, todo!());
    }
}
