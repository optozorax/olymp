pub fn main() {
    // ----------------------------- Fast IO ------------------------------ //
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
    macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

    let input = stdin();
    let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
    #[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
    #[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
    #[rustfmt::skip] macro_rules! byte { () => { scanner.byte() }; }
    #[rustfmt::skip] macro_rules! bytes { () => { scanner.bytes() }; }
    // -------------------------------------------------------------------- //


}


//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ 
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::{identity, TryFrom, TryInto},
    error,
    fmt::{Debug, Display, Error, Formatter},
    io::{self, stdin, stdout, BufRead, BufWriter, Read, Write},
    iter::{once, FromIterator, Peekable},
    mem::swap,
    ops::{Add, Div, Mul, Neg, Range, RangeInclusive, Rem, Sub},
    str::FromStr,
};
