#![allow(
	clippy::many_single_char_names,
	unused_macros,
	clippy::collapsible_if,
	clippy::too_many_arguments,
	dead_code,
	unused_imports
)]

// This thing with files: `dev.rs`, `main.rs`, `lib.rs` is the only way how to
// disable warnings, and not include this big #![allow(...)] into file which
// will be copied.

pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod e2;


