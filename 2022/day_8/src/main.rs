use std::collections::HashMap;

fn main() {
    let filename = "day_8/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Part1 Total Size {}", part1_result);

    // let part2_result = part2(lines);
    // println!("Part2 Directory to delete size: {}", part2_result);
}

fn part1(lines: Vec<String>) -> usize {
    let mut map: HashMap<(usize, usize), (u8, bool, bool, bool, bool)> = HashMap::new();
    let max_row = lines.len();
    let max_col = lines[0].len();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map.insert(
                (row, col),
                (c.to_string().parse::<u8>().unwrap(), true, true, true, true),
            );
        }
    }

    for row in 1..max_row {
        for col in 1..max_col {
            let mut current_tree = **&mut map.get(&(row, col)).unwrap();
            // Check left
            for j in (0..col).rev() {
                let ele = map.get(&(row, j)).unwrap();
                if ele.0 >= current_tree.0 {
                    current_tree.1 = false;
                    break;
                }
            }
            // Check right
            for j in col+1..max_col {
                let ele = map.get(&(row, j)).unwrap();
                if ele.0 >= current_tree.0 {
                    current_tree.2 = false;
                    break;
                }
            }
            // Check top
            for i in (0..row).rev(){
                let ele =  map.get(&(i, col)).unwrap();
                if ele.0 >= current_tree.0 {
                    current_tree.3 = false;
                    break;
                }
            }
            // Check bottom
            for i in row + 1..max_col {
                let ele = map.get(&(i, col)).unwrap();
                if ele.0 >= current_tree.0 {
                    current_tree.4 = false;
                    break;
                }
            }
            map.insert((row, col), current_tree);
        }
    }
    let visible_trees = map.iter().filter(|(_, v)| v.1 || v.2 || v.3 || v.4)
    .map(|((row, col), tree)| ((*row, *col), tree.0))
    .collect::<Vec<((usize, usize), u8)>>();
    visible_trees.iter().count()
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE: [&str; 5] = ["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn example_case_part1() {
        let result = part1(EXAMPLE.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 21);
    }
}
