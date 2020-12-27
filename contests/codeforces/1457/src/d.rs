fn solve(a: &[u64]) -> Option<usize> {
	for i in 0..a.len() - 1 {
		let result = a[i] ^ a[i + 1];
		if i.checked_sub(1)
			.and_then(|x| a.get(x))
			.map(|x| *x > result)
			.unwrap_or(false)
			|| a.get(i + 2).map(|x| *x < result).unwrap_or(false)
		{
			return Some(1);
		}
	}

	// else size of array is 64

	let mut answer: Option<usize> = None;
	for i in 0..a.len() {
		for j in i + 1..=a.len() {
			let xored = a[i..j].iter().fold(1, |acc, x| acc ^ x) ^ 1;

			let mut result1 = None;
			for i1 in j..a.len() {
				for j1 in i1 + 1..=a.len() {
					let xored1 = a[i1..j1].iter().fold(1, |acc, x| acc ^ x) ^ 1;
					if xored > xored1 {
						result1 = match result1 {
							Some(a) => Some(std::cmp::min(a, j1 - i1 - 1)),
							None => Some(j1 - i1 - 1),
						};
					}
				}
			}

			if let Some(sum) = result1 {
				answer = match answer {
					Some(a) => Some(std::cmp::min(a, j - i - 1 + sum)),
					None => Some(j - i - 1 + sum),
				};
			}

			let mut result2 = None;
			for i1 in 0..i {
				for j1 in i1 + 1..=i {
					let xored1 = a[i1..j1].iter().fold(1, |acc, x| acc ^ x) ^ 1;
					if xored < xored1 {
						result2 = match result2 {
							Some(a) => Some(std::cmp::min(a, j1 - i1 - 1)),
							None => Some(j1 - i1 - 1),
						};
					}
				}
			}

			if let Some(sum) = result2 {
				answer = match answer {
					Some(a) => Some(std::cmp::min(a, j - i - 1 + sum)),
					None => Some(j - i - 1 + sum),
				};
			}
		}
	}

	answer
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

	let _n = read!(usize);
	let a = readln!(u64);
	if let Some(result) = solve(&a) {
		println!("{}", result);
	} else {
		println!("-1");
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
