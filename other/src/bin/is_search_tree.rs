#[derive(Eq, PartialEq, Debug)]
struct Tree { 
	value: i32,
	left: Option<Box<Tree>>,
	right: Option<Box<Tree>>,
}

#[derive(Eq, PartialEq, Debug)]
struct TreeResult {
	min: i32,
	max: i32,
}

fn is_search_tree(root: Tree) -> Option<TreeResult> {
	let mut result = TreeResult { min: root.value, max: root.value };

	if let Some(left) = root.left {
		let TreeResult { min, max } = is_search_tree(*left)?;
		if root.value < max { return None; }
		result.min = result.min.min(min);
	}

	if let Some(right) = root.right {
		let TreeResult { min, max } = is_search_tree(*right)?;
		if root.value > min { return None; }
		result.max = result.max.max(max);
	}

	Some(result)
}

fn main() {
    assert_eq!(is_search_tree(
        Tree {
            value: 8,
            left: Some(Box::new(Tree {
                value: 3,
                left: None,
                right: None
            })),
            right: Some(Box::new(Tree {
                value: 10,
                left: None,
                right: None
            })),
        }
    ), Some(TreeResult { min: 3, max: 10 }));
}