#![allow(
    unused_imports,
    clippy::many_single_char_names,
    dead_code,
    unused_macros,
    clippy::collapsible_if,
    clippy::too_many_arguments
)]

fn main() {
    // ---------------- FAST OUTPUT AND NOT FAST INPUT -------------------- //
	#[cfg(color_backtrace)] color_backtrace::install();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
    macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

    let input = stdin();
    let mut stdin = input.lock().lines();
    macro_rules! read { ($($x:tt)*) => { read(&mut stdin) }; }

    // -------------------------------------------------------------------- //


}


//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ 
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

use std::cmp::{max, min, Reverse, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::convert::{identity, TryFrom, TryInto};
use std::fmt::{Debug, Display, Error, Formatter};
use std::io::{BufRead, Read, Write, BufWriter, stdout, stdin};
use std::iter::once;
use std::iter::{FromIterator, Peekable};
use std::ops::{Add, Div, Mul, Neg, Range, RangeInclusive, Sub};
use std::str::FromStr;

