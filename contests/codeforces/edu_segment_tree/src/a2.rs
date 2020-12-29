#[derive(Eq, PartialEq, Debug, Clone)]
struct SummableRange(Range<usize>);

impl SummableRange {
	fn single(pos: usize) -> Self { Self(pos..pos + 1) }

	fn empty(pos: usize) -> Self { Self(pos..pos) }
}

impl Add for SummableRange {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		if self.0.end == other.0.start {
			Self(self.0.start..other.0.end)
		} else {
			panic!("trying to add segment wrong way: {:?} + {:?}", self.0, other.0);
		}
	}
}

#[fastio::fastio]
pub fn main() {
	let _n = read!(i64);
	let m = read!(i64);
	let mut a = readln!(i64);

	#[derive(Eq, PartialEq, Debug, Clone)]
	struct SumPos {
		sum: i64,
		on: SummableRange,
	}

	impl SumPos {
		fn new(pos: usize, x: i64) -> Self { Self { sum: x, on: SummableRange::single(pos) } }

		fn empty(pos: usize) -> Self { Self { sum: 0, on: SummableRange::empty(pos) } }
	}

	impl Add for SumPos {
		type Output = Self;

		fn add(self, other: Self) -> Self { Self { sum: self.sum + other.sum, on: self.on + other.on } }
	}

	impl PartialOrd for SumPos {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
	}

	impl Ord for SumPos {
		fn cmp(&self, other: &Self) -> Ordering { self.sum.cmp(&other.sum) }
	}

	#[derive(Clone)]
	struct MaxSegmentSum {
		all: SumPos,
		suffix: SumPos,
		prefix: SumPos,
		middle: SumPos,
	}

	let mut tree = SegmentTree::create(
		&mut a,
		|_on, s1: MaxSegmentSum, s2: MaxSegmentSum| MaxSegmentSum {
			all: s1.all.clone() + s2.all.clone(),
			suffix: std::cmp::max(s1.suffix.clone() + s2.all, s2.suffix),
			prefix: std::cmp::max(s1.all + s2.prefix.clone(), s1.prefix),
			middle: std::cmp::max(s1.suffix + s2.prefix, std::cmp::max(s1.middle, s2.middle)),
		},
		|pos, x| {
			let result = std::cmp::max(SumPos::new(pos, x), SumPos::empty(pos));
			MaxSegmentSum {
				all: SumPos::new(pos, x),
				suffix: std::cmp::max(SumPos::new(pos, x), SumPos::empty(pos + 1)),
				prefix: result.clone(),
				middle: result,
			}
		},
	);

	writeln!(writer, "{}", tree.f_for_segment(0..tree.on().len()).unwrap().middle.sum).unwrap();

	for _ in 0..m {
		let i = read!(usize);
		let v = read!(i64);
		tree.set(i, v);
		let result = tree
			.f_for_segment(0..tree.on().len())
			.map(|x| x.middle.sum)
			.unwrap_or(0);
		writeln!(writer, "{}", result).unwrap();
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/option.rs");
include!("../../../../templates/src/to_include/segment_tree.rs");
