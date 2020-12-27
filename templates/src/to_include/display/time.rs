macro_rules! print_time {
    ($($a:tt)*) => {
        eprintln!("[{}:{}] Time: {:?}", file!(), line!(), $($a)*);
    };
}

macro_rules! time {
    ($($a:tt)*) => {
        let now = std::time::Instant::now();
        $($a)*
        print_time!(now.elapsed());
    };
}

macro_rules! sum_time {
    ($($a:tt)*) => {{
        let now = std::time::Instant::now();
        $($a)*
        now.elapsed()
    }};
}
