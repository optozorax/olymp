macro_rules! try_bool { ($($t:tt)*) => { if let Some(result) = $($t)* { result } else { return false; } }; }