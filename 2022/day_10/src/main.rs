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
    let mut cycle = 1;
    let mut register = 1;
    let mut total_frequency = 0;
    lines.iter().for_each(|l| {
        let mut instruction = Instruction::from(l.as_ref());
        loop {
            if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 ||  cycle == 220{
                println!("register is {} at cycle {} totalling {}", register, cycle,  register * cycle);
                total_frequency += register * cycle;
            }
            let result = instruction.execute(register);
            cycle += 1;
            if result.is_none() {
                continue;
            }
            register = result.unwrap();
            break;
        }
        
    });
    println!("Finished after {} cycles with register {}", cycle, register);
    total_frequency
}

fn try_flush(current_line: &mut Vec<String>, screen: &mut Vec<String>) {
    if current_line.len() == 40 {
        screen.push(current_line.join(""));
        *current_line = vec![];
    }
}

fn part2<T: AsRef<str>>(lines: Vec<T>) -> Vec<String> {
    let mut cycle = 0;
    let mut register = 1;
    let mut current_screen: Vec<String> = vec![];
    let mut screen: Vec<String> = vec![];
    lines.iter().for_each(|l| {
        let mut instruction = Instruction::from(l.as_ref());
        loop {
            if cycle % 40 == 0 && current_screen.len() == 40 {
                println!("register is {} at cycle {}", register, cycle);
                screen.push(current_screen.join(""));
                current_screen = vec![];
            }
            let result = instruction.execute(register);
            let sprite_position = register % 40;
            if (sprite_position-1..=sprite_position+1).contains(&(cycle % 40)){
                current_screen.push("#".to_string());
            } else {
                current_screen.push(".".to_string());
            }
            cycle += 1;
            if result.is_none() {
                continue;
            }
            register = result.unwrap();
            if cycle % 40 == 0 && current_screen.len() == 40 {
                println!("register is {} at cycle {}", register, cycle);
                screen.push(current_screen.join(""));
                current_screen = vec![];
            }
            break;
        }
        
    });
    println!("Finished after {} cycles with register {}", cycle, register);
    screen
}

struct Instruction {
    operation: Operation,
    cycles: u32
}

enum Operation {
    Add(i32),
    Noop
}

impl Instruction {
    fn from(str: &str) -> Instruction {
        match str.split(" ").collect::<Vec<&str>>()[..] {
            ["addx", register] => Instruction {
                operation: Operation::Add( register.parse::<i32>().unwrap()),
                cycles: 2
            },
            ["noop"] => Instruction {
                operation: Operation::Noop,
                cycles: 1
            },
            _ => panic!("Unknown instruction: {}", str)
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
                    return Some(register + value)
                }
                None
            }
            Operation::Noop => {
                // println!("Start of Operation: Noop");
                self.cycles -= 1;
                if self.cycles == 0 {
                    return Some(register)
                }
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 3] = [
        "noop",
        "addx 3",
        "addx -5"
    ];

    fn example_input() -> Vec<String> {
        let filename = "day_10/src/example.txt";
        fileutils::lines_from_file(filename)
    }

    #[test]
    fn example() {
        let result = part1(EXAMPLE.iter().map(|str| str.to_string()).collect());
        assert_eq!(result, 1);
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
        println!("{}", result.join("\n"));
    }
}
