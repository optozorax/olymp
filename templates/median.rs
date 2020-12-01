// Returned value `x` which minimizes `|x0-x|+|x1-x|+...+|xn-x|`
fn median(x: &mut [i64]) -> i64 {
    assert!(!x.is_empty());
    x.sort_unstable();
    x[x.len() / 2]
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