pub struct Joined<'a, I> {
    elements: I,
    by: &'a str,
}

pub trait JoinedByTrait: Sized {
    fn joined_by(self, by: &str) -> Joined<Self>;
}

impl<I: Iterator<Item = T> + Clone, T: Display> JoinedByTrait for I {
    fn joined_by(self, by: &str) -> Joined<Self> {
        Joined {
            elements: self,
            by,
        }
    }
}

impl<I: Iterator<Item = T> + Clone, T: Display> Display for Joined<'_, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
