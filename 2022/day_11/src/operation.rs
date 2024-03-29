use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u64),
    AddOld,
    Multiply(u64),
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
                    _ => Some(Operation::Multiply(num.parse::<u64>().unwrap())),
                },
                "+" => match num {
                    "old" => Some(Operation::AddOld),
                    _ => Some(Operation::Add(num.parse::<u64>().unwrap())),
                },
                _ => panic!("Unknown operation: {}", operation.as_ref()),
            };
        }
        None
    }

    pub fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(num) => old + num,
            Operation::AddOld => old + old,
            Operation::Multiply(num) => old * num,
            Operation::MultiplyOld => old * old,
        }
    }
}
