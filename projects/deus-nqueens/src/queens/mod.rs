use std::collections::BTreeSet;
use std::iter::from_generator;
use std::fmt::Display;
use rand::Rng;

mod display;

#[derive(Clone, Debug)]
pub struct NQueensState {
    size: isize,
    filled: Vec<isize>,
    unfilled: BTreeSet<isize>,
}


impl NQueensState {
    pub fn new(size: usize) -> Self {
        Self {
            size: size as isize,
            filled: Vec::with_capacity(size),
            unfilled: (0..size as isize).collect(),
        }
    }
    pub fn full_filled(&self) -> bool {
        self.unfilled.is_empty()
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
        self.unfilled.remove(&column);
    }
    pub fn go_back(&mut self) {
        match self.filled.pop() {
            Some(s) => {
                self.unfilled.insert(s);
            }
            None => {}
        }
    }
}

pub fn n_queens_backtracking(size: usize) -> impl Iterator<Item=NQueensState> {
    let mut stack = vec![NQueensState::new(size)];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            if state.full_filled()  {
                yield state;
                continue;
            };
            for row in state.unfilled.clone() {
                if state.valid_at(row) {
                    state.go_step(row);
                    stack.push(state.clone());
                    state.go_back();
                }
            }
        }
    })
}

pub fn n_queens_greedy(size: usize) -> Vec<NQueensState> {
    let mut stack = vec![NQueensState::new(size)];
    todo!()
}

#[test]
fn test_n_queens_backtracking() {
    let mut count = 0;
    for solution in n_queens_backtracking(8) {
        println!("{}", solution);
        count += 1;
    }
    println!("{} solutions found", count);
}