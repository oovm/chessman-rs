#![feature(iter_from_generator)]
#![feature(generators)]

mod castles;
mod queens;

pub use crate::queens::{n_queens_backtrack, n_queens_modular, NQueensState};
