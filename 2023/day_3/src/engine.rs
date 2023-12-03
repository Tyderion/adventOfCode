use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct PartNumber {
    pub id: u32,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
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

fn add_part_number(number: &Option<(usize, usize, String)>, list: &mut Vec<PartNumber>) {
    if let Some((start, end, num)) = &number {
        list.push(PartNumber {
            id: num.parse::<u32>().unwrap(),
            start: *start,
            end: *end,
        })
    }
}

impl Engine {
    pub fn parse(input: &Vec<impl AsRef<str>>) -> Engine {
        let mut parts = vec![];
        let mut part_numbers = HashMap::new();

        input.iter().enumerate().for_each(|(row, l)| {
            let mut part_nums: Vec<PartNumber> = vec![];
            let mut number: Option<(usize, usize, String)> = None;
            l.as_ref().char_indices().for_each(|(col, c)| match c {
                '0'..='9' => {
                    number = match &number {
                        Some((start, _, num)) => Some((*start, col, format!("{}{}", num, c))),
                        None => Some((col, col, String::from(c))),
                    }
                }
                _ => {
                    match c {
                        'A'..='z' | '.' => (),
                        _ => parts.push(Part {
                            symbol: c,
                            row,
                            col,
                        }),
                    }
                    add_part_number(&number, &mut part_nums);
                    number = None;
                }
            });
            add_part_number(&number, &mut part_nums);
            if part_nums.len() > 0 {
                part_numbers.insert(row, part_nums);
            }
        });

        return Engine {
            parts,
            part_numbers,
        };
    }
}
