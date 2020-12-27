fn solve(arr: &[u32]) -> Option<(usize, usize)> {
	let mut z_maxs = Vec::with_capacity(arr.len());
	let mut current_max = *arr.last().unwrap();
	for i in arr.iter().rev().skip(1).take(arr.len() - 2) {
		z_maxs.push(current_max);
		current_max = std::cmp::max(current_max, *i);
	}

	let min_tree = Rmq::create(arr, RmqType::Min);
	let mut current_max = arr[0];
	for (x_size, i) in arr.iter().enumerate().skip(1).take(arr.len() - 2) {
		if let Some(min_range) = binary_search_number_range(x_size..arr.len() - 1, current_max, |pos| {
			min_tree.most_for_segment(x_size..pos + 1)
		}) {
			if let Some(final_y_pos) =
				binary_search_number_range(min_range, current_max, |pos| z_maxs[arr.len() - pos - 2])
			{
				let y_size = final_y_pos.start - x_size + 1;
				return Some((x_size, y_size));
			}
		}
		current_max = std::cmp::max(current_max, *i);
	}

	None
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let vec = readln!(u32);
		if let Some((x, y)) = solve(&vec) {
			println!("YES");
			println!("{} {} {}", x, y, n - (x + y));
		} else {
			println!("NO");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
include!("../../../../templates/src/to_include/binary_search/number.rs");
include!("../../../../templates/src/to_include/binary_search/range.rs");
include!("../../../../templates/src/to_include/binary_function.rs");
include!("../../../../templates/src/to_include/sparse_table/base.rs");
include!("../../../../templates/src/to_include/sparse_table/rmq.rs");
