struct TravelInputTree<Y> {
    current_value: Y,
    left_value: Y,
    right_value: Y,
    on: Range<usize>,
}

struct TravelInputLeaf<Y, T> {
    current_value_lifted: Y,
    current_value: T,
    pos: usize,
}

enum TravelResultTree<S, R> {
    /// Go to left subtree, and get return value from there
    Left(S),
    /// Go to right subtree, and get return value from there
    Right(S),

    /// Return the value
    Return(Option<R>),
}

trait SegmentTreeTraveller<Y, T>: Sized {
    type Return;

    fn visit_tree(
        self,
        current_value: Y,
        left_subtree_value: Y,
        right_subtree_value: Y,
        on: Range<usize>, // TODO remove this, this is useless
    ) -> TravelResultTree<Self, Self::Return>;

    fn visit_leaf(
        self,
        current_value_lifted: Y,
        current_value: T, // TODO i think this is useless too
        pos: usize,
    ) -> Option<Self::Return>;
}

impl<'a, T: Clone, Y: Clone, F: Fn(Range<usize>, Y, Y) -> Y, FY: Fn(usize, T) -> Y>
    SegmentTree<'a, T, Y, F, FY>
{
    fn travel_helper<S: SegmentTreeTraveller<Y, T>>(&self, pos: usize, start: S) -> Option<S::Return> {
        unimplemented!()
    }

    // TODO it must can travel on any subsegment
    /// Calculate some function with state on tree, by time O(log(n))
    pub fn travel<S: SegmentTreeTraveller<Y, T>>(&self, start: S) -> Option<S::Return> {
        unimplemented!()
    }
}

#[fastio::fastio]
pub fn main() {
	let _n = read!(i64);
	let m = read!(i64);
	let mut a = readln!(i64);
	let mut tree = SegmentTree::create(&mut a, |_on, s1: i64, s2: i64| s1 + s2, |_pos, x| x);

    struct KthUnitFinder {
        k: usize,
    }

    impl SegmentTreeTraveller<i64, i64> for KthUnitFinder {
        type Return = usize;

        fn visit_tree(
            self,
            current_value: i64,
            left_subtree_value: i64,
            _right_subtree_value: i64,
            _on: Range<usize>,
        ) -> TravelResultTree<Self, Self::Return> {
            if current_value > self.k as i64 {
                TravelResultTree::Return(None)
            } else {
                match left_subtree_value.cmp(&(self.k as i64)) {
                    Ordering::Equal | Ordering::Less => TravelResultTree::Right(Self {
                        k: self.k - left_subtree_value as usize,
                    }),
                    Ordering::Greater => TravelResultTree::Left(self),
                }
            }
        }

        fn visit_leaf(
            self,
            _current_value_lifted: i64,
            current_value: i64,
            pos: usize,
        ) -> Option<Self::Return> {
            if current_value == 1 {
                Some(pos)
            } else {
                None
            }
        }
    }

    let k = 5;
    tree.travel(KthUnitFinder { k }).unwrap();

    writeln!(
        writer,
        "{}",
        tree.f_for_segment(0..tree.on().len()).unwrap()
    )
    .unwrap();

    for _ in 0..m {
    	let i = read!(usize);
    	let v = read!(i64);
        tree.set(i, v);
        let result = tree.f_for_segment(0..tree.on().len()).unwrap_or(0);
        writeln!(writer, "{}", result).unwrap();
    }
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/option.rs");
include!("../../../../templates/src/to_include/segment_tree.rs");

