#[derive(Debug, Clone)]
pub enum QuadraticSolvingResult {
	Two(f64, f64),
	One(f64),
	Zero,
}

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> QuadraticSolvingResult {
	use QuadraticSolvingResult::*;
	let d = b*b - 4.0*a*c;
	if d < 0.0 {
		Zero
	} else if d.abs() < 1e-9 {
		One(-b/(2.0*a))
	} else {
		let sq_d = d.sqrt();
		let x1 = (-b + sq_d)/(2.0*a);
		let x2 = (-b - sq_d)/(2.0*a);
		Two(
			x1.max(x2),
			x1.min(x2),
		)
	}
}
