use std::collections::HashMap;

pub trait WithBid {
    fn get_bid(&self) -> u32;
}

pub trait CardCounting {
    fn count_single_card<'a>(acc: HashMap<&'a Self, u32>, card: &'a Self) -> HashMap<&'a Self, u32>;
}
