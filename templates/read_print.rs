fn is_ascii_newline(byte: u8) -> bool { byte == b'\n' || byte == b'\r' }

struct Scanner<I: Iterator> {
    iter: Peekable<I>,
    buf: Vec<u8>,
}

impl<I: Iterator<Item = u8>> Scanner<I> {
    pub fn new(iter: I) -> Self { Self { iter: iter.peekable(), buf: Vec::with_capacity(100) } }

    pub fn byte(&mut self) -> u8 { self.iter.next().unwrap() }

    pub fn read<T: FromStr>(&mut self) -> T
    where T::Err: Debug {
        self.buf.clear();
        let mut skip_spaces = true;
        while let Some(byte) = self.iter.peek() {
            if skip_spaces {
                if !byte.is_ascii_whitespace() {
                    skip_spaces = false;
                    self.buf.push(*byte);
                }
            } else {
                if byte.is_ascii_whitespace() {
                    break;
                } else {
                    self.buf.push(*byte);
                }
            }
            self.iter.next();
        }
        let s = std::str::from_utf8(&self.buf).unwrap_or_else(|_| panic!("input is not utf8: {:?}", self.buf));
        T::from_str(s).unwrap_or_else(|err| panic!("cannot parse from {:?}, error: {:?}", s, err))
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        while let Some(byte) = self.iter.peek().copied() {
            self.iter.next();
            if is_ascii_newline(byte) {
                break;
            } else {
                result.push(byte);
            }
        }
        result
    }

    pub fn readln<T: FromStr>(&mut self) -> Vec<T>
    where T::Err: Debug {
        let mut result = Vec::new();
        while self.iter.peek().map(|x| !is_ascii_newline(*x)).unwrap_or(false) {
            result.push(self.read());
        }
        self.iter.next();
        result
    }
}

//----------------------------------------------------------------------------

struct SpaceVec<T>(pub Vec<T>);
impl<T: Display> Display for SpaceVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.0.len();
        for (index, i) in self.0.iter().enumerate() {
            write!(f, "{}", i)?;
            if index + 1 != len {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

struct Lines<T>(pub Vec<T>);
impl<T: Display> Display for Lines<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

struct Chars(pub Vec<u8>);
impl Display for Chars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{}", char::from(*i))?;
        }
        Ok(())
    }
}

struct SpaceMatrix<T>(pub Vec<Vec<T>>);

impl<T: Display> Display for SpaceMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for (index, i) in line.iter().enumerate() {
                if index != 0 { write!(f, " ")?; }
                write!(f, "{}", i)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
