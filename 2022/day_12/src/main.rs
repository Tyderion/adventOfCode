use std::collections::{HashSet, VecDeque};
fn main() {
    let filename = "day_12/src/input.txt";
    let lines = fileutils::lines_from_file(filename);
    
    let part1_result = part1(lines.clone());
    // let part2_result = part2(lines.clone());
    println!("Steps required to walk to target: {}", part1_result);
    // println!("part2 : {}", part2_result);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn left(&self) -> Option<Position> {
        if self.0 == 0 {
            return None;
        }
        Some(Position(self.0 - 1, self.1))
    }

    fn right(&self) -> Option<Position> {
        Some(Position(self.0 + 1, self.1))
    }

    fn top(&self) -> Option<Position> {
        if self.1 == 0 {
            return None;
        }
        Some(Position(self.0, self.1 - 1))
    }

    fn bottom(&self) -> Option<Position> {
        Some(Position(self.0, self.1 + 1))
    }

    fn surrounding(&self) -> Vec<Position> {
        let mut result = Vec::new();
        if let Some(left) = self.left() {
            result.push(left);
        }
        if let Some(right) = self.right() {
            result.push(right);
        }
        if let Some(top) = self.top() {
            result.push(top);
        }
        if let Some(bottom) = self.bottom() {
            result.push(bottom);
        }
        result
    }
}

fn height_at_position(playing_field: &Vec<Vec<u8>>, position: Option<Position>) -> Option<u8> {
    if position.is_none()
        || position.unwrap().0 >= playing_field.len()
        || position.unwrap().1 >= playing_field[0].len()
    {
        return None;
    }
    Some(
        playing_field
            .iter()
            .nth(position.unwrap().0)
            .unwrap()
            .iter()
            .nth(position.unwrap().1)
            .unwrap()
            .clone(),
    )
}

fn breadth_first(
    end: Position,
    current_position: Position,
    playing_field: &Vec<Vec<u8>>,
    visited: HashSet<Position>,
) -> Option<Vec<Position>> {
    let current_height = height_at_position(playing_field, Some(current_position)).unwrap();
    let mut result = vec![];
    let mut new_visited = visited.clone();
    new_visited.insert(current_position);

    if current_position == end {
        return Some(vec![current_position]);
    }

    for new_position in current_position.surrounding() {
        if !visited.contains(&new_position) {
            if let Some(left_height) = height_at_position(playing_field, Some(new_position)) {
                // println!("checking position {:?}", new_position);
                if left_height <= current_height + 1 {
                    let mut vis = new_visited.clone();
                    vis.insert(current_position);
                    if let Some(new_path) = breadth_first(end, new_position, playing_field, vis) {
                        if result.is_empty() || result.len() > new_path.len() {
                            println!(
                                "Best route from {:?} to {:?} is {:?}",
                                end, current_position, new_path
                            );
                            result = new_path.clone();
                        }
                    }
                }
            }
        }
    }

    if result.len() != 0 {
        result.push(current_position);
        return Some(result);
    }

    None
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> u32 {
    let mut playing_field = Vec::new();
    let mut start_position = Position(0, 0);
    let mut end_position = Position(0, 0);
    for (row, line) in lines.iter().enumerate() {
        let mut current_row = Vec::new();
        for (col, c) in line.as_ref().chars().enumerate() {
            match c {
                'S' => {
                    current_row.push('a' as u8);
                    start_position = Position(row, col);
                }
                'E' => {
                    current_row.push('z' as u8);
                    end_position = Position(row, col);
                }
                _ => current_row.push(c as u8),
            }
        }
        playing_field.push(current_row);
    }
    let shortest_path = breadth_first(end_position, start_position, &playing_field, HashSet::new());

    println!("Shortest path: {:?}", shortest_path);
    // one less because we don't have to step to start position, but it's part of the path
    return shortest_path.unwrap().len() as u32 - 1;
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static EXAMPLE: [&str; 5] = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn test_simple() {
        let playing_field = vec![vec![0, 0, 1, 2, 3, 4, 5, 6]];
        if let Some(result) = breadth_first(
            Position(0, 7),
            Position(0, 0),
            &playing_field,
            HashSet::new(),
        ) {
            // println!("result: {:?}", result);
            assert_eq!(result.len() - 1, 7);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_simple2() {
        let playing_field = vec![
            vec![1, 2, 3, 6, 7, 8, 9, 10],
            vec![1, 2, 4, 5, 8, 10, 13, 14],
        ];
        if let Some(result) = breadth_first(
            Position(0, 7),
            Position(0, 0),
            &playing_field,
            HashSet::new(),
        ) {
            // println!("result: {:?}", result);
            assert_eq!(result.len() - 1, 9);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_breadth_first() {
        let playing_field = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        if let Some(result) = breadth_first(
            Position(2, 5),
            Position(0, 0),
            &playing_field,
            HashSet::new(),
        ) {
            println!("result: {:?}", result);
            assert_eq!(result.len() - 1, 31);
        } else {
            assert!(false);
        }
    }

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
