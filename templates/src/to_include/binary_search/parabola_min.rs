fn parabola_min<T: Ord + Debug, F: Fn(usize) -> T>(on: Range<usize>, f: F) -> Option<(usize, T)> {
	#[cfg(debug_assertions)]
	{
		let mut rising = false;
		for i in on.start..on.end-1 {
			let v1 = f(i);
			let v2 = f(i+1);
			if v1 == v2 {
				panic!("parabola_min guarenties violated f({}) == f({}) == {:?}, try using parabola_eq_min", i, i+1, v1);
			}
			if rising {
				if v1 > v2 {
					panic!("parabola_min guarantries violated, previos function is rising, but now is not: f({}) = {:?} > f({}) = {:?}", i, v1, i+1, v2);
				}
			} else {
				if v1 < v2 {
					rising = true;
				}
			}
		}
	}
	let a = on.start;
	let b = on.end;
	match b.checked_sub(a)? {
		0 => None,
		1 => Some((a, f(a))),
		_ => {
			let va = f(a);
			let vb = f(b - 1);

			let middle = binary_search(on.start..on.end - 1, |i| f(i) < f(i + 1)).unwrap_or(b - 1);

			let mut vmiddle = f(middle);
			let mut result = middle;

			if va < vmiddle {
				vmiddle = va;
				result = a;
			}
			if vb < vmiddle {
				vmiddle = vb;
				result = b - 1;
			}

			Some((result, vmiddle))
		},
	}
}