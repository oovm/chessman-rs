use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct NCastlesState {
    size: isize,
    filled: Vec<isize>,
    unused: BTreeSet<isize>,
}

mod display;
