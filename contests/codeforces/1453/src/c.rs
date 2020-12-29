fn is_two_some<A, B, C>(a: Option<A>, b: Option<B>, c: Option<C>) -> bool {
	(a.is_some() && b.is_some()) || (a.is_some() && c.is_some()) || (b.is_some() && c.is_some())
}

fn solve(n: usize, a: Vec<Vec<u8>>) -> Vec<usize> {
	fn inner<X: Fn(usize, usize) -> bool, Y: Fn(usize, usize) -> bool>(n: usize, fx: X, fy: Y) -> usize {
		let xs = (0..n)
			.map(|pos| (0..n).map(|inpos| fx(pos, inpos)).position(|b| b))
			.collect::<Vec<_>>();
		let ys = (0..n)
			.map(|pos| (0..n).map(|inpos| fy(pos, inpos)).position(|b| b))
			.collect::<Vec<_>>();
		let mut max = 0;
		for x in 0..n {
			for y in 0..n {
				let current = Some(()).filter(|_| fx(x, y));
				let xs = xs[x].filter(|xs| *xs < y);
				let ys = ys[y].filter(|ys| *ys < x);
				if let Some((xs, ys)) = (|| -> Option<_> { Some((xs?, ys?)) })() {
					max = std::cmp::max(max, (x - ys) * (n - 1 - xs));
					max = std::cmp::max(max, (x - ys) * (y - xs));
					max = std::cmp::max(max, (n - 1 - ys) * (y - xs));
				}
				if let Some(((), xs)) = (|| -> Option<_> { Some((current?, xs?)) })() {
					max = std::cmp::max(max, x * (y - xs));
					max = std::cmp::max(max, (n - 1 - x) * (y - xs));
				}
				if let Some(((), ys)) = (|| -> Option<_> { Some((current?, ys?)) })() {
					max = std::cmp::max(max, (x - ys) * y);
					max = std::cmp::max(max, (x - ys) * (n - 1 - y));
				}
			}
		}
		max
	}

	let at = (0..n)
		.map(|y| (0..n).map(|x| a[x][y]).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let mut result = Vec::with_capacity(10);
	for d in 0..=9 {
		let mut max = 0;
		max = std::cmp::max(max, inner(n, |x, y| at[x][y] == d, |y, x| a[y][x] == d));
		max = std::cmp::max(max, inner(n, |x, y| at[n - 1 - x][y] == d, |y, x| a[y][n - 1 - x] == d));
		max = std::cmp::max(max, inner(n, |x, y| at[n - 1 - x][n - 1 - y] == d, |y, x| a[n - 1 - y][n - 1 - x] == d));
		max = std::cmp::max(max, inner(n, |x, y| at[x][n - 1 - y] == d, |y, x| a[n - 1 - y][x] == d));
		result.push(max);
	}
	result
}

#[fastio::fastio]
pub fn main() {
	let t = read!(i64);
	for _ in 0..t {
		let n = read!(i64);
		let mut arr = Vec::with_capacity(n as usize);
		for _ in 0..n {
			let c = bytes!().into_iter().map(|c| c - b'0').collect::<Vec<u8>>();
			arr.push(c);
		}
		println!("{}", SpaceVec(solve(n as usize, arr)));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
