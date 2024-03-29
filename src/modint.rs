use std::{fmt, ops};

pub const MOD: usize = 998244353; // 119 * (1 << 23) + 1

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModInt {
    pub val: usize,
}

impl ModInt {
    pub fn new(n: usize) -> ModInt {
        ModInt { val: n % MOD }
    }

    pub fn val(&self) -> usize {
        self.val
    }

    /// # Run time
    /// $O(\log(\mathrm{MOD}))$
    pub fn inv(&self) -> ModInt {
        self.pow(MOD - 2)
    }

    /// # Run time
    /// $O(\log(n))$
    pub fn pow(&self, mut n: usize) -> ModInt {
        let mut res = ModInt::new(1);
        let mut x = *self;
        while n > 0 {
            if n % 2 == 1 {
                res *= x;
            }
            x = x * x;
            n /= 2;
        }

        res
    }
}

/// # Run time
/// $O(1)$
impl ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        let res = self.val + other.val;
        if res < MOD {
            ModInt { val: res }
        } else {
            ModInt { val: res - MOD }
        }
    }
}

/// # Run time
/// $O(1)$
impl ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        if self.val >= other.val {
            ModInt {
                val: self.val - other.val,
            }
        } else {
            ModInt {
                val: MOD + self.val - other.val,
            }
        }
    }
}

/// # Run time
/// $O(1)$
impl ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.val * other.val)
    }
}

/// # Run time
/// $O(\log(\mathrm{MOD}))$
impl ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl ops::Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> Self::Output {
        ModInt { val: 0 } - self
    }
}

/// # Run time
/// $O(1)$
impl ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

/// # Run time
/// $O(1)$
impl ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// # Run time
/// $O(1)$
impl ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

/// # Run time
/// $O(\log(\mathrm{MOD}))$
impl ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val())
    }
}
