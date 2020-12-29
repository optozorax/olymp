use proc_macro::*;

#[proc_macro_attribute]
pub fn fastio(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut iter = item.into_iter();
	let mut result = TokenStream::new();
	let group = loop {
		let next = iter.next().unwrap();
		match next {
			TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => break group,
			other => result.extend(std::iter::once(other)),
		}
	};

	let to_add: TokenStream = include_str!("../../templates/src/to_copy_paste/fast_io.rs").parse().unwrap();

	let mut inner = TokenStream::new();
	inner.extend(to_add);
	inner.extend(group.stream());
	result.extend(std::iter::once(TokenTree::Group(Group::new(Delimiter::Brace, inner))));
	result
}
