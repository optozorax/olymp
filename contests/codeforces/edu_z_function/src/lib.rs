#![allow(
	clippy::many_single_char_names,
	unused_macros,
	clippy::collapsible_if,
	clippy::too_many_arguments,
	dead_code,
	unused_imports
)]

#[macro_export]
macro_rules! print {
	($($t:tt)*) => {
		compile_error!("Use print only in main because of Fast IO!");
	};
}

#[macro_export]
macro_rules! println {
	($($t:tt)*) => {
		compile_error!("Use print only in main because of Fast IO!");
	};
}

// This thing with files: `dev.rs`, `main.rs`, `lib.rs` is the only way how to
// disable warnings, and not include this big #![allow(...)] into file which
// will be copied.

pub mod step1_a;
pub mod step1_b;
pub mod step1_c;
pub mod step1_d;
pub mod step2_a;
pub mod step2_b;
pub mod step2_c;
pub mod step2_c_checker;
pub mod step3_a;
pub mod step4_a;
pub mod step4_b;
pub mod step4_c;
pub mod step4_d;
pub mod step4_e;
pub mod step4_f;
pub mod step4_g;
pub mod step4_h;
pub mod step4_i;
pub mod step4_j;
