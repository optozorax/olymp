pub struct Joined<I, By> {
    elements: I,
    by: By,
}

pub trait JoinedByTrait: Sized {
    fn joined_by<By: Display>(self, by: By) -> Joined<Self, By>;
}

impl<I: Iterator<Item = T> + Clone, T: Display> JoinedByTrait for I {
    fn joined_by<By: Display>(self, by: By) -> Joined<Self, By> {
        Joined {
            elements: self,
            by,
        }
    }
}

impl<I: Iterator<Item = T> + Clone, T: Display, By: Display> Display for Joined<I, By> {
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
