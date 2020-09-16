use std::iter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NQueens {
    side_length: usize,
}

impl NQueens {
    pub fn new(side_length: usize) -> NQueens {
        NQueens { side_length }
    }

    pub fn solve(&self) -> Solutions {
        (0..self.side_length).fold(
            Solutions(vec![Rows(Vec::with_capacity(self.side_length))]),
            |acc, row| self.add_new_layer(&acc, &Row(row)),
        )
    }

    fn add_new_layer(
        &self,
        &Solutions(ref previous_solutions): &Solutions,
        row: &Row,
    ) -> Solutions {
        let rows = previous_solutions
            .iter()
            .flat_map(|previous_solution| {
                (0..self.side_length)
                    .filter_map(|col_num| {
                        let column = Column(col_num);
                        if is_safe(previous_solution, row, &column) {
                            let mut cloned_data = previous_solution.0.clone();
                            cloned_data.push(column);
                            Some(Rows(cloned_data))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Rows>>()
            })
            .collect();
        Solutions(rows)
    }
}

/// Check to see if a given Row and Column are safe to use
/// in the context of the provided existing board configuration
/// of other Queens
///
/// Note that this only makes sense when the board configuration is for
/// Queen pieces because of the check that we do (row, col, diagonals).
fn is_safe(&Rows(ref solution_rows): &Rows, &Row(row): &Row, &Column(column): &Column) -> bool {
    solution_rows.iter().enumerate().all(
        |(solution_row, &Column(solution_column))| {
            // Test for same column
            column != solution_column &&
            // Test for same row
            row != solution_row &&
            // Test for same NE-SW diagonal
                column as isize + row as isize !=
                    solution_column as isize + solution_row as isize &&
            // Test for same NW-SE diagonal
                column as isize - row as isize !=
                    solution_column as isize - solution_row as isize
        },
    )
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Solutions(Vec<Rows>);

impl Solutions {
    pub fn render(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|&Rows(ref columns)| {
                let board_slots = columns.len() ^ 2;
                let mut s = String::with_capacity(board_slots + columns.len());
                for &Column(q_column) in columns.iter() {
                    let mut row_vec: Vec<char> =
                        iter::repeat('＿').take(columns.len() + 1).collect();
                    row_vec[q_column] = '〇';
                    row_vec[columns.len()] = '\n';
                    let row_s: String = row_vec.iter().collect();
                    s.push_str(&row_s);
                }
                s
            })
            .collect()
    }

    pub fn data(&self) -> &Vec<Rows> {
        &self.0
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rows(Vec<Column>);

impl Rows {
    pub fn data(&self) -> &Vec<Column> {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Column(usize);

impl Column {
    pub fn data(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Row(usize);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nqueens() {
        let solutions = NQueens::new(8).solve();
        let rendered = solutions.render();
        for render in rendered.iter() {
            println!("{}", render);
        }

        fn build_checking_tuple(
            (row_idx, &Column(col)): (usize, &Column),
        ) -> (usize, isize, isize) {
            (
                col,
                row_idx as isize + col as isize,
                row_idx as isize - col as isize,
            )
        }

        for solution in solutions.data() {
            let checker: Vec<_> = solution
                .data()
                .iter()
                .enumerate()
                .map(build_checking_tuple)
                .collect();
            for (row_idx, &Column(column)) in solution.data().iter().enumerate() {
                let checker_without_current = {
                    let mut cloned = checker.clone();
                    cloned.remove(row_idx);
                    cloned
                };
                assert!(checker_without_current.iter().all(|&checking_tuple| {
                    build_checking_tuple((row_idx, &Column(column))) != checking_tuple
                }));
            }
        }
    }
}
