use std::{collections::HashMap, fmt};

pub trait WithBid {
    fn get_bid(&self) -> u32;
}

pub trait CardCounting {
   fn counts_as<'a>(&'a self, counts: &HashMap<&'a Self, u32>) -> &'a Self;
}

pub trait CardTraits: Eq + PartialEq + PartialOrd + Ord + Copy + Clone + CardCounting + From<char> + fmt::Debug + std::hash::Hash {}
impl<T: Eq + PartialEq + PartialOrd + Ord + Copy + Clone + CardCounting + From<char> + fmt::Debug + std::hash::Hash> CardTraits for T {}
