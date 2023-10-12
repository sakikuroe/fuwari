use crate::{
    convolution::{conv, intt, ntt},
    modint::ModInt,
};
use std::{fmt, ops};

/// fps![]
#[macro_export]
macro_rules! fps {
    { $( $x:expr ),* } => {
        {
            let coeff = vec![$(ModInt {val: $x as usize})*];
            FPS { coeff }
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
            let mut f = fps![];
            $(
                f.set($n as usize, ModInt {val: $a as usize});
            )*
            f
        }
    };
}

#[derive(Debug, Clone)]
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
        self.coeff.iter().all(|&c| c == ModInt { val: 0 })
    }

    /// # Returns
    /// $[x^{n}] f$
    pub fn get(&self, n: usize) -> ModInt {
        *self.coeff.get(n).unwrap_or(&ModInt { val: 0 })
    }

    pub fn set(&mut self, n: usize, a: ModInt) {
        if self.len() < n + 1 {
            self.coeff.resize(n + 1, ModInt { val: 0 });
        }
        self.coeff[n] = a;
    }

    /// Discard $[x^{n}]_{n \geq \mathrm{len}}$.
    pub fn truncate(&mut self, len: usize) {
        self.coeff.truncate(len);
        while self.coeff.last() == Some(&ModInt { val: 0 }) {
            self.coeff.pop();
        }
    }

    pub fn shift_left(&mut self, n: usize) {
        self.coeff.rotate_left(n);
        self.truncate(self.len() - n);
    }

    pub fn shift_right(&mut self, n: usize) {
        self.coeff.rotate_right(n);
        self.truncate(self.len() - n);
    }

    pub fn inv(&self, len: usize) -> Self {
        assert_ne!(self.get(0), ModInt { val: 0 });

        let mut g = FPS::new(vec![self.get(0).inv()]);
        for d in (0..).map(|n| 1 << n).take_while(|&n| n < len) {
            let mut f = FPS::new(self.coeff.iter().cloned().take(2 * d).collect::<Vec<_>>());
            f.coeff.resize(4 * d, ModInt { val: 0 });
            g.coeff.resize(4 * d, ModInt { val: 0 });
            ntt(&mut f.coeff);
            ntt(&mut g.coeff);
            g.coeff
                .iter_mut()
                .zip(f.coeff.iter())
                .for_each(|(g, f)| *g *= -*f * *g + ModInt { val: 2 });
            intt(&mut g.coeff);
            g.coeff.truncate(2 * d);
            let four_d_inv = ModInt { val: 4 * d }.inv();
            g.coeff.iter_mut().for_each(|x| *x *= four_d_inv);
        }

        g.truncate(len);
        g
    }

    pub fn derivative(&self) -> Self {
        FPS::new(
            self.coeff
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &x)| ModInt::new(i) * x)
                .collect(),
        )
    }

    pub fn integral(&self) -> Self {
        if self.is_empty() {
            self.clone()
        } else {
            let mut fact = vec![ModInt { val: 1 }];
            for i in 1..=self.len() {
                fact.push(fact[i - 1] * ModInt { val: i });
            }
            let mut ifact = vec![fact[self.len()].inv()];
            for i in 0..self.len() {
                ifact.push(
                    ifact[i]
                        * ModInt {
                            val: self.len() - i,
                        },
                )
            }
            ifact.reverse();

            FPS::new(
                vec![ModInt::new(0)]
                    .into_iter()
                    .chain(
                        self.coeff
                            .iter()
                            .enumerate()
                            .map(|(i, &x)| x * ifact[i + 1] * fact[i]),
                    )
                    .collect(),
            )
        }
    }

    pub fn log(&self, len: usize) -> Self {
        assert_eq!(self.get(0), ModInt { val: 1 });
        let mut f = FPS::new(self.coeff.iter().cloned().take(len).collect::<Vec<_>>()).derivative();
        let g = FPS::new(self.coeff.iter().cloned().take(len).collect::<Vec<_>>()).inv(len - 1);
        f *= g;
        f.truncate(len - 1);
        f = f.integral();
        f
    }

    pub fn exp(&self, len: usize) -> Self {
        assert_eq!(self.get(0), ModInt { val: 0 });

        let mut g = fps![1];
        for d in (0..).map(|n| 1 << n).take_while(|&n| n < len) {
            let mut f = FPS::new(self.coeff.iter().cloned().take(2 * d).collect::<Vec<_>>());
            f -= g.log(2 * d);
            f += fps![1];
            g *= f;
            g.coeff.truncate(2 * d);
        }

        g.truncate(len);
        g
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

impl ops::Neg for FPS {
    type Output = FPS;
    fn neg(self) -> Self {
        fps![] - self
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
