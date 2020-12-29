#[derive(Debug)]
struct Tree {
	leaf_max_h: usize,
	leaf_min_h: usize,
	childs: Vec<Tree>,
}

fn make_tree(prev: usize, current: usize, edges: &[Vec<usize>]) -> Tree {
	let childs = edges[current]
		.iter()
		.filter(|next| **next != prev)
		.map(|next| make_tree(current, *next, edges))
		.collect::<Vec<Tree>>();
	let leaf_max_h = childs.iter().map(|tree| tree.leaf_max_h + 1).max().unwrap_or(0);
	let leaf_min_h = childs.iter().map(|tree| tree.leaf_min_h + 1).min().unwrap_or(0);
	Tree { leaf_max_h, leaf_min_h, childs }
}

#[derive(Debug)]
struct DfsResult {
	current_path_len: usize,
	max_k: usize,
}

fn dfs(is_first: bool, mut tree: Tree) -> DfsResult {
	if tree.childs.is_empty() {
		DfsResult { current_path_len: 1, max_k: 1 }
	} else if tree.childs.len() == 1 {
		let result = dfs(false, tree.childs.remove(0));
		DfsResult { current_path_len: result.current_path_len + 1, max_k: result.max_k }
	} else {
		let results = tree.childs.into_iter().map(|tree| dfs(false, tree)).collect::<Vec<_>>();
		let mut max_k = results.iter().map(|t| t.max_k).max().unwrap();
		let mut paths = results.into_iter().map(|x| x.current_path_len).collect::<Vec<_>>();
		if is_first {
			paths.sort_unstable_by_key(|x| Reverse(*x));
		} else {
			paths.sort_unstable();
		}
		max_k = std::cmp::max(max_k, paths.iter().skip(1).map(|x| x + 1).max().unwrap());
		DfsResult { current_path_len: paths[0] + 1, max_k }
	}
}

fn solve(edges: Vec<Vec<usize>>) -> usize {
	let tree = make_tree(0, 0, &edges);
	let result = dfs(true, tree);
	std::cmp::max(result.current_path_len - 1, result.max_k)
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let mut edges: Vec<Vec<usize>> = vec![Vec::with_capacity(3); n];
		for _ in 0..n - 1 {
			let mut u = read!(usize);
			let mut v = read!(usize);
			u -= 1;
			v -= 1;
			edges[u].push(v);
			edges[v].push(u);
		}
		println!("{}", solve(edges));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
