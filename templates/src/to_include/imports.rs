use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::{identity, TryFrom, TryInto},
    error,
    fmt::{Debug, Display, Error, Formatter},
    io::{self, stdin, stdout, BufRead, BufWriter, Read, Write},
    iter::{once, FromIterator, Peekable, FusedIterator},
    marker::PhantomData,
    mem::swap,
    ops::{Add, Div, Mul, Neg, Range, RangeInclusive, Rem, Sub, AddAssign},
    str::FromStr,
};
