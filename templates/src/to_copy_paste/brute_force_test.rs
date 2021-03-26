//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..200000 {
            let size = 20;
            let a = (0..size).map(|_| rng.gen_range(1, 10)).collect::<Vec<u64>>();
            let fast_ans = solve_fast(&a);
            let long_ans = solve_long(&a);
            if fast_ans != long_ans {
                dbg!(a, fast_ans, long_ans);
                panic!();
            }
        }
    }
}
