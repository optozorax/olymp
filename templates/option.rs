trait AnyOrBothWith {
    type Inner;
    fn any_or_both_with<F: FnOnce(Self::Inner, Self::Inner) -> Self::Inner>(self, b: Option<Self::Inner>, f: F) -> Option<Self::Inner>;
}

impl<T> AnyOrBothWith for Option<T> {
    type Inner = T;
    fn any_or_both_with<F: FnOnce(Self::Inner, Self::Inner) -> Self::Inner>(self, b: Option<Self::Inner>, f: F) -> Option<Self::Inner> {
        match (self, b) {
            (Some(a), Some(b)) => Some((f)(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}
