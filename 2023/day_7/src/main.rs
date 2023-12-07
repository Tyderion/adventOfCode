mod bidv1;
mod bidv2;

use bidv1::BidV1;
use bidv2::BidV2;

pub fn main() {
    let filename = "day_7/src/input.txt";
    let input = fileutils::safe_lines_from_file(filename);
    let part1_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part1(lines),
    };
    let part2_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part2(lines),
    };
    println!("Sum of games: {}", part1_result);
    println!("Sum of part 2: {}", part2_result);
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    let mut bids = lines
        .iter()
        .map(|l| BidV1::from(l.as_ref()))
        .collect::<Vec<BidV1>>();

    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(index, bid)| (index as u32 + 1) * bid.bid)
        .sum()
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u32 {
    let mut bids = lines
        .iter()
        .map(|l| BidV2::from(l.as_ref()))
        .collect::<Vec<BidV2>>();

    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(index, bid)| (index as u32 + 1) * bid.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 5] = [
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    #[test]
    fn example_case_part1() {
        let result = part1(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 6440);
    }

    #[test]
    fn example_case_part2() {
        let result = part2(&EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect());
        assert_eq!(result, 5905);
    }
}
