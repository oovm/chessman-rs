#![feature(iter_from_generator)]
#![feature(generators)]

mod bishops;
mod castles;
mod queens;
mod queens_symmetry;

pub use crate::{
    bishops::{n_bishops_backtrack, NBishopsState},
    castles::{n_castles_permute, NCastlesState},
    queens::{n_queens_backtrack, n_queens_modular, n_queens_symmetry, NQueensState},
};
