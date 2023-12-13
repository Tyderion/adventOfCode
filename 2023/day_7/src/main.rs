mod bid;
mod card_p1;
mod card_p2;
mod traits;
use bid::Bid;
use traits::CardTraits;

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

fn compute_total_winnings<T>(lines: &Vec<impl AsRef<str>>) -> u32
where
    T: CardTraits,
{
    let mut bids = lines
        .iter()
        .map(|l| Bid::<T>::from(l.as_ref()))
        .collect::<Vec<_>>();

    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(index, bid)| (index as u32 + 1) * bid.amount)
        .sum()
}

fn part1(lines: &Vec<impl AsRef<str>>) -> u32 {
    compute_total_winnings::<card_p1::Card>(lines)
}

fn part2(lines: &Vec<impl AsRef<str>>) -> u32 {
    compute_total_winnings::<card_p2::Card>(lines)
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
