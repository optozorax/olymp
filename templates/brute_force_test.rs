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
            let size = rng.gen_range(1, 35);
            let fast_ans = unimplemented!();
            let long_ans = unimplemented!();
            if fast_ans != long_ans {
                panic!();
            }
        }
    }
}
