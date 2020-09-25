use super::*;

impl Display for NBishopsState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for column in &self.filled {
            for index in 0..self.size {
                if index.eq(column) {
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
