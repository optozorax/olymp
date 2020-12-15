#![allow(
    unused_imports,
    clippy::many_single_char_names,
    dead_code,
    unused_macros
)]

fn main() {
	#[cfg(color_backtrace)] color_backtrace::install();

    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());
    macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
    macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

    let input = std::io::stdin();
    let mut stdin = input.lock().lines();
    #[rustfmt::skip] macro_rules! read { ($($x:tt)*) => { read(&mut stdin) }; }

    // -------------------------------------------------------------------- //


}


//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ 
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::convert::{TryFrom, TryInto, identity};
use std::fmt::{Debug, Display, Error, Formatter};
use std::io::{BufRead, Read, Write};
use std::iter::{FromIterator, Peekable};
use std::ops::{Range, RangeInclusive, Add, Mul};
use std::str::FromStr;
use std::iter::once;

