fn main() {
    let filename = "day_12/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());

    // let part2_result = part2(lines.clone());
    println!("Steps required to walk to target: {}", part1_result);
    // println!("part2 : {}", part2_result);
}


fn part1<T: AsRef<str>>(lines: Vec<T>) -> u32 {
   0
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: [&str; 5] = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn example_case_part1() {
        let result = part1(
            EXAMPLE
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(result, 31);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(
            EXAMPLE
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(result, 0);
    }
}
