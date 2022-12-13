use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub}, collections::HashSet,
};

fn main() {
    let filename = "day_9/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Planck Rope Part 1: {} Positions", part1_result);

    let part2_result = part2(lines);
    println!("Part2 max viewable trees: {}", part2_result);
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> usize {
    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);
    let mut tail_positions = HashSet::new();
    tail_positions.insert(tail);

    lines.iter().map(|str| Move::from(str.as_ref())).for_each(|move1| {
        (head, tail ) = move1.execute_move(head, tail, &mut tail_positions);
    });
    tail_positions.len() as usize
}

fn part2<T: AsRef<str>>(_lines: Vec<T>) -> u32 {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move(Direction, i32);



impl Move {
    fn from(str: &str) -> Move {
        match str.split(" ").collect::<Vec<&str>>()[..] {
            [direction, distance] => Move(
                Direction::from(direction.chars().next().unwrap()),
                distance.parse().unwrap(),
            ),
            _ => panic!("Invalid line: {}", str),
        }
    }

    fn spread(&self) -> Vec<Direction> {
        (0..self.1).into_iter().map(|_| self.0).collect()
    }

    fn execute_move(&self, mut head: Position, mut tail: Position, tail_positions: &mut HashSet<Position>) -> (Position, Position) {
        self.spread().into_iter().for_each(|direction| {
            head += direction;
            tail += head - tail;
            tail_positions.insert(tail);
        });
        (head, tail)
    }
}

impl Direction {
    fn from(char: char) -> Direction {
        match char {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Invalid direction: {}", char),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Right => write!(f, "R"),
            Direction::Left => write!(f, "L"),
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        match other {
            // 2 Steps in the same direction moves in a straight line
            Position(0, 2) => Position(self.0, self.1 + 1),
            Position(0, -2) => Position(self.0, self.1 - 1),
            Position(2, 0) => Position(self.0 + 1, self.1),
            Position(-2, 0) => Position(self.0 - 1, self.1),
            // not touching moves diagonally
            Position(2, 1) | Position(1, 2) => Position(self.0 + 1, self.1 + 1),
            Position(2, -1) | Position(1, -2)  => Position(self.0 + 1, self.1 - 1),
            Position(-2, 1) | Position(-1, 2)  => Position(self.0 - 1, self.1 + 1),
            Position(-2, -1) | Position(-1, -2) => Position(self.0 - 1, self.1 - 1),
            _ => self
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position(self.0 + 1, self.1),
            Direction::Down => Position(self.0 - 1, self.1),
            Direction::Left => Position(self.0, self.1 + 1),
            Direction::Right => Position(self.0, self.1 - 1),
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static EXAMPLE: [&str; 8] = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];

    #[test]
    fn move_from() {
        let move1 = Move::from("R 4");
        assert_eq!(move1, Move(Direction::Right, 4));
    }

    #[test_case(Direction::Up, Position(1, 0); "Up")]
    #[test_case(Direction::Down, Position(-1, 0); "Down")]
    #[test_case(Direction::Left, Position(0, 1); "Left")]
    #[test_case(Direction::Right, Position(0, -1); "Right")]
    fn add_direction(direction: Direction, expected: Position) {
        let position = Position(0, 0);
        let new_position = position + direction;
        assert_eq!(new_position, expected);
    }

    #[test]
    fn example_case_part1() {
        let result = part1(EXAMPLE.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 13);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(EXAMPLE.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 0);
    }
}
