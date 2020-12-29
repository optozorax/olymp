fn solve(mut s: Vec<char>, a: &[u8]) -> String {
	s.sort_unstable();

	if s == ['+', '-'] {
		s = vec!['+'];
	}
	if s == ['*', '+', '-'] {
		s = vec!['*', '+'];
	}

	if s.len() == 1 {
		return a.iter().joined_by(s[0]).to_string();
	}

	if s == ['*', '-'] {
		let first_zero = a.iter().position(|x| *x == 0).unwrap_or(a.len());
		let before_zero = a[..first_zero].iter().joined_by('*').to_string();
		let after_zero = a[first_zero..].iter().joined_by('-').to_string();
		return before_zero + &after_zero;
	}

	if s == ['*', '+'] {
		return a.iter().joined_by('!').to_string();
	}

	unreachable!()
	// TODO
}

#[fastio::fastio]
pub fn main() {
	let _n = read!(i64);
	let a = readln!(u8);
	let s = bytes!().into_iter().map(char::from).collect::<Vec<_>>();
	println!("{}", solve(s, &a));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/joined_by.rs");
