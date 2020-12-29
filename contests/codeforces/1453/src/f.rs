#[derive(Clone, Debug)]
struct DpElem {
	offset: usize,
	suffix_min: usize,
}

#[derive(Clone, Debug)]
struct DpPredElem {
	offset: usize,
	current_min: usize,
}

#[derive(Clone, Debug)]
struct FullDp {
	can_win: bool,
	dp: Vec<DpElem>,
}

fn calc_suffix_mins(mut input: Vec<DpPredElem>) -> Vec<DpElem> {
	let mut min = input.last().map(|x| x.current_min).unwrap_or(0);
	for i in input.iter_mut().rev() {
		min = std::cmp::min(min, i.current_min);
		i.current_min = min;
	}
	input
		.into_iter()
		.map(|elem| DpElem { offset: elem.offset, suffix_min: elem.current_min })
		.collect()
}

fn solve(a: &[usize]) -> usize {
	let mut dp: Vec<FullDp> = vec![FullDp { can_win: false, dp: vec![] }; a.len()];
	for (index, elem) in a.iter().enumerate().rev() {
		let mut current_dp: Vec<DpPredElem> = Vec::new();
		let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
		for (offset, in_elem, in_dp) in a
			.iter()
			.skip(index + 1)
			.take(*elem)
			.enumerate()
			.map(|(offset, in_elem)| (offset + 1, in_elem, dp.get(index + offset + 1).unwrap()))
			.filter(|(_, _, in_dp)| in_dp.can_win)
		{
			while heap.peek().map(|elem| elem.0 <= offset).unwrap_or(false) {
				heap.pop().unwrap();
			}

			let need_to_delete_before = heap.len();
			if let Some(index_need_to_delete_after) =
				binary_search(0..in_dp.dp.len(), |pos| in_dp.dp[pos].offset + offset > *elem)
			{
				let need_to_delete_after = in_dp.dp[index_need_to_delete_after].suffix_min;
				current_dp.push(DpPredElem { offset, current_min: need_to_delete_after + need_to_delete_before });
			}
			heap.push(Reverse(offset + in_elem + 1));
		}
		if index == a.len() - 1 {
			dp[index] = FullDp { can_win: true, dp: vec![DpElem { offset: 1, suffix_min: 0 }] };
		} else {
			dp[index] =
				FullDp { can_win: index == a.len() - 1 || !current_dp.is_empty(), dp: calc_suffix_mins(current_dp) };
		}
	}
	dp[0].dp[0].suffix_min
}

#[fastio::fastio]
pub fn main() {
	let t = read!(i64);
	for _ in 0..t {
		let _n = read!(usize);
		let a = readln!(usize);
		println!("{}", solve(&a));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
