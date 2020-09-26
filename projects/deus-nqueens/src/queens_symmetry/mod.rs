use crate::NQueensState;
use ndarray::{Array2, Axis};
use std::iter::from_generator;

/// unique solutions for n-queens problem under rotation and reflection
#[derive(Clone)]
pub struct NQueensSymmetry {
    rank: usize,
    board: Array2<bool>,
    // column
    filled: Vec<isize>,
}

impl NQueensSymmetry {
    pub fn new(rank: usize) -> Self {
        // all points available
        Self { rank, board: Array2::from_elem((rank, rank), true), filled: Vec::new() }
    }
    pub fn rank(&self) -> usize {
        self.rank
    }
    /// all points horizontally, vertically, diagonally related to (x, y)
    pub fn related_hvd(&self, x: isize, y: isize) -> impl Iterator<Item = (usize, usize)> {
        let n = self.rank as isize;
        from_generator(move || {
            for i in 0..n {
                // horizontal
                yield (i, y);
                // vertical
                yield (x, i);
                // diagonal
                yield (x + i - y, i);
                // anti-diagonal
                yield (x - i + y, i);
            }
        })
        .filter(move |(x, y)| *x >= 0 && *x < n && *y >= 0 && *y < n)
        .map(|(x, y)| (x as usize, y as usize))
    }
    /// all points under rotation and mirror symmetry
    pub fn related_sym(&self, x: isize, y: isize) -> impl Iterator<Item = (usize, usize)> {
        let n = self.rank as isize;
        from_generator(move || {
            yield (x, y);
            yield (x, n - y - 1);
            yield (n - x - 1, y);
            yield (n - x - 1, n - y - 1);
            yield (y, x);
            yield (y, n - x - 1);
            yield (n - y - 1, x);
            yield (n - y - 1, n - x - 1);
        })
        .map(|(x, y)| (x as usize, y as usize))
    }
    pub fn disable_related(&mut self, x: isize, y: isize) {
        for (rx, ry) in self.related_sym(x, y) {
            self.board[(rx, ry)] = false;
        }
        for (x, y) in self.related_hvd(x, y) {
            self.board[(x, y)] = false;
        }
    }
    pub fn enable_related(&mut self, x: isize, y: isize) {
        for (rx, ry) in self.related_sym(x, y) {
            self.board[(rx, ry)] = true;
        }
        for (x, y) in self.related_hvd(x, y) {
            self.board[(x, y)] = true;
        }
    }
    pub fn go_walk(&mut self, y: isize) {
        let x = self.filled.len() as isize;
        self.filled.push(y);
        self.disable_related(x, y);
    }
    pub fn go_back(&mut self) {
        if let Some(y) = self.filled.pop() {
            let x = self.filled.len() as isize;
            self.enable_related(x, y)
        }
    }
    pub fn available_moves(&self) -> Vec<isize> {
        let col = self.filled.len();
        // all true point on next column
        self.board.index_axis(Axis(1), col).iter().enumerate().filter(|(_, &v)| v).map(|(i, _)| i as isize).collect()
    }
    pub fn full_filled(&self) -> bool {
        self.filled.len() == self.rank
    }
}

pub fn n_queens_backtrack(size: usize) -> impl Iterator<Item = NQueensSymmetry> {
    let mut stack = vec![NQueensSymmetry::new(size)];
    from_generator(move || {
        while let Some(mut state) = stack.pop() {
            if state.full_filled() {
                yield state;
                continue;
            };
            // println!("Available  {:?}", state.available_moves());
            for y in state.available_moves() {
                state.go_walk(y);
                stack.push(state.clone());
                state.go_back();
            }
        }
    })
}

#[test]
fn test_n_queens_backtrack() {
    let mut count = 0;
    for state in n_queens_backtrack(8) {
        count += 1;
        println!("{:?}", state.filled);
    }
    assert_eq!(92, count);
}
