use crate::modint::ModInt;

const FFT_RATE: [usize; 22] = [
    0x3656d65b, 0x1e5ea9e6, 0x16038782, 0x13caac90, 0x3a9a4cfa, 0x761af21, 0xe372007, 0x3a2be7d4,
    0x23fe18b2, 0x330f5b68, 0x7d37cf9, 0x3239edef, 0x2b8ea5c3, 0x382d2452, 0x300e9be2, 0x908b3f5,
    0x1e726cd9, 0x1e02c2f0, 0x2c49629c, 0x2c2b7c93, 0x35a5081, 0x33b69d8b,
];

const FFT_IRATE: [usize; 22] = [
    0x52929a6, 0x163456b8, 0x16400573, 0x267c5b5f, 0x6b059a5, 0x294c15f1, 0x94415d9, 0x2f83389c,
    0x569c0ec, 0x3346ebba, 0x37473ab0, 0x1524e16f, 0x68442e3, 0x117ab9d0, 0x1fe52df0, 0x1263f553,
    0x7392943, 0x24433aa8, 0x1a2993eb, 0x156d2fbf, 0x311e570f, 0x6294a13,
];

pub fn conv(a: &Vec<ModInt>, b: &Vec<ModInt>) -> Vec<ModInt> {
    let ntt = |a: &mut Vec<ModInt>| {
        let n = a.len();
        let h = n.trailing_zeros();

        for len in 0..h {
            let p = 1 << (h - len - 1);
            let mut rot = ModInt { val: 1 };
            for (s, (al, ar)) in a
                .chunks_mut(1 << (h - len))
                .map(|a| a.split_at_mut(p))
                .enumerate()
            {
                for (al, ar) in al.iter_mut().zip(ar.iter_mut()) {
                    let l = *al;
                    let r = *ar * rot;
                    *al = l + r;
                    *ar = l - r;
                }
                rot *= ModInt {
                    val: FFT_RATE[(!s).trailing_zeros() as usize],
                };
            }
        }
    };

    let intt = |a: &mut Vec<ModInt>| {
        let n = a.len();
        let h = n.trailing_zeros();

        for len in (1..=h).rev() {
            let p = 1 << (h - len);
            let mut irot = ModInt { val: 1 };
            for (s, (al, ar)) in a
                .chunks_mut(1 << (h - len + 1))
                .map(|a| a.split_at_mut(p))
                .enumerate()
            {
                for (al, ar) in al.iter_mut().zip(ar.iter_mut()) {
                    let l = *al;
                    let r = *ar;
                    *al = l + r;
                    *ar = (l - r) * irot;
                }
                irot *= ModInt {
                    val: FFT_IRATE[(!s).trailing_zeros() as usize],
                };
            }
        }
    };

    let s = a.len() + b.len() - 1;
    let t = s.next_power_of_two();

    let (mut a, mut b) = (a.to_owned(), b.to_owned());
    a.resize(t, ModInt { val: 0 });
    ntt(&mut a);
    b.resize(t, ModInt { val: 0 });
    ntt(&mut b);
    a.iter_mut().zip(b.iter()).for_each(|(x, y)| *x *= *y);
    intt(&mut a);
    a.resize(s, ModInt { val: 0 });
    let t_inv = ModInt { val: t }.inv();
    a.iter_mut().for_each(|x| *x *= t_inv);
    a
}
