mod bid;
mod bidv1;
mod bidv2;

use bid::WithBid;
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

fn compute_games<T>(lines: &Vec<impl AsRef<str>>) -> u32
where
    T: PartialEq + PartialOrd + Eq + Ord + for<'a> From<&'a str> + WithBid,
{
    let mut bids = lines
        .iter()
        .map(|l| T::from(l.as_ref()))
        .collect::<Vec<T>>();

    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(index, bid)| (index as u32 + 1) * bid.get_bid())
        .sum()
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    compute_games::<BidV1>(lines)
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u32 {
    compute_games::<BidV2>(lines)
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
