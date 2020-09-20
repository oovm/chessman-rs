use std::iter::from_generator;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct NQueensState {
    size: isize,
    filled: Vec<isize>,
}

mod display;

impl NQueensState {
    pub fn new(size: usize) -> Self {
        Self {
            size: size as isize,
            filled: Vec::with_capacity(size),
        }
    }
    pub fn is_filled(&self) -> bool {
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
        let row = self.filled.len() as isize;
        self.filled
            .iter()
            .enumerate()
            .all(|(solution_row, &solution_column)| {
                // Test for same column
                column != solution_column &&
                    // Test for same NE-SW diagonal
                    column + row != solution_column + solution_row as isize &&
                    // Test for same NW-SE diagonal
                    column - row != solution_column - solution_row as isize
            })
    }
    pub fn go_step(&mut self, column: isize) {
        self.filled.push(column);
    }
    pub fn go_back(&mut self) {
        self.filled.pop();
    }
}


pub fn solve_n_queens_backtracking(size: usize) -> impl Iterator<Item=NQueensState> {
    let state = NQueensState::new(size);
    let mut stack = vec![state];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            let column = state.filled.len() as isize;
            if state.is_filled() && state.is_solution() {
                yield state;
                continue;
            };
            for row in 0..state.size {
                if state.valid_at(row) {
                    state.go_step(row);
                    stack.push(state.clone());
                    state.go_back();
                }
            }
        }
    })
}

#[test]
fn main() {
    for solution in solve_n_queens_backtracking(4).take(5) {
        println!("{}", solution);
    }
}