use crate::operation::Operation;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct MonkeyId(i32);

impl MonkeyId {
    pub fn from<T: AsRef<str>>(input: T) -> MonkeyId {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?:Monkey |\s+ If (?:true|false): throw to monkey )(\d+):?$").unwrap();
        }
        if let Some(capture) = RE.captures(input.as_ref()) {
            return MonkeyId(capture.get(1).unwrap().as_str().parse::<i32>().unwrap());
        }
        panic!("Invalid monkey id: {}", input.as_ref());
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub id: MonkeyId,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    if_true: MonkeyId,
    if_false: MonkeyId,
    inspections: i32,
}

impl Monkey {
    pub fn from<T: AsRef<str>>(input: Vec<T>) -> Monkey {
        if input.len() != 6 {
            panic!(
                "Invalid monkey config: {}",
                input
                    .iter()
                    .map(|x| x.as_ref())
                    .collect::<Vec<&str>>()
                    .join("\n")
            );
        }
        Monkey {
            id: MonkeyId::from(input[0].as_ref()),
            items: Monkey::parse_items(input[1].as_ref()),
            operation: Operation::from(input[2].as_ref()).unwrap(),
            test: Monkey::parse_test(input[3].as_ref()),
            if_true: MonkeyId::from(input[4].as_ref()),
            if_false: MonkeyId::from(input[5].as_ref()),
            inspections: 0,
        }
    }

    fn parse_items<T: AsRef<str>>(input: T) -> Vec<i32> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s+Starting items: ((?:\d+(?:, )?)+)$").unwrap();
        }

        if let Some(capture) = RE.captures(input.as_ref()) {
            return capture
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|i| i.trim().parse::<i32>().unwrap())
                .collect();
        }
        vec![]
    }

    fn parse_test<T: AsRef<str>>(input: T) -> i32 {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s+Test: divisible by (\d+)$").unwrap();
        }
        if let Some(capture) = RE.captures(input.as_ref()) {
            return capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        }
        panic!("Invalid test: {}", input.as_ref());
    }

    pub fn inspect(&mut self) -> Vec<(MonkeyId, i32)> {
        let result = self.items.iter_mut().map(|item| {
            self.inspections += 1;
            // Inspection increases worry level
            *item = self.operation.apply(*item);
            // After inspection, worry level decreases
            *item /= 3;
            if *item % self.test == 0 {
                (self.if_true, *item)
            } else {
                (self.if_false, *item)
            }
        }).collect();
        self.items = vec![];
        result
    }

    pub fn catch_item(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn get_inspection_count(&self) -> i32 {
        self.inspections
    }
}
