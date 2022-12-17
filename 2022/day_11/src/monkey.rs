use crate::operation::Operation;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct MonkeyId(i32);

impl MonkeyId {
    pub fn from<T: AsRef<str>>(input: T) -> MonkeyId {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?:Monkey |\s+ If (?:true|false): throw to monkey )(\d+):?$")
                    .unwrap();
        }
        if let Some(capture) = RE.captures(input.as_ref()) {
            return MonkeyId(capture.get(1).unwrap().as_str().parse::<i32>().unwrap());
        }
        panic!("Invalid monkey id: {}", input.as_ref());
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: MonkeyId,
    items: Vec<u64>,
    operation: Operation,
    pub test: u64,
    if_true: MonkeyId,
    if_false: MonkeyId,
    inspections: u64,
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

    fn parse_items<T: AsRef<str>>(input: T) -> Vec<u64> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s+Starting items: ((?:\d+(?:, )?)+)$").unwrap();
        }

        if let Some(capture) = RE.captures(input.as_ref()) {
            return capture
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|i| i.trim().parse::<u64>().unwrap())
                .collect();
        }
        vec![]
    }

    fn parse_test<T: AsRef<str>>(input: T) -> u64 {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s+Test: divisible by (\d+)$").unwrap();
        }
        if let Some(capture) = RE.captures(input.as_ref()) {
            return capture.get(1).unwrap().as_str().parse::<u64>().unwrap();
        }
        panic!("Invalid test: {}", input.as_ref());
    }

    pub fn inspect<F>(&self, worry_decrease: &mut F) -> (Monkey, Vec<(MonkeyId, u64)>)
    where F: FnMut(u64) -> u64 {
        let mut inspections = self.inspections;
        let result = self
            .items
            .iter()
            .map(|item| {
                inspections += 1;
                // Inspection increases worry level
                let new_item = worry_decrease(self.operation.apply(*item));
                if new_item % self.test == 0 {
                    (self.if_true, new_item)
                } else {
                    (self.if_false, new_item)
                }
            })
            .collect();
        (
            Monkey {
                id: self.id,
                inspections,
                items: vec![],
                if_false: self.if_false,
                if_true: self.if_true,
                operation: self.operation,
                test: self.test,
            },
            result,
        )
    }

    pub fn catch_item(self, item: u64) -> Self {
        let mut new_items = self.items.clone();
        new_items.push(item);
        Monkey {
            id: self.id,
            inspections: self.inspections,
            items: new_items,
            if_false: self.if_false,
            if_true: self.if_true,
            operation: self.operation,
            test: self.test,
        }
    }

    pub fn get_inspection_count(&self) -> u64 {
        self.inspections
    }
}
