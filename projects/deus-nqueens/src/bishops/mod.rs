use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
    iter::from_generator,
};
mod display;

#[derive(Clone, Debug)]
pub struct NBishopsState {
    size: isize,
    filled: Vec<isize>,
    unused: BTreeSet<isize>,
}

impl NBishopsState {
    pub fn new(size: usize) -> Self {
        Self { size: size as isize, filled: Vec::with_capacity(size), unused: (0..size as isize).collect() }
    }
    pub fn full_filled(&self) -> bool {
        self.unused.is_empty()
    }
    pub fn is_solution(&self) -> bool {
        for i in 0..self.size {
            if !self.filled.contains(&i) {
                return false;
            }
        }
        true
    }
    pub fn valid_at(&self, column: isize) -> bool {
        // let row = self.filled.len() as isize;
        let row = self.filled.len() as isize;
        self.filled.iter().enumerate().all(|(solution_row, &solution_column)| {
            // Test for same NE-SW diagonal
            column + row != solution_column + solution_row as isize &&
                // Test for same NW-SE diagonal
                column - row != solution_column - solution_row as isize
        })
    }
    pub fn go_walk(&mut self, column: isize) {
        self.filled.push(column);
        self.unused.remove(&column);
    }
    pub fn go_back(&mut self) {
        match self.filled.pop() {
            Some(s) => self.unused.insert(s),
            None => false,
        };
    }
}

/// O(n!) time to find all solutions
pub fn n_bishop_backtrack(size: usize) -> impl Iterator<Item = NBishopsState> {
    let mut stack = vec![NBishopsState::new(size)];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            if state.full_filled() {
                yield state;
                continue;
            };
            for row in state.unused.clone() {
                if state.valid_at(row) {
                    state.go_walk(row);
                    stack.push(state.clone());
                    state.go_back();
                }
            }
        }
    })
}
