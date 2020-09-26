use num::Integer;
use std::{collections::BTreeSet, fmt::Display, iter::from_generator};

mod display;
mod symmetry;

#[derive(Clone, Debug)]
pub struct NQueensState {
    rank: isize,
    filled: Vec<isize>,
    unused: BTreeSet<isize>,
}

impl NQueensState {
    /// Create a new NQueensState
    ///
    /// # Arguments
    ///
    /// * `size`:  size of the board
    ///
    /// returns: NQueensState
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(size: usize) -> Self {
        Self { rank: size as isize, filled: Vec::with_capacity(size), unused: (0..size as isize).collect() }
    }
    pub fn full_filled(&self) -> bool {
        self.unused.is_empty()
    }
    pub fn is_solution(&self) -> bool {
        for i in 0..self.rank {
            if !self.filled.contains(&i) {
                return false;
            }
        }
        true
    }
    /// All available moves
    pub fn available_moves(&self) -> Vec<isize> {
        self.unused.iter().copied().filter(|x| self.valid_at(*x)).collect()
    }

    pub fn valid_at(&self, column: isize) -> bool {
        let row = self.filled.len() as isize;
        self.filled.iter().enumerate().all(|(solution_row, &solution_column)| {
            // Test for same column
            column != solution_column
                // Test for same NE-SW diagonal
                && column + row != solution_column + solution_row as isize
                // Test for same NW-SE diagonal
                && column - row != solution_column - solution_row as isize
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

/// O(n × n!) time to find all solutions
pub fn n_queens_backtrack(size: usize) -> impl Iterator<Item = NQueensState> {
    let mut stack = vec![NQueensState::new(size)];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            if state.full_filled() {
                yield state;
                continue;
            };
            for row in state.available_moves() {
                state.go_walk(row);
                stack.push(state.clone());
                state.go_back();
            }
        }
    })
}

/// O(n × n!) time to find all solutions
pub fn n_queens_symmetry(size: usize) -> impl Iterator<Item = NQueensState> {
    let mut stack = vec![NQueensState::new(size)];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            if state.full_filled() {
                yield state;
                continue;
            };
            for row in state.symmetry_available_moves() {
                state.go_walk(row);
                stack.push(state.clone());
                state.go_back();
            }
        }
    })
}

/// O(1) time to find one solution if exists
///
/// <https://arxiv.org/abs/1805.07329>
pub fn n_queens_modular(n: usize) -> Option<NQueensState> {
    let mut arrange = vec![0; n];
    unsafe {
        match n + 1 {
            1 => return None,
            2 => *arrange.get_unchecked_mut(0) = 1,
            3 | 4 => return None,
            // Lemma 1.9: n = 12k + 2, where 6k+2, k -> 2k
            m if n % 12 == 2 => {
                // println!("1.9: {n}");
                for i in 1..=n {
                    *arrange.get_unchecked_mut(i - 1) = match n / 2 {
                        _ if i == n => (2 * i + 4) % m,
                        k if i < k && i.is_odd() => (2 * i + 4) % m,
                        k if i < k && i.is_even() => (2 * i) % m,
                        _ => (2 * i + 2) % m,
                    };
                }
            }
            // Lemma 1.6: n = 12k-4, where 6k+2, k -> 2k+1
            m if n % 12 == 8 => {
                // println!("1.6: {n}");
                for i in 1..=n {
                    *arrange.get_unchecked_mut(i - 1) = match n / 2 {
                        k if n > k && i.is_odd() => (2 * i + 2) % m,
                        k if n > k && i.is_even() => (2 * i - 2) % m,
                        _ => (2 * i) % m,
                    }
                }
            }
            // Lemma 1.7: n = 6k, 6k+4
            m if n % 6 == 0 || n % 6 == 4 => {
                // println!("1.7: {n}");
                for i in 1..=n {
                    *arrange.get_unchecked_mut(i - 1) = (2 * i) % m
                }
            }
            // Lemma 1.8: n = 6k + 1, 6k + 5
            m if n % 6 == 1 || n % 6 == 5 => {
                // println!("1.8: {n}");
                for i in 1..=n {
                    *arrange.get_unchecked_mut(i - 1) = match n / 2 {
                        k if i > k => (2 * i + 1) % m,
                        _ => (2 * i) % m,
                    }
                }
            }
            // Lemma 1.10: n = 6k + 3
            m if n % 6 == 3 => {
                // println!("1.10: {n}");
                for i in 1..=n {
                    *arrange.get_unchecked_mut(i - 1) = match (n - 1) / 2 {
                        k if i < k => (2 * i + 2) % m,
                        k if i > k => (2 * i + 5) % m,
                        _ => (2 * i + 4) % m,
                    }
                }
            }
            _ => return None,
        }
    }
    Some(NQueensState {
        rank: n as isize,
        filled: arrange.iter().map(|s| *s as isize - 1).collect(),
        unused: BTreeSet::default(),
    })
}
