use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct PartNumber {
    pub id: u32,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Part {
    pub symbol: char,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Engine {
    pub parts: Vec<Part>,
    pub part_numbers: HashMap<usize, Vec<PartNumber>>,
}

impl Engine {
    pub fn parse(input: &Vec<impl AsRef<str>>) -> Engine {
        let mut parts = vec![];
        let mut part_numbers = HashMap::new();

        input.iter().enumerate().for_each(|(row, l)| {
            let mut parts_nums: Vec<PartNumber> = vec![];
            let mut number: Option<(usize, usize, String)> = None;
            l.as_ref().char_indices().for_each(|(col, c)| match c {
                '0'..='9' => {
                    number = match &number {
                        Some((ref start, _, ref num)) => {
                            let mut new_value = String::from(num);
                            new_value.push(c);
                            Some((*start, col, new_value.to_string()))
                        }
                        None => Some((col, col, String::from(c))),
                    }
                }
                _ => {
                    match &number {
                        None => (),
                        Some((start, end, num)) => parts_nums.push(PartNumber {
                            id: num.parse::<u32>().unwrap(),

                            start: *start,
                            end: *end,
                        }),
                    }
                    match c {
                        'A'..='z' | '.' | '0'..='9' => (),
                        _ => {
                            parts.push(Part {
                                symbol: c,
                                row,
                                col,
                            })
                        }
                    }
                    number = None;
                }
            });
            if parts_nums.len() > 0 {
                part_numbers.insert(row, parts_nums);
            }
        });

        return Engine {
            parts,
            part_numbers,
        };
    }
}
