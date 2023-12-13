use std::{cmp::Reverse, collections::HashMap};

use crate::traits::CardCounting;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
            'J' => Card::Joker,
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

// fn count_single_card(mut map: HashMap<Card, u32>, card: &Card) -> HashMap<Card, u32> {
//     if *card == Card::Joker && acc.len() > 0 {
//         let clone = acc.clone();
//         // sort by most present and then by most valuable card
//         let mut existing = clone.iter().collect::<Vec<_>>();
//         existing.sort_by_key(|v| Reverse((v.1, v.0)));
//         *acc.entry(*existing.first().unwrap().0).or_default() += 1;
//     } else {
//         *acc.entry(*card).or_default() += 1;
//     }
//     acc
// }

impl CardCounting for Card {
    fn counts_as<'a>(&'a self, counts: &HashMap<&'a Self, u32>) -> &'a Self {
        if *self == Card::Joker && counts.len() > 0 {
            let clone = counts.clone();
            // sort by most present and then by most valuable card
            let mut existing = clone.iter().collect::<Vec<_>>();
            existing.sort_by_key(|v| Reverse((v.1, v.0)));
            *existing.first().unwrap().0
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

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
