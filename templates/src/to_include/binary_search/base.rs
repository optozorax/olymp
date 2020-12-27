// It is supposed that `f` returns `false` `n` times, then `true` `n` times. This method uses binary search to find `n`. This function calls `f` only on range [on.start; on.end).
fn binary_search<F: Fn(usize) -> bool>(on: Range<usize>, f: F) -> Option<usize> {
	let mut a = on.start;
	let mut b = on.end;
	match b.checked_sub(a)? {
		0 => None,
		1 => Some(a).filter(|&x| f(x)),
		_ => if f(a) {
			Some(a)
		} else if !f(b-1) {
			None
		} else {
			b -= 1;
			loop {
				let c = a+(b-a)/2;
				match b-a {
					0 => unreachable!(),
					1 => return Some(b),
					2 => return Some(if f(c) { c } else { b }),
					_ => if f(c) { b = c; } else { a = c; }
				}
			}
		}
	}
}
