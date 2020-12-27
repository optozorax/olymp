use std::fmt;

pub struct Joined<'a, I> {
    pub elements: I,
    pub by: &'a str,
}

impl<I: Iterator<Item = T> + Clone, T: fmt::Display> fmt::Display for Joined<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for i in self.elements.clone() {
            if !first {
                write!(f, "{}", self.by)?;
            }
            write!(f, "{}", i)?;
            first = false;
        }
        Ok(())
    }
}
