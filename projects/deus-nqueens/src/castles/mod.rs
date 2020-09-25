use std::{collections::BTreeSet, iter::from_generator};

mod display;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct NCastlesState {
    size: usize,
    filled: Vec<usize>,
}

/// O(n!) time to find all solutions
pub fn n_castles_permute(size: usize) -> impl Iterator<Item = NCastlesState> {
    (0..size).permutations(size).map(move |filled| NCastlesState { size, filled })
}
