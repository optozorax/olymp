#![allow(
    clippy::many_single_char_names,
    unused_macros,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    dead_code,
    unused_imports
)]

include!("to_include/macro/time.rs");
include!("to_include/macro/try_bool.rs");

include!("to_include/imports.rs");

include!("to_include/binary_search/base.rs");
include!("to_include/binary_search/number.rs");
include!("to_include/binary_search/range.rs");
include!("to_include/binary_search/parabola_min.rs");

include!("to_include/display/chars.rs");
include!("to_include/display/joined_by.rs");
include!("to_include/display/lines.rs");
include!("to_include/display/space_matrix.rs");
include!("to_include/display/space_vec.rs");

include!("to_include/for_each_number/base.rs");
include!("to_include/for_each_number/subset.rs");

include!("to_include/iterator/count.rs");
include!("to_include/iterator/duplicate.rs");
include!("to_include/iterator/fst_snd.rs");
include!("to_include/iterator/push_map.rs");
include!("to_include/iterator/rev_position.rs");
include!("to_include/iterator/scan_after.rs");
include!("to_include/iterator/scan_after_first.rs");
include!("to_include/iterator/scan_before.rs");
include!("to_include/iterator/scan_before_first.rs");

include!("to_include/math/ceil_div.rs");
include!("to_include/math/fastmod.rs");
include!("to_include/math/gcd.rs");
include!("to_include/math/lcm.rs");

include!("to_include/sparse_table/base.rs");
include!("to_include/sparse_table/rmq.rs");
include!("to_include/sparse_table/find_most_segment.rs");

include!("to_include/arithmetic_progression.rs");
include!("to_include/binary_function.rs");
include!("to_include/bit64.rs");
include!("to_include/for_each_subslice.rs");
include!("to_include/median.rs");
include!("to_include/mod_arithmetic.rs");
include!("to_include/my_range.rs");
include!("to_include/option.rs");
include!("to_include/palindrome.rs");
include!("to_include/permutations.rs");
include!("to_include/prefix.rs");
include!("to_include/quadratic_equation.rs");
include!("to_include/scanner.rs");
include!("to_include/segment_tree.rs");
include!("to_include/suffix.rs");
include!("to_include/z_function.rs");

#[cfg(test)]
mod prefix_tests {
    use super::*;

    #[test]
    fn sum() {
        let a: Vec<i32> = vec![1, 2, -3, 0, 555, -1, 0, 0];
        let pr_sum = Prefix::new(&a, |x, y| x + y);
        assert_eq!(pr_sum.get_prefix_f(0), None);
        for i in 1..=a.len() {
            assert_eq!(pr_sum.get_prefix_f(i), Some(a[0..i].iter().sum::<i32>()));
        }

        for i in 0..a.len() {
            for j in i + 1..=a.len() {
                assert_eq!(
                    pr_sum.get_segment_f(i..j, |x, y| x - y),
                    Some(a[i..j].iter().sum::<i32>())
                );
            }
        }
    }

    #[test]
    fn xor() {
        let a: Vec<i32> = vec![1, 2, 32, 6, 7, 8, 53, 33];
        let pr_sum = Prefix::new(&a, |x, y| x ^ y);
        assert_eq!(pr_sum.get_prefix_f(0), None);
        for i in 1..=a.len() {
            assert_eq!(
                pr_sum.get_prefix_f(i),
                Some(a[0..i].iter().fold(1, |acc, x| acc ^ x) ^ 1)
            );
        }

        for i in 0..a.len() {
            for j in i + 1..=a.len() {
                assert_eq!(
                    pr_sum.get_segment_f(i..j, |x, y| x ^ y),
                    Some(a[i..j].iter().fold(1, |acc, x| acc ^ x) ^ 1)
                );
            }
        }
    }

    #[test]
    fn min() {
        let a: Vec<i32> = vec![1, 2, -3, 0, 555, -1, 0, 0];
        let pr_sum = Prefix::new(&a, std::cmp::min);
        assert_eq!(pr_sum.get_prefix_f(0), None);
        for i in 1..=a.len() {
            assert_eq!(pr_sum.get_prefix_f(i), a[0..i].iter().copied().min());
        }
    }
}

//----------------------------------------------------------------------------

#[cfg(test)]
mod suffix_tests {
    use super::*;

    #[test]
    fn sum() {
        let a: Vec<i32> = vec![1, 2, -3, 0, 555, -1, 0, 0];
        let sf_sum = Suffix::new(&a, |x, y| x + y);
        assert_eq!(sf_sum.get_suffix_f(a.len()), None);
        for i in 0..a.len() {
            assert_eq!(sf_sum.get_suffix_f(i), Some(a[i..].iter().sum::<i32>()));
        }

        for i in 0..a.len() {
            for j in i + 1..=a.len() {
                dbg!(i, j);
                assert_eq!(
                    sf_sum.get_segment_f(i..j, |x, y| x - y),
                    Some(a[i..j].iter().sum::<i32>())
                );
            }
        }
    }

    #[test]
    fn xor() {
        let a: Vec<i32> = vec![1, 2, 32, 6, 7, 8, 53, 33];
        let sf_sum = Suffix::new(&a, |x, y| x ^ y);
        assert_eq!(sf_sum.get_suffix_f(a.len()), None);
        for i in 0..a.len() {
            assert_eq!(
                sf_sum.get_suffix_f(i),
                Some(a[i..].iter().fold(1, |acc, x| acc ^ x) ^ 1)
            );
        }

        for i in 0..a.len() {
            for j in i + 1..=a.len() {
                assert_eq!(
                    sf_sum.get_segment_f(i..j, |x, y| x ^ y),
                    Some(a[i..j].iter().fold(1, |acc, x| acc ^ x) ^ 1)
                );
            }
        }
    }

    #[test]
    fn min() {
        let a: Vec<i32> = vec![1, 2, -3, 0, 555, -1, 0, 0];
        let sf_sum = Suffix::new(&a, std::cmp::min);
        assert_eq!(sf_sum.get_suffix_f(a.len()), None);
        for i in 0..a.len() {
            assert_eq!(sf_sum.get_suffix_f(i), a[i..].iter().copied().min());
        }
    }
}

#[cfg(test)]
mod tests_binary_search {
    use super::*;

    #[test]
    fn testing() {
        fn test(vec: Vec<bool>) -> Option<usize> {
            binary_search(0..vec.len(), |pos| vec[pos])
        }
        assert_eq!(test(vec![]), None);
        assert_eq!(test(vec![false]), None);
        assert_eq!(test(vec![true]), Some(0));
        assert_eq!(test(vec![false, false]), None);
        assert_eq!(test(vec![false, true]), Some(1));
        assert_eq!(test(vec![true, true]), Some(0));
        assert_eq!(test(vec![false, false, true]), Some(2));
        assert_eq!(test(vec![false, true, true]), Some(1));
        assert_eq!(test(vec![true, true, true]), Some(0));
        assert_eq!(test(vec![false, false, false, false]), None);
        assert_eq!(test(vec![false, false, false, true]), Some(3));
        assert_eq!(test(vec![false, false, true, true]), Some(2));
        assert_eq!(test(vec![false, true, true, true]), Some(1));
        assert_eq!(test(vec![true, true, true, true]), Some(0));
        assert_eq!(test(vec![false, false, false, false, false]), None);
        assert_eq!(test(vec![false, false, false, false, true]), Some(4));
        assert_eq!(test(vec![false, false, false, true, true]), Some(3));
        assert_eq!(test(vec![false, false, true, true, true]), Some(2));
        assert_eq!(test(vec![false, true, true, true, true]), Some(1));
        assert_eq!(test(vec![true, true, true, true, true]), Some(0));
    }
}

#[cfg(test)]
mod tests_binary_search_number_range {
    use super::*;

    #[test]
    fn testing() {
        fn test_number(arr: &[u32], number: u32) {
            let result = binary_search_number_leftmost(0..arr.len(), number, |x| arr[x]);
            let start = arr
                .iter()
                .enumerate()
                .find(|(_, value)| **value == number)
                .map(|(index, _)| index);
            if result != start {
                dbg!(arr, result, start);
                panic!();
            }
        }
        fn test_range(arr: &[u32], number: u32) {
            let result1 = binary_search_number_range(0..arr.len(), number, |x| arr[x]);
            let start = arr
                .iter()
                .enumerate()
                .find(|(_, value)| **value == number)
                .map(|(index, _)| index);
            let end = arr
                .iter()
                .enumerate()
                .rev()
                .find(|(_, value)| **value == number)
                .map(|(index, _)| index);
            let result2 = start.zip(end).map(|(a, b)| a..b + 1);
            if result1 != result2 {
                dbg!(arr, result1, result2);
                panic!();
            }
        }
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..200000 {
            let mut arr = (0..rng.gen_range(0, 20))
                .map(|_| rng.gen_range(0, 10))
                .collect::<Vec<u32>>();
            arr.sort_unstable();
            if rng.gen() {
                arr.reverse();
            }
            let number = rng.gen_range(0, 10);
            test_number(&arr, number);
            test_range(&arr, number);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ArithmeticProgresion;

    #[test]
    fn canonical() {
        let ar = ArithmeticProgresion::new_canonical();

        assert_eq!(ar.sum_to(0), 0);
        assert_eq!(ar.sum_to(1), 1);
        assert_eq!(ar.sum_to(2), 3);
        assert_eq!(ar.sum_to(3), 6);
        assert_eq!(ar.sum_to(100), 5050);

        assert_eq!(ar.from_sum(0), 0);
        assert_eq!(ar.from_sum(1), 1);
        assert_eq!(ar.from_sum(2), 1);
        assert_eq!(ar.from_sum(3), 2);
        assert_eq!(ar.from_sum(4), 2);
        assert_eq!(ar.from_sum(5), 2);
        assert_eq!(ar.from_sum(6), 3);
        assert_eq!(ar.from_sum(5050), 100);
        assert_eq!(ar.from_sum(5055), 100);
        assert_eq!(ar.from_sum(5100), 100);

        for sum in 0..1000 {
            let n = ar.from_sum(sum);
            assert!(ar.sum_to(n) <= sum);
            assert!(sum < ar.sum_to(n + 1));
        }
    }

    #[test]
    fn complex() {
        let ar = ArithmeticProgresion::new(5, 10);

        assert_eq!(ar.nth(0), 5);
        assert_eq!(ar.nth(1), 15);
        assert_eq!(ar.nth(2), 25);
        assert_eq!(ar.nth(3), 35);

        assert_eq!(ar.sum_to(0), 0);
        assert_eq!(ar.sum_to(1), ar.nth(0));
        assert_eq!(ar.sum_to(2), ar.nth(0) + ar.nth(1));
        assert_eq!(ar.sum_to(10), (0..10).map(|x| ar.nth(x)).sum());

        assert_eq!(ar.from_sum(0), 0);
        assert_eq!(ar.from_sum(4), 0);
        assert_eq!(ar.from_sum(5), 1);
        assert_eq!(ar.from_sum(10), 1);
        assert_eq!(ar.from_sum(16), 1);

        for sum in 0..1000 {
            let n = ar.from_sum(sum);
            assert!(ar.sum_to(n) <= sum);
            assert!(sum < ar.sum_to(n + 1));
        }
    }
}

#[cfg(test)]
mod tests_parabola_min {
    use super::*;

    #[test]
    fn testing() {
        fn test(vec: Vec<i64>) -> Option<(usize, i64)> {
            parabola_min(0..vec.len(), |pos| vec[pos])
        }
        assert_eq!(test(vec![]), None);
        assert_eq!(test(vec![11, 11, 11, 8, 8, 8, 8, 14, 14]), Some((3, 8)));
        assert_eq!(test(vec![4, 5, 6]), Some((0, 4)));
        assert_eq!(test(vec![4, 4, 4, 5, 6]), Some((2, 4)));
        assert_eq!(test(vec![6, 5, 4]), Some((2, 4)));
        assert_eq!(test(vec![6, 5, 5, 5, 4, 4, 4]), Some((4, 4)));
    }
}


#[cfg(test)]
mod tests_median {
    use super::*;

    #[test]
    fn test() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..200000 {
            let mut arr = (1..rng.gen_range(2, 50))
                .map(|_| rng.gen_range(0, 10))
                .collect::<Vec<i64>>();

            let brute_force_answer: i64 = (-5..35)
                .map(|x| (x, arr.iter().map(|y| (x - y).abs()).sum::<i64>()))
                .min_by_key(|(_, value)| *value)
                .unwrap()
                .1;

            let m = median(&mut arr);
            let answer = arr.iter().map(|y| (m - y).abs()).sum::<i64>();

            if answer != brute_force_answer {
                dbg!(arr, brute_force_answer, answer, m);
                panic!();
            }
        }
    }
}

#[cfg(test)]
mod rmq_tests {
    use super::*;

    #[test]
    fn rmq() {
        fn test(vec: Vec<u32>) {
            let rmq = Rmq::create(&vec, RmqType::Min);
            for i in 0..vec.len() {
                for j in i + 1..vec.len() {
                    dbg!(i, j);
                    assert_eq!(
                        rmq.most_for_segment(i..j),
                        vec[i..j].iter().copied().min().unwrap()
                    );
                }
            }

            for i in 0..vec.len() {
                let fast_max_segment =
                    find_max_segment_with_this_number_as_min(0..vec.len(), vec[i], i, |r| {
                        rmq.most_for_segment(r)
                    });
                let slow_max_segment = {
                    let right = (0..i)
                        .rev()
                        .find(|pos| rmq.most_for_segment(*pos..i + 1) != vec[i])
                        .map(|x| x + 1)
                        .unwrap_or(0);
                    let left = (i + 1..=vec.len())
                        .find(|pos| rmq.most_for_segment(i..*pos) != vec[i])
                        .map(|x| x - 1)
                        .unwrap_or(vec.len());
                    right..left
                };
                assert_eq!(rmq.most_for_segment(fast_max_segment.clone()), vec[i]);
                assert_eq!(rmq.most_for_segment(slow_max_segment.clone()), vec[i]);
                assert_eq!(fast_max_segment, slow_max_segment);
            }

            let rmq = Rmq::create(&vec, RmqType::Max);
            for i in 0..vec.len() {
                for j in i + 1..vec.len() {
                    dbg!(i, j);
                    assert_eq!(
                        rmq.most_for_segment(i..j),
                        vec[i..j].iter().copied().max().unwrap()
                    );
                }
            }

            for i in 0..vec.len() {
                let fast_max_segment =
                    find_max_segment_with_this_number_as_max(0..vec.len(), vec[i], i, |r| {
                        rmq.most_for_segment(r)
                    });
                let slow_max_segment = {
                    let right = (0..i)
                        .rev()
                        .find(|pos| rmq.most_for_segment(*pos..i + 1) != vec[i])
                        .map(|x| x + 1)
                        .unwrap_or(0);
                    let left = (i + 1..=vec.len())
                        .find(|pos| rmq.most_for_segment(i..*pos) != vec[i])
                        .map(|x| x - 1)
                        .unwrap_or(vec.len());
                    right..left
                };
                assert_eq!(rmq.most_for_segment(fast_max_segment.clone()), vec[i]);
                assert_eq!(rmq.most_for_segment(slow_max_segment.clone()), vec[i]);
                assert_eq!(fast_max_segment, slow_max_segment);
            }
        }
        test(vec![]);
        test(vec![1]);
        test(vec![1, 2]);
        test(vec![1, 3, 4]);
        test(vec![1, 3, 4, 5]);
        test(vec![1, 2, 3, 3, 3, 4, 4, 3, 4, 2, 1]);
        test(vec![1, 2, 3, 0, 555, 1, 0, 0]);
        test(vec![1, 2, 32, 6, 7, 8, 53, 33]);
    }
}

#[cfg(test)]
mod segment_tree_tests {
    use super::*;

    #[test]
    fn segment_tree() {
        color_backtrace::install();
        fn test(vec: Vec<u64>) {
            use std::ops::*;
            test1(vec.clone(), std::cmp::min);
            test1(vec.clone(), std::cmp::max);
            test1(vec.clone(), u64::add);
            test1(vec.clone(), u64::bitxor);
            test1(vec.clone(), u64::mul);
            drop(vec);
        }
        fn test1<F: Fn(u64, u64) -> u64 + Copy>(mut vec: Vec<u64>, operation: F) {
            let mut vec_clone = vec.clone();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mut tree = SegmentTree::create(&mut vec, |_, x, y| operation(x, y), |_, x| x);
            for _ in 0..200 {
                if !tree.on().is_empty() {
                    let pos = rng.gen_range(0, tree.on().len());
                    let value = rng.gen_range(0, 50);
                    tree.set(pos, value);
                    vec_clone[pos] = value;
                }
                for i in 0..tree.on().len() {
                    for j in i + 1..=tree.on().len() {
                        let fast = tree.f_for_segment(i..j);
                        let long = Some(
                            vec_clone[i + 1..j]
                                .iter()
                                .copied()
                                .fold(vec_clone[i], operation),
                        )
                        .filter(|_| j - i > 0);
                        if fast != long {
                            dbg!(i..j, tree.tree, tree.input, vec_clone, fast, long);
                            panic!();
                        }
                    }
                }
            }
        }
        test(vec![]);
        test(vec![1]);
        test(vec![1, 2]);
        test(vec![1, 3, 4]);
        test(vec![1, 3, 4, 5]);
        test(vec![1, 2, 3, 3, 3, 4, 4, 3, 4, 2, 1]);
        test(vec![1, 2, 3, 0, 555, 1, 0, 0]);
        test(vec![1, 2, 32, 6, 7, 8, 53, 33]);
    }
}

#[cfg(test)]
mod scan_tests {
    use super::*;

    #[test]
    fn scan() {
        color_backtrace::install();

        let a = [1, 2, 3];
        assert!(
            a.iter()
                .copied()
                .scan_after(4, |state, &item| state * item)
                .eq([(4, 1), (4, 2), (8, 3)].iter().copied())
        );
        assert!(
            a.iter()
                .copied()
                .scan_after_first(|state, &item| state * item)
                .eq([(1, 2), (2, 3)].iter().copied())
        );
        assert!(
            a.iter()
                .copied()
                .scan_before(4, |state, &item| state * item)
                .eq([(4, 1), (8, 2), (24, 3)].iter().copied())
        );
        assert!(
            a.iter()
                .copied()
                .scan_before_first(|state, &item| state * item)
                .eq([(2, 2), (6, 3)].iter().copied())
        );
    }
}
