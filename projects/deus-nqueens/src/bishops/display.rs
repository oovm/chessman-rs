use super::*;

impl Display for NBishopsState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size {
            for column in 0..self.size {
                if self.filled.contains(&column) && self.filled.iter().position(|&x| x == column).unwrap() as isize == row {
                    write!(f, "B")?;
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
