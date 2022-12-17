use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Operation {
    Add(i32),
    AddOld,
    Multiply(i32),
    MultiplyOld,
}

impl Operation {
    pub fn from<T: AsRef<str>>(operation: T) -> Option<Operation> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s+Operation: new = old ([*+]) (old|\d+)").unwrap();
        }
        if let Some(capture) = RE.captures(operation.as_ref()) {
            let op = capture.get(1).unwrap().as_str();
            let num = capture.get(2).unwrap().as_str();
            return match op {
                "*" => match num {
                    "old" => Some(Operation::MultiplyOld),
                    _ => Some(Operation::Multiply(num.parse::<i32>().unwrap())),
                },
                "+" => match num {
                    "old" => Some(Operation::AddOld),
                    _ => Some(Operation::Add(num.parse::<i32>().unwrap())),
                },
                _ => panic!("Unknown operation: {}", operation.as_ref()),
            };
        }
        None
    }

    pub fn apply(&self, old: i32) -> i32 {
        match self {
            Operation::Add(num) => old + num,
            Operation::AddOld => old + old,
            Operation::Multiply(num) => old * num,
            Operation::MultiplyOld => old * old,
        }
    }
}
