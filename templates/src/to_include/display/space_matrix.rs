//----------------------------------------------------------------------------

struct SpaceMatrix<T>(pub Vec<Vec<T>>);

impl<T: Display> Display for SpaceMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for (index, i) in line.iter().enumerate() {
                if index != 0 { write!(f, " ")?; }
                write!(f, "{}", i)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
