use super::*;

impl Display for NQueensState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rank {
            for column in 0..self.rank {
                if self.filled.contains(&column) && self.filled.iter().position(|&x| x == column).unwrap() as isize == row {
                    write!(f, "Q")?;
                }
                else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
