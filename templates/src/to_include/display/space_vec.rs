struct SpaceVec<T>(pub Vec<T>);

impl<T: Display> Display for SpaceVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.0.len();
        for (index, i) in self.0.iter().enumerate() {
            write!(f, "{}", i)?;
            if index + 1 != len {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}