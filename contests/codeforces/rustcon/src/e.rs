#[derive(Debug, Clone)]
enum Expr {
	Const(u8),
	Func(u8, Vec<Expr>),
}

fn parse_expr(s: &[u8]) -> Expr {
	if s.len() == 1 {
		Expr::Const(s[0])
	} else if b'a' <= s[0] && s[0] <= b'z' {
		let name = s[0];
		let mut params = Vec::new();

		let mut br = 0;
		let mut j = 2;
		for i in 2..s.len() - 2 {
			match s[i] {
				b',' if br == 0 => {
					params.push(parse_expr(&s[j..i]));
					j = i + 1;
				}
				b'(' => br += 1,
				b')' => br -= 1,
				_ => {}
			}
		}
		params.push(parse_expr(&s[j..s.len() - 1]));

		Expr::Func(name, params)
	} else {
		unreachable!()
	}
}

fn recur(expr: &Expr, ans: &mut Vec<String>) -> String {
	use Expr::*;
	match expr {
		Const(a) => return format!("{}", char::from(*a)),
		Func(f, exprs) => {
			let exprs = exprs.iter().map(|x| recur(x, ans)).collect::<Vec<_>>().join(",");
			let result = format!("{}({})", char::from(*f), exprs);
			ans.push(result);
			return format!("_{}", ans.len());
		}
	}
}

#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	let expr = parse_expr(&s[6..s.len() - 1]);
	let mut ans: Vec<String> = Vec::new();
	let last = recur(&expr, &mut ans);
	for (i, s) in ans.iter().enumerate() {
		println!("_{}={}", i + 1, s);
	}
	println!("print({})", last);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
