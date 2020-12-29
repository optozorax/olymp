fn is_palindrome<T: Eq>(input: &[T]) -> bool { input.iter().zip(input.iter().rev()).all(|(x, y)| x == y) }