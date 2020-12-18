trait JoinDisplay {
    fn join_display<T: Display>(self, separator: T) -> String;
}

impl<I: Iterator<Item = Y>, Y: Display> JoinDisplay for I {
    fn join_display<T: Display>(mut self, separator: T) -> String {
        use std::fmt::Write;
        if let Some(first) = self.next() {
            let mut result = String::new();
            result.write_fmt(format_args!("{}", first)).unwrap();
            for i in self {
                result.write_fmt(format_args!("{}{}", separator, i)).unwrap();
            }
            result
        } else {
            String::new()
        }
    }
}