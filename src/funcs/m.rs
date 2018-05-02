pub fn recip_f32(f: f32) -> f32 {
    powi_f32(f, -1)
}

pub fn recip_f64(f: f64) -> f64 {
    powi_f64(f, -1)
}

pub fn powi_f32(f: f32, p: i32) -> f32 {
    if p > 0 {
        let (mut f, mut p, mut a) = (f, p, 1.0);
        while p != 0 {
            if p % 2 == 1 {
                a *= f;
            }
            p /= 2;
            f *= f;
        }
        a
    } else {
        1.0 / powi_f32(f, -p)
    }
}

pub fn powi_f64(f: f64, p: i32) -> f64 {
    if p > 0 {
        let (mut f, mut p, mut a) = (f, p, 1.0);
        while p != 0 {
            if p % 2 == 1 {
                a *= f;
            }
            p /= 2;
            f *= f;
        }
        a
    } else {
        1.0 / powi_f64(f, -p)
    }
}

pub fn expm1_f32(p: f32) -> f32 {
    expm1_f32_(p, 15)
}

pub fn expm1_f32_(p: f32, n: i32) -> f32 {
    let (mut pn, mut f, mut a) = (p, 1.0, 0.0);
    for i in 1..=n {
        f *= i as f32;
        a += pn / f;
        pn *= p;
    }
    a
}

pub fn expm1_f64(p: f64) -> f64 {
    expm1_f64_(p, 15)
}

pub fn expm1_f64_(p: f64, n: i32) -> f64 {
    let (mut pn, mut f, mut a) = (p, 1.0, 0.0);
    for i in 1..=n {
        f *= i as f64;
        a += pn / f;
        pn *= p;
    }
    a
}

pub fn exp_f32(p: f32) -> f32 {
    1.0 + expm1_f32(p)
}

pub fn exp_f64(p: f64) -> f64 {
    1.0 + expm1_f64(p)
}

pub fn ln_f32(p: f32) -> f32 {
    ln_f32_(p, 15)
}

pub fn ln_f32_(p: f32, n: i32) -> f32 {
    let mut a = 0.0;
    for i in 1..=n {
        a += powi_f32((p - 1.0) / p, i) / i as f32
    }
    a
}

pub fn ln_f64(p: f64) -> f64 {
    ln_f64_(p, 15)
}

pub fn ln_f64_(p: f64, n: i32) -> f64 {
    let mut a = 0.0;
    for i in 1..=n {
        a += powi_f64((p - 1.0) / p, i) / i as f64
    }
    a
}

pub fn powf_f32(f: f32, p: f32) -> f32 {
    exp_f32(p * ln_f32(f))
}

pub fn powf_f64(f: f64, p: f64) -> f64 {
    exp_f64(p * ln_f64(f))
}

pub fn sqrt_f32(f: f32) -> f32 {
    sqrt_f32_(f, 25)
}

pub fn sqrt_f32_(f: f32, n: usize) -> f32 {
    let (mut min, mut max) = (0.0, f);
    let mut r = (min + max) / 2.0;
    for i in 0..n {
        if r * r == f {
            return r;
        } else if r * r < f {
            min = r;
        } else {
            max = r;
        }
        r = (min + max) / 2.0;
    }
    r
}

pub fn sqrt_f64(f: f64) -> f64 {
    sqrt_f64_(f, 25)
}

pub fn sqrt_f64_(f: f64, n: usize) -> f64 {
    let (mut min, mut max) = (0.0, f);
    let mut r = (min + max) / 2.0;
    for i in 0..n {
        if r * r == f {
            return r;
        } else if r * r < f {
            min = r;
        } else {
            max = r;
        }
        r = (min + max) / 2.0;
    }
    r
}

use core::{f32::consts::{LN_10 as fLN_10, LN_2 as fLN_2},
           f64::consts::{LN_10 as dLN_10, LN_2 as dLN_2}};

pub fn exp2_f32(p: f32) -> f32 {
    exp_f32(p * fLN_2)
}

pub fn exp2_f64(p: f64) -> f64 {
    exp_f64(p * dLN_2)
}

pub fn log_f32(f: f32, b: f32) -> f32 {
    ln_f32(f) / ln_f32(b)
}

pub fn log_f64(f: f64, b: f64) -> f64 {
    ln_f64(f) / ln_f64(b)
}

pub fn log2_f32(f: f32) -> f32 {
    ln_f32(f) / fLN_2
}

pub fn log2_f64(f: f64) -> f64 {
    ln_f64(f) / dLN_2
}

pub fn log10_f32(f: f32) -> f32 {
    ln_f32(f) / fLN_10
}

pub fn log10_f64(f: f64) -> f64 {
    ln_f64(f) / dLN_10
}
