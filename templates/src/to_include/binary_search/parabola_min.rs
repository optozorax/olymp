fn parabola_min<T: Ord + Sub<Output = T> + From<u8>, F: Fn(usize) -> T>(on: Range<usize>, f: F) -> Option<(usize, T)> {
	let a = on.start;
	let b = on.end;
	match b.checked_sub(a)? {
		0 => None,
		1 => Some((a, f(a))),
		_ => {
			let va = f(a);
			let vb = f(b - 1);
			let middle = binary_search(on.start..on.end - 1, |i| f(i) - f(i + 1) < T::from(0)).unwrap_or(b - 1);

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