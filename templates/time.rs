macro_rules! time {
    ($a:expr) => {{
        let now = std::time::Instant::now();
        $a;
        eprintln!("[{}:{}] Time: {:?}", file!(), line!(), now.elapsed());
    }};
}

time! {{

}};