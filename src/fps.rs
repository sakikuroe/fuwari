use crate::{convolution::conv, modint::ModInt};
use std::{fmt, ops};

/// fps![]
#[macro_export]
macro_rules! fps {
    { $( $x:expr ),* } => {
        {
            use fuwari::fps::FPS;
            use fuwari::modint::ModInt;
            let mut coeff = vec![];
            $(
                 coeff.push(ModInt {val: $x as usize});
            )*
            FPS::new(coeff)
        }
    };
}

/// sfps!()
#[macro_export]
macro_rules! sfps {
    { $( ($n:expr, $a:expr) ),* } => {
        {
            use fuwari::fps::FPS;
            use fuwari::modint::ModInt;
            let mut f = FPS::new(vec![]);
            $(
                f.set($n as usize, ModInt {val: $a as usize});
            )*
            f
        }
    };
}

#[derive(Clone)]
pub struct FPS {
    pub coeff: Vec<ModInt>,
}

impl FPS {
    pub fn new(coeff: Vec<ModInt>) -> Self {
        FPS { coeff }
    }

    pub fn len(&self) -> usize {
        self.coeff.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coeff.iter().all(|&c| c == ModInt::new(0))
    }

    /// # Returns
    /// $[x^{n}] f$
    pub fn get(&self, n: usize) -> ModInt {
        *self.coeff.get(n).unwrap_or(&ModInt::new(0))
    }

    pub fn set(&mut self, n: usize, a: ModInt) {
        if self.len() < n + 1 {
            self.coeff.resize(n + 1, ModInt::new(0));
        }
        self.coeff[n] = a;
    }

    /// Discard $[x^{n}]_{n \geq \mathrm{len}}$.
    pub fn truncate(&mut self, len: usize) {
        self.coeff.truncate(len);
        while self.coeff.last() == Some(&ModInt::new(0)) {
            self.coeff.pop();
        }
    }
}

impl ops::Add for FPS {
    type Output = FPS;
    fn add(self, other: Self) -> Self {
        let len = std::cmp::max(self.len(), other.len());
        FPS::new((0..len).map(|i| self.get(i) + other.get(i)).collect())
    }
}

impl ops::Sub for FPS {
    type Output = FPS;
    fn sub(self, other: Self) -> Self {
        let len = std::cmp::max(self.len(), other.len());
        FPS::new((0..len).map(|i| self.get(i) - other.get(i)).collect())
    }
}

impl ops::Mul for FPS {
    type Output = FPS;
    fn mul(self, other: Self) -> Self {
        FPS::new(conv(&self.coeff, &other.coeff))
    }
}

impl ops::AddAssign for FPS {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl ops::SubAssign for FPS {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl ops::MulAssign for FPS {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl fmt::Display for FPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "0x^0")
        } else {
            write!(
                f,
                "{}",
                self.coeff
                    .iter()
                    .enumerate()
                    .map(|(n, a)| format!("{}x^{}", a, n))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )
        }
    }
}
