//----------------------------------------------------------------------------
// Copy-pasted from https://github.com/EbTech/rust-algorithms/blob/master/src/math/num.rs
// And slightly modified

#[derive(Debug)]
pub struct ModInverses {
    inner: Vec<Field>,
}

impl ModInverses {
    pub fn new(n: usize) -> Self {
        let mut recips = Vec::with_capacity(n + 1);
        recips.push(Field::from(0));
        recips.push(Field::from(1));
        for i in (2..=n).map(|x| x as i64) {
            let (md, dv) = (Field::MOD % i, Field::MOD / i);
            recips.push(recips[md as usize] * Field::from_small(-dv));
        }
        Self { inner: recips }
    }

    pub fn get_inverse(&self, n: Field) -> Field {
        self.inner[n.val as usize]
    }
}

#[derive(Debug)]
pub struct ModFactorials {
    inner: Vec<Field>,
    inv_inner: Vec<Field>,
}

impl ModFactorials {
    pub fn new(n: usize, inverses: &ModInverses) -> Self {
        let mut factorial = Vec::with_capacity(n + 1);
        let mut inv_factorial = Vec::with_capacity(n + 1);
        factorial.push(Field::from(1));
        factorial.push(Field::from(1));
        inv_factorial.push(Field::from(1));
        inv_factorial.push(Field::from(1));
        for (index, i) in (2..=n).map(|i| (i, Field::from(i as i64))) {
            factorial.push(factorial[index - 1] * i);
            inv_factorial.push(inv_factorial[index - 1] * inverses.get_inverse(i));
        }
        Self {
            inner: factorial,
            inv_inner: inv_factorial,
        }
    }

    pub fn get_factorial(&self, n: Field) -> Field {
        self.inner[n.val as usize]
    }

    pub fn get_inv_factorial(&self, n: Field) -> Field {
        self.inv_inner[n.val as usize]
    }

    /// n choose r
    pub fn get_choose(&self, n: Field, r: Field) -> Field {
        std::debug_assert!(r <= n);
        self.get_factorial(n) * self.get_inv_factorial(r) * self.get_inv_factorial(n - r)
    }

    /// (n choose r)^-1
    pub fn get_inv_choose(&self, n: Field, r: Field) -> Field {
        std::debug_assert!(r <= n);
        self.get_inv_factorial(n) * self.get_factorial(r) * self.get_factorial(n - r)
    }

    /// P(n, k)
    pub fn get_permute(&self, n: Field, k: Field) -> Field {
        std::debug_assert!(k <= n);
        self.get_factorial(n) * self.get_inv_factorial(n - k)
    }

    /// P(n, k)^-1
    pub fn get_inv_permute(&self, n: Field, k: Field) -> Field {
        std::debug_assert!(k <= n);
        self.get_inv_factorial(n) * self.get_factorial(n - k)
    }
}

/// Represents an element of the finite (Galois) field of prime order, given by
/// MOD. Until Rust gets const generics, MOD must be hardcoded, but any prime in
/// [1, 2^31.5] will work. If MOD is not prime, ring operations are still valid
/// but recip() and division are not. Note that the latter operations are also
/// the slowest, so precompute any inverses that you intend to use frequently.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Field {
    pub val: i64,
}
impl Field {
    pub const MOD: i64 = 1_000_000_007;

    /// Computes self^exp in O(log(exp)) time
    pub fn pow(mut self, mut exp: u64) -> Self {
        let mut result = Self::from_small(1);
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * self;
            }
            self = self * self;
            exp /= 2;
        }
        result
    }

    /// Computes self^-1 in O(log(Self::MOD)) time
    /// You should use `ModInverses` with precomputed inverse values
    pub fn recip(self) -> Self {
        self.pow(Self::MOD as u64 - 2)
    }

    /// Avoids the % operation but requires -Self::MOD <= x < Self::MOD
    fn from_small(s: i64) -> Self {
        let val = if s < 0 { s + Self::MOD } else { s };
        Self { val }
    }
}
impl From<i32> for Field {
    fn from(val: i32) -> Self {
        Self::from(i64::from(val))
    }
}
impl From<i64> for Field {
    fn from(val: i64) -> Self {
        Self::from_small(val % Self::MOD)
    }
}
impl From<usize> for Field {
    fn from(val: usize) -> Self {
        Self {
            val: i64::try_from(val).unwrap() % Self::MOD,
        }
    }
}
impl Neg for Field {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from_small(-self.val)
    }
}
impl Add for Field {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_small(self.val + other.val - Self::MOD)
    }
}
impl Sub for Field {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_small(self.val - other.val)
    }
}
impl Mul for Field {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::from(self.val * other.val)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Field {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}
