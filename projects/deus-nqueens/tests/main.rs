use deus_nqueens::{
    n_bishops_backtrack, n_castles_permute, n_queens_backtrack, n_queens_modular, n_queens_symmetry, NQueensState,
};

#[test]
fn test_n_queens_backtrack() {
    let mut count = 0;
    for s in n_queens_backtrack(4) {
        println!("{s}");
        count += 1;
    }
    println!("{} solutions found", count);
}

#[test]
fn test_n_queens_symmetry() {
    let mut state = NQueensState::new(8);
    state.go_walk(1);
    println!("{:?}", state.symmetry_available_moves());
    println!("{}", state);
    state.go_walk(3);
    println!("{:?}", state.symmetry_available_moves());
    println!("{}", state);

    let mut count = 0;
    for s in n_queens_symmetry(8) {
        println!("{s}");
        count += 1;
    }
    println!("{} solutions found", count);
}

#[test]
fn test_n_queens_modular() {
    let mut count = 0;
    for n in 0..20 {
        match n_queens_modular(n) {
            None => {
                println!("{n}: no solutions found");
            }
            Some(s) => {
                println!("{n}: \n{s}");
                count += 1;
            }
        }
    }
    println!("{} solutions found", count);
}

#[test]
fn test_n_castles_backtrack() {
    let mut count = 0;
    for s in n_castles_permute(4) {
        println!("{s}");
        count += 1;
    }
    println!("{} solutions found", count);
}

#[test]
fn test_n_bishops_backtrack() {
    let mut count = 0;
    for s in n_bishops_backtrack(4) {
        println!("{s}");
        count += 1;
    }
    println!("{} solutions found", count);
}
