use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::bid::WithBid;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Card {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Hash, Copy, Clone)]
enum Hand {
    FiveOfAKind([Card; 5]),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPairs([Card; 5]),
    Pair([Card; 5]),
    High([Card; 5]),
}

impl From<&str> for Hand {
    fn from(s: &str) -> Hand {
        if s.len() != 5 {
            panic!("Not a valid hand {}", s)
        }
        let cards = s.chars().map(Card::from).collect::<Vec<_>>();
        let card_counts =
            cards
                .iter()
                .fold(HashMap::new() as HashMap<Card, u32>, |mut acc, card| {
                    *acc.entry(*card).or_default() += 1;
                    acc
                });

        let mut card_counts = card_counts.iter().collect::<Vec<_>>();
        card_counts.sort_by_key(|s| std::cmp::Reverse(*s.1));
        // println!("Got cards {:#?} and counts {:#?}", cards, card_counts);

        match card_counts.iter().map(|c| c.1).take(2).collect::<Vec<_>>()[..] {
            [1, _] => Hand::High(cards.try_into().unwrap()),
            [2, 2] => Hand::TwoPairs(cards.try_into().unwrap()),
            [2, _] => Hand::Pair(cards.try_into().unwrap()),
            [3, 2] => Hand::FullHouse(cards.try_into().unwrap()),
            [3, _] => Hand::ThreeOfAKind(cards.try_into().unwrap()),
            [4, _] => Hand::FourOfAKind(cards.try_into().unwrap()),
            [5] => Hand::FiveOfAKind(cards.try_into().unwrap()),
            _ => panic!("IMPOSSIBLE {:?}", card_counts),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hand::FiveOfAKind(a), Hand::FiveOfAKind(b)) => Some(a.cmp(b)),
            (Hand::FiveOfAKind(_), _) => Some(Ordering::Greater),
            (_, Hand::FiveOfAKind(_)) => Some(Ordering::Less),
            (Hand::FourOfAKind(a), Hand::FourOfAKind(b)) => Some(a.cmp(b)),
            (Hand::FourOfAKind(_), _) => Some(Ordering::Greater),
            (_, Hand::FourOfAKind(_)) => Some(Ordering::Less),
            (Hand::FullHouse(a), Hand::FullHouse(b)) => Some(a.cmp(b)),
            (Hand::FullHouse(_), _) => Some(Ordering::Greater),
            (_, Hand::FullHouse(_)) => Some(Ordering::Less),
            (Hand::ThreeOfAKind(a), Hand::ThreeOfAKind(b)) => Some(a.cmp(b)),
            (Hand::ThreeOfAKind(_), _) => Some(Ordering::Greater),
            (_, Hand::ThreeOfAKind(_)) => Some(Ordering::Less),
            (Hand::TwoPairs(a), Hand::TwoPairs(b)) => Some(a.cmp(b)),
            (Hand::TwoPairs(_), _) => Some(Ordering::Greater),
            (_, Hand::TwoPairs(_)) => Some(Ordering::Less),
            (Hand::Pair(a), Hand::Pair(b)) => Some(a.cmp(b)),
            (Hand::Pair(_), _) => Some(Ordering::Greater),
            (_, Hand::Pair(_)) => Some(Ordering::Less),
            (Hand::High(a), Hand::High(b)) => Some(a.cmp(b)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord)]
pub struct BidV1 {
    hand: Hand,
    pub bid: u32,
}

impl PartialOrd for BidV1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl From<&str> for BidV1 {
    fn from(value: &str) -> Self {
        let (hand, bid) = value.split(" ").collect_tuple().unwrap();
        BidV1 {
            hand: Hand::from(hand),
            bid: bid.parse::<u32>().unwrap(),
        }
    }
}

impl WithBid for BidV1 {
    fn get_bid(&self) -> u32 {
        self.bid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let result = Hand::from("32T3K");
        assert_eq!(
            Hand::Pair([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
            result
        );
    }

    #[test]
    fn test_parse_bid() {
        let result = BidV1::from("32T3K 765");
        assert_eq!(
            BidV1 {
                hand: Hand::Pair([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
                bid: 765
            },
            result
        );
    }

    #[test]
    fn test_card_ordering() {
        let result = Card::from('K') > Card::from('9');
        assert!(result);
    }

    #[test]
    fn test_card_ordering2() {
        let result = Card::from('T') > Card::from('9');
        assert!(result);
    }

    #[test]
    fn test_hand_eq() {
        let result = Hand::Pair([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King])
            == Hand::Pair([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]);
        assert!(result);
    }

    #[test]
    fn test_hand_ne() {
        let result = Hand::Pair([Card::Three, Card::Five, Card::Ten, Card::Three, Card::King])
            == Hand::Pair([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]);
        assert!(!result);
    }

    #[test]
    fn test_card_array_ordering_greater() {
        let result = [Card::Three, Card::Five, Card::Ten, Card::Three, Card::King].cmp(&[
            Card::Three,
            Card::Two,
            Card::Ten,
            Card::Three,
            Card::King,
        ]);
        assert_eq!(Ordering::Greater, result);
    }

    #[test]
    fn test_card_array_ordering_less() {
        let result = [Card::Two, Card::Two, Card::Ten, Card::Three, Card::King].cmp(&[
            Card::Three,
            Card::Two,
            Card::Ten,
            Card::Three,
            Card::King,
        ]);
        assert_eq!(Ordering::Less, result);
    }

    #[test]
    fn test_card_array_ordering_equal() {
        let result = [Card::Two, Card::Two, Card::Ten, Card::Three, Card::King].cmp(&[
            Card::Two,
            Card::Two,
            Card::Ten,
            Card::Three,
            Card::King,
        ]);
        assert_eq!(Ordering::Equal, result);
    }
}
