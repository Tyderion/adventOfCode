use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub trait CardCounting {
    fn counts_as<'a>(&'a self, counted: &HashMap<&'a Self, u32>) -> &'a Self;
}

// All the traits needed for card sorting/counting/hashing/debugging
pub trait CardTraits:
    Eq + PartialEq + PartialOrd + Ord + Copy + CardCounting + From<char> + Debug + Hash
{
}
impl<T: Eq + PartialEq + PartialOrd + Ord + Copy + CardCounting + From<char> + Debug + Hash>
    CardTraits for T
{
}
