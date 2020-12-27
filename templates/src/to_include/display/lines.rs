//----------------------------------------------------------------------------

struct Lines<T>(pub Vec<T>);

impl<T: Display> Display for Lines<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}