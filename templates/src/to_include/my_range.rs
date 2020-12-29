#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct MyRange {
	start: i64,
	end: i64,
}

impl<T> From<Range<T>> for MyRange
where T: TryInto<i64>, T::Error: Debug
{
	fn from(r: Range<T>) -> MyRange {
		MyRange { 
			start: r.start.try_into().unwrap(), 
			end: r.end.try_into().unwrap() 
		}.check().unwrap()
	}
}

impl<T> From<RangeInclusive<T>> for MyRange
where T: TryInto<i64> + Copy, T::Error: Debug
{
	fn from(r: RangeInclusive<T>) -> MyRange {
		MyRange { 
			start: (*r.start()).try_into().unwrap(), 
			end: (*r.end()).try_into().unwrap() + 1 
		}.check().unwrap()
	}
}

impl MyRange {
	fn safe_new(start: i64, end: i64) -> Option<MyRange> { MyRange { start, end }.check() }
	fn not_sorted_(a: i64, b: i64) -> MyRange { MyRange { start: min(a, b), end: max(a, b) } }
	fn not_sorted_inclusive(a: i64, b: i64) -> MyRange { MyRange { start: min(a, b), end: max(a, b)+1 } }
	fn point(start: i64) -> MyRange { (start..=start).into() }
	fn check(self) -> Option<Self> { Some(self).filter(|x| x.start <= x.end) }
	fn not_empty(self) -> Option<Self> { Some(self).filter(|x| x.start < x.end) }
	fn contains(&self, value: i64) -> bool { self.start <= value && value < self.end }
	fn intersect(&self, other: Self) -> Option<Self> {
		Self::safe_new(max(self.start, other.start), min(self.end, other.end))
	}
	fn split(&self, at: i64) -> Option<(MyRange, MyRange)> {
		Some((Self::safe_new(self.start, at)?, Self::safe_new(at, self.end)?))
	}
}

impl Add<i64> for MyRange {
    type Output = Self;
    fn add(self, other: i64) -> Self {
    	MyRange { start: self.start + other, end: self.end + other }
    }
}
impl Sub<i64> for MyRange {
    type Output = Self;
    fn sub(self, other: i64) -> Self {
        MyRange { start: self.start - other, end: self.end - other }
    }
}

impl<T> From<MyRange> for Range<T>
where T: TryFrom<i64>, T::Error: Debug
{
	fn from(r: MyRange) -> Range<T> {
		r.start.try_into().unwrap()..r.end.try_into().unwrap()
	}
}