use std::vec;

fn main() {
    let filename = "day_10/src/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let part1_result = part1(lines.clone());
    println!("Instructions Part1: {} Total Frequency", part1_result);

    let part2_result = part2(lines.clone());
    println!("Part2: \n{}", part2_result.join("\n"));
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> i32 {
    let mut total_frequency = 0;
    let instructions = lines
        .iter()
        .map(|l| Instruction::from(l.as_ref()))
        .collect::<Vec<Instruction>>();

    run_instructions(instructions, |cycle: i32, register: i32| {
        if (cycle - 20) % 40 == 0 {
            total_frequency += register * cycle;
        }
    });
    total_frequency
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> Vec<String> {
    let mut screen_line: Vec<String> = vec![];
    let mut screen: Vec<String> = vec![];
    let instructions = lines
        .iter()
        .map(|l| Instruction::from(l.as_ref()))
        .collect::<Vec<Instruction>>();

    run_instructions(instructions, |cycle: i32, register: i32| {
        let sprite_position = register % 40;
        let sprite_range = sprite_position - 1..=sprite_position + 1;
        // cycle - 1 because the first cycle is 1 but the first position is 0
        if sprite_range.contains(&((cycle - 1) % 40)) {
            screen_line.push("#".to_string());
        } else {
            screen_line.push(" ".to_string());
        }
        if screen_line.len() == 40 {
            screen.push(screen_line.join(""));
            screen_line = vec![];
        }
    });
    screen
}

fn run_instructions<F>(mut instructions: Vec<Instruction>, mut cycle_callback: F)
where
    F: FnMut(i32, i32) -> (),
{
    let mut cycle = 0;
    let mut register = 1;
    instructions.iter_mut().for_each(|instruction| loop {
        cycle += 1;
        cycle_callback(cycle, register);
        match instruction.execute(register) {
            Some(result) => {
                register = result;
                break;
            }
            None => (),
        }
    });
}

struct Instruction {
    operation: Operation,
    cycles: u32,
}

enum Operation {
    Add(i32),
    Noop,
}

impl Instruction {
    fn from(str: &str) -> Instruction {
        match str.split(" ").collect::<Vec<&str>>()[..] {
            ["addx", register] => Instruction {
                operation: Operation::Add(register.parse::<i32>().unwrap()),
                cycles: 2,
            },
            ["noop"] => Instruction {
                operation: Operation::Noop,
                cycles: 1,
            },
            _ => panic!("Unknown instruction: {}", str),
        }
    }

    fn execute(&mut self, register: i32) -> Option<i32> {
        return match self.operation {
            Operation::Add(value) => {
                if self.cycles == 2 {
                    // println!("Start of Operation: Add({})", value)
                }
                self.cycles -= 1;
                if self.cycles == 0 {
                    return Some(register + value);
                }
                None
            }
            Operation::Noop => {
                // println!("Start of Operation: Noop");
                self.cycles -= 1;
                if self.cycles == 0 {
                    return Some(register);
                }
                None
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        let filename = "src/example.txt";
        match fileutils::safe_lines_from_file(filename) {
            Some(lines) => lines,
            _ => fileutils::lines_from_file("day_10/".to_string() + filename),
        }
    }

    #[test]
    fn example_case_part1() {
        let result = part1(example_input());
        assert_eq!(result, 13140);
    }

    #[test]
    fn example_case_part2() {
        // Cannot be tested as it draws letters on the command line
        let result = part2(example_input());
        assert_eq!(
            r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string(),
            result.join("\n")
        )
    }
}
