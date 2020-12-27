struct Chars(pub Vec<u8>);

impl Display for Chars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{}", char::from(*i))?;
        }
        Ok(())
    }
}
