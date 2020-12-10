#[derive(Eq, PartialEq, Debug, Clone)]
struct SummableRange(Range<usize>);

impl SummableRange {
    fn single(pos: usize) -> Self {
        Self(pos..pos + 1)
    }

    fn empty(pos: usize) -> Self {
        Self(pos..pos)
    }
}

impl Add for SummableRange {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.0.end == other.0.start {
            Self(self.0.start..other.0.end)
        } else {
            panic!(
                "trying to add segment wrong way: {:?} + {:?}",
                self.0, other.0
            );
        }
    }
}