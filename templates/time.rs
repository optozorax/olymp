macro_rules! time {
    ($a:expr) => {{
        let now = std::time::Instant::now();
        #[allow(clippy::redundant_closure_call)]
        (|| $a)();
        eprintln!("[{}:{}] Time: {:?}", file!(), line!(), now.elapsed());
    }};
}

time! {{

}};

#[test]
fn bench() {
    panic!();
}