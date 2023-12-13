use std::collections::HashMap;

use crate::traits::CardCounting;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Card {
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

impl CardCounting for Card {
    fn counts_as<'a>(&'a self, counts: &HashMap<&'a Self, u32>) -> &'a Self {
        self
    }

    // fn count_single_card<'a>(mut acc: &'a mut HashMap<&Card, u32>, card: &Card) -> &'a HashMap<&'a Card, u32> {
    //     *acc.entry(card).or_default() += 1;
    //     acc
    // }
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
