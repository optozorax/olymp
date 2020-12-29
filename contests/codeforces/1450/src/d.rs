fn solve(a: &[usize]) -> Vec<bool> {
	let mut a_in;
	a_in = a.iter().copied().enumerate().collect::<Vec<_>>();
	a_in.sort_unstable_by_key(|(_, v)| *v);
	let pos;
	let mut result;
	pos = a_in
		.iter()
		.enumerate()
		.find(|(index, (_original, value))| index + 1 != *value)
		.map(|(index, _)| index)
		.unwrap_or(a.len());
	result = Vec::with_capacity(a.len());

	let rmq = Rmq::create(a, RmqType::Min);

	let mut min = a.len() + 100;
	for (current_pos, current_elem) in a_in.into_iter().take(pos) {
		let r = find_max_segment_with_this_number_as_min(0..a.len(), current_elem, current_pos, |r| {
			rmq.most_for_segment(r)
		});
		min = std::cmp::min(min, r.end - 1 - r.start);
		result.push(min >= a.len() - current_elem);
	}

	result.resize(a.len(), false);
	result.reverse();

	result
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }
	#[rustfmt::skip] macro_rules! flush { ($($x:tt)*) => { writer.flush().unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
	#[rustfmt::skip] macro_rules! byte { () => { scanner.byte() }; }
	#[rustfmt::skip] macro_rules! bytes { () => { scanner.bytes() }; }
	// -------------------------------------------------------------------- //

	let t = read!(i64);
	for _ in 0..t {
		let _n = read!(i64);
		let a = readln!(usize);
		println!(
			"{}",
			solve(&a)
				.into_iter()
				.map(|x| if x { '1' } else { '0' })
				.collect::<String>()
		);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_function.rs");
include!("../../../../templates/src/to_include/sparse_table/base.rs");
include!("../../../../templates/src/to_include/sparse_table/rmq.rs");
include!("../../../../templates/src/to_include/sparse_table/find_most_segment.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
