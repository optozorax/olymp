fn parabola_eq_min<T: Ord + Debug, F: Fn(usize) -> T>(on: Range<usize>, f: F) -> Option<(usize, T)> {
	#[cfg(debug_assertions)]
	{
		let mut rising = false;
		let mut i = on.start;
		'out: while i < on.end {
			let v1 = f(i);
			let v2 = loop {
				i += 1;
				if i >= on.end {
					break 'out;
				}
				let v2 = f(i);
				if v2 != v1 {
					break v2;
				}
			};
			if rising {
				if v1 > v2 {
					panic!("parabola_eq_min guarantries violated, previos function is rising, but now is not: f({}) = {:?} > f({}) = {:?}", i, v1, i+1, v2);
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
			todo!()
		},
	}
}