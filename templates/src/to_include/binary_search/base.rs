// It is supposed that `f` returns `false` `n` times, then `true` `m` times. This method uses binary search to find `n`. This function calls `f` only on range [on.start; on.end).
fn binary_search<F: Fn(usize) -> bool>(on: Range<usize>, f: F) -> Option<usize> {
	#[cfg(debug_assertions)]
	{
		let mut met_true = false;
		for i in on.clone() {
			let v = f(i);
			if met_true && !v {
				panic!("binary_search guarantees (true only after false) violated on position: `{}`", i);
			}
			met_true |= v;
		}
	}
	let mut a = on.start;
	let mut b = on.end;

	// Old code
	// match b.checked_sub(a)? {
	// 	0 => None,
	// 	1 => Some(a).filter(|&x| f(x)),
	// 	_ => if f(a) {
	// 		Some(a)
	// 	} else if !f(b-1) {
	// 		None
	// 	} else {
	// 		b -= 1;
	// 		loop {
	// 			let c = a+(b-a)/2;
	// 			match b-a {
	// 				0 => unreachable!(),
	// 				1 => return Some(b),
	// 				2 => return Some(if f(c) { c } else { b }),
	// 				_ => if f(c) { b = c; } else { a = c; }
	// 			}
	// 		}
	// 	}
	// }

	// New code, more straightforward
	if b-a == 0 {
		return None;
	}
	if f(a) {
		return Some(a);
	}
	if !f(b-1) {
		return None;
	}
	while b-a != 1 {
		let c = (a+b)/2;
		if f(c) {
			b = c;
		} else {
			a = c;
		}
	}
	if f(a) {
		Some(a)
	} else if b < on.end && f(b) {
		Some(b)
	} else {
		None
	}
}
