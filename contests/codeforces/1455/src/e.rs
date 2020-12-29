static mut XS: *mut Vec<i64> = 0 as _;
static mut YS: *mut Vec<i64> = 0 as _;

fn init() {
	let mut xs = Box::new(vec![0i64; 4]);
	unsafe {
		XS = &mut *xs;
	}
	std::mem::forget(xs);

	let mut ys = Box::new(vec![0i64; 4]);
	unsafe {
		YS = &mut *ys;
	}
	std::mem::forget(ys);
}

fn cost(points: &[(i64, i64)], k: i64) -> i64 {
	points
		.iter()
		.enumerate()
		.map(|(index, (x, _))| if [1, 3].contains(&index) { x - k } else { *x })
		.zip(unsafe { (*XS).iter_mut() })
		.for_each(|(value, x)| *x = value);
	points
		.iter()
		.enumerate()
		.map(|(index, (_, y))| if [2, 3].contains(&index) { y - k } else { *y })
		.zip(unsafe { (*YS).iter_mut() })
		.for_each(|(value, y)| *y = value);
	let opt_x = median(unsafe { &mut (*XS) });
	let opt_y = median(unsafe { &mut (*YS) });

	let square = [(opt_x, opt_y), (opt_x + k, opt_y), (opt_x, opt_y + k), (opt_x + k, opt_y + k)];

	points
		.iter()
		.zip(square.iter())
		.map(|((x, y), (x1, y1))| (x - x1).abs() + (y - y1).abs())
		.sum()
}

fn ternary_search<F: Fn(i64) -> i64>(mut left: i64, mut right: i64, f: F) -> i64 {
	while (right - left).abs() > 3 {
		let a = (left * 2 + right) / 3;
		let b = (left + right * 2) / 3;
		if f(a) < f(b) {
			right = b;
		} else {
			left = a;
		}
	}
	(left..=right).map(|x| (x, f(x))).min_by_key(|(_, x)| *x).unwrap().0
}

fn delta_binary_search<F: Fn(i64) -> i64>(mut a: i64, mut b: i64, f: F) -> i64 {
	while a < b {
		let c = (a + b) / 2;
		if f(c) < f(c + 1) {
			b = c;
		} else {
			a = c + 1;
		}
	}
	a
}

fn solve(mut points: &mut [(i64, i64)]) -> i64 {
	let iter = points.iter().map(|(x, y)| vec![x, y].into_iter()).flatten();
	let start = iter.clone().min().unwrap();
	let end = iter.clone().max().unwrap();
	let max_size = end - start;
	let mut min = cost(&points, 0);
	permutations(&mut points, |a| {
		min = std::cmp::min(cost(a, delta_binary_search(0, max_size, |pos| cost(a, pos))), min);
	});
	min
}

#[fastio::fastio]
pub fn main() {
	init();
	let t = read!(usize);
	for _ in 0..t {
		let mut points = Vec::new();
		for _ in 0..4 {
			let x = read!(i64);
			let y = read!(i64);
			points.push((x, y));
		}
		println!("{}", solve(&mut points));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/median.rs");
include!("../../../../templates/src/to_include/permutations.rs");
