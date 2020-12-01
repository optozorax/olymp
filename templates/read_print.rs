fn read<T: FromStr, I: Iterator<Item = std::io::Result<String>>>(i: &mut I) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    i.next().unwrap().unwrap().parse().unwrap()
}

struct SpaceVec<T>(pub Vec<T>);
impl<T: FromStr> FromStr for SpaceVec<T>
where
    <T as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SpaceVec(
            s.split_whitespace()
                .map(|x| x.parse::<T>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

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

struct Chars(pub Vec<char>);
impl FromStr for Chars {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Chars(s.chars().into_iter().collect::<Vec<char>>()))
    }
}

// Allows to read two different types, separated by space
struct SpaceTuple2<A, B>(pub A, pub B);
impl<A: FromStr, B: FromStr> FromStr for SpaceTuple2<A, B>
where
    <A as FromStr>::Err: std::error::Error + 'static,
    <B as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let a = A::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        let b = B::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        Ok(SpaceTuple2(a, b))
    }
}

// Allows to read three different types, separated by space
struct SpaceTuple3<A, B, C>(pub A, pub B, pub C);
impl<A: FromStr, B: FromStr, C: FromStr> FromStr for SpaceTuple3<A, B, C>
where
    <A as FromStr>::Err: std::error::Error + 'static,
    <B as FromStr>::Err: std::error::Error + 'static,
    <C as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let a = A::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        let b = B::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        let c = C::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        Ok(SpaceTuple3(a, b, c))
    }
}

// Allows to read four different types, separated by space
struct SpaceTuple4<A, B, C, D>(pub A, pub B, pub C, pub D);
impl<A: FromStr, B: FromStr, C: FromStr, D: FromStr> FromStr for SpaceTuple4<A, B, C, D>
where
    <A as FromStr>::Err: std::error::Error + 'static,
    <B as FromStr>::Err: std::error::Error + 'static,
    <C as FromStr>::Err: std::error::Error + 'static,
    <D as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let a = A::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        let b = B::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        let c = C::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        let d = D::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
        Ok(SpaceTuple4(a, b, c, d))
    }
}

fn main() {
    let input = std::io::stdin();
    let mut stdin = input.lock().lines();
    let a: i64 = read(&mut stdin);
}