fn intersect(a1: &(f64, f64), a2: &(f64, f64), b1: &(f64, f64), b2: &(f64, f64)) -> Option<(f64, f64)> {
	let ka = a2.1 - a1.1;
	let ba = a1.1 - a1.0 * ka;

	let kb = b2.1 - b1.1;
	let bb = b1.1 - b1.0 * kb;

	if (ka - kb).abs() < 0.0000001 {
		return None;
	}

	let x = (bb - ba) / (ka - kb);

	if a1.0 < x && x < a2.0 {
		Some((x, ka * x + ba))
	} else {
		None
	}
}

fn s(p: &[(f64, f64)]) -> f64 {
	let mut sum: f64 = 0.0;
	for i in 0..p.len() - 1 {
		sum += p[i].0 * p[i + 1].1;
	}
	sum += p[p.len() - 1].0 * p[0].1;
	for i in 0..p.len() - 1 {
		sum -= p[i + 1].0 * p[i].1;
	}
	sum -= p[0].0 * p[p.len() - 1].1;

	sum.abs() / 2.
}

#[fastio::fastio]
pub fn main() {
	let n = read!(usize);
	let a1 = {
		let mut a1 = vec![(0., 0.)];
		for x in 1..=n {
			let y = read!(f64);
			a1.push((x as f64, y));
		}
		a1.push((n as f64 + 1., 0.));
		a1
	};
	let a2 = {
		let mut a2 = vec![(0., 0.)];
		for x in 1..=n {
			let y = read!(f64);
			a2.push((x as f64, y));
		}
		a2.push((n as f64 + 1., 0.));
		a2
	};

	let mut sum: f64 = 0.;
	for (a, b) in a1.windows(2).zip(a2.windows(2)) {
		if let Some(pos) = intersect(&a[0], &a[1], &b[0], &b[1]) {
			sum += s(&[a[0], pos, b[0]]);
			sum += s(&[a[1], pos, b[1]]);
		} else {
			sum += s(&a.iter().chain(b.iter().rev()).cloned().collect::<Vec<_>>());
		}
	}
	println!("{:.6}", sum);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
