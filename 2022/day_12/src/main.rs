use lazy_static::lazy_static;
use pathfinding::prelude::bfs;
use std::collections::{HashSet, VecDeque};
fn main() {
    let part1_result = part1();
    println!("Steps required to walk to target: {}", part1_result);
    let part2_result = part2();
    println!("part2 : {}", part2_result);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(usize, usize);
lazy_static! {
    static ref PLAYING_DATA: (Vec<Vec<u8>>, VecDeque<Position>, Position, Position) =
        create_playing_field(fileutils::lines_from_file("day_12/src/input.txt"));
        // create_playing_field(fileutils::lines_from_file("day_12/src/example.txt"));
}

fn create_playing_field<T: AsRef<str>>(
    lines: Vec<T>,
) -> (Vec<Vec<u8>>, VecDeque<Position>, Position, Position) {
    let mut playing_field = Vec::new();

    let mut starting_positions: VecDeque<Position> = VecDeque::new();
    let mut end_position = Position(0, 0);
    let mut max_position = Position(0, 0);
    for (row, line) in lines.iter().enumerate() {
        let mut current_row = Vec::new();
        for (col, c) in line.as_ref().chars().enumerate() {
            match c {
                'S' => {
                    current_row.push('a' as u8);
                    starting_positions.push_front(Position(row, col));
                }
                'E' => {
                    current_row.push('z' as u8);
                    end_position = Position(row, col);
                }
                'a' => {
                    current_row.push('a' as u8);
                    starting_positions.push_back(Position(row, col));
                }
                _ => current_row.push(c as u8),
            }
            if (row > max_position.0) || (col > max_position.1) {
                max_position = Position(row, col);
            }
        }
        playing_field.push(current_row);
    }
    (
        playing_field,
        starting_positions,
        end_position,
        max_position,
    )
}

impl Position {
    fn height(&self) -> u8 {
        PLAYING_DATA
            .0
            .iter()
            .nth(self.0)
            .unwrap()
            .iter()
            .nth(self.1)
            .unwrap()
            .clone()
    }

    fn max() -> Position {
        PLAYING_DATA.3
    }

    fn viable_pos(&self, new_pos: Position) -> Option<Position> {
        if new_pos.0 > Self::max().0 || new_pos.1 > Self::max().1 {
            return None;
        }
        if new_pos.height() > self.height() + 1 {
            return None;
        }
        Some(new_pos)
    }

    fn left(&self) -> Option<Position> {
        if self.0 == 0 {
            return None;
        }
        self.viable_pos(Position(self.0 - 1, self.1))
    }

    fn right(&self) -> Option<Position> {
        self.viable_pos(Position(self.0 + 1, self.1))
    }

    fn top(&self) -> Option<Position> {
        if self.1 == 0 {
            return None;
        }
        self.viable_pos(Position(self.0, self.1 - 1))
    }

    fn bottom(&self) -> Option<Position> {
        self.viable_pos(Position(self.0, self.1 + 1))
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

fn part1() -> usize {
    let starting_pos = PLAYING_DATA.1.front().unwrap();
    let goal_pos = PLAYING_DATA.2;
    let result = bfs(starting_pos, |p| p.surrounding(), |p| *p == goal_pos);
    result.unwrap().len() - 1
}

fn part2() -> usize {
    let goal_pos = PLAYING_DATA.2;

    let mut shortest_path: usize = usize::MAX;
    for starting_position in PLAYING_DATA.1.iter() {
        let result = bfs(starting_position, |p| p.surrounding(), |p| *p == goal_pos);
        match result {
            Some(path) => {
                if path.len() < shortest_path {
                    shortest_path = path.len();
                }
            }
            None => (),
        }
    }
    shortest_path - 1
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    // No tests because we use static shit data
}
