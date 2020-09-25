use std::{
    fmt::{Display, Formatter},
    iter::from_generator,
};
mod display;

#[derive(Clone, Debug)]
pub struct NBishopsState {
    size: isize,
    filled: Vec<isize>,
}

impl NBishopsState {
    pub fn new(size: usize) -> Self {
        Self { size: size as isize, filled: Vec::with_capacity(size) }
    }
    pub fn full_filled(&self) -> bool {
        self.filled.len() == self.size as usize
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
    }
    pub fn go_back(&mut self) {
        self.filled.pop();
    }
}

/// O(n × n!) time to find all solutions
pub fn n_bishops_backtrack(size: usize) -> impl Iterator<Item = NBishopsState> {
    let mut stack = vec![NBishopsState::new(size)];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            if state.full_filled() {
                yield state;
                continue;
            };
            for row in 0..state.size {
                if state.valid_at(row) {
                    state.go_walk(row);
                    stack.push(state.clone());
                    state.go_back();
                }
            }
        }
    })
}
