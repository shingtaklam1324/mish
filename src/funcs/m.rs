use Flt;

pub fn recip<T: Flt>(f: T) -> T {
    T::ONE / f
}

pub fn powi<T: Flt>(f: T, p: i16) -> T {
    if p > 0 {
        let (mut f, mut p, mut a) = (f, p, T::ONE);
        while p != 0 {
            if p % 2 == 1 {
                a = a * f;
            }
            p /= 2;
            f = f * f;
        }
        a
    } else {
        T::ONE / powi(f, -p)
    }
}

pub fn expm1<T: Flt>(p: T) -> T {
    expm1_(p, 15)
}

pub fn expm1_<T: Flt>(p: T, n: i16) -> T {
    let (mut pn, mut f, mut a) = (p, T::ONE, T::ZERO);
    for i in 1..=n {
        f = f * T::from(i);
        a = a + pn / f;
        pn = p * pn;
    }
    a
}

pub fn exp<T: Flt>(p: T) -> T {
    T::ONE + expm1(p)
}

pub fn ln<T: Flt>(p: T) -> T {
    ln_(p, 300)
}

pub fn ln_<T: Flt>(p: T, n: i16) -> T {
    let mut a = T::ZERO;
    for i in 1..=n {
        a = a + powi((p - T::ONE) / p, i) / T::from(i)
    }
    a
}

pub fn powf<T: Flt>(f: T, p: T) -> T {
    exp(p * ln(f))
}

pub fn sqrt<T: Flt>(f: T) -> T {
    sqrt_(f, 25)
}

pub fn sqrt_<T: Flt>(f: T, n: u16) -> T {
    let (mut min, mut max) = (T::ZERO, f);
    let mut r = (min + max) / T::TWO;
    for _ in 0..n {
        if r * r == f {
            return r;
        } else if r * r < f {
            min = r;
        } else {
            max = r;
        }
        r = (min + max) / T::TWO;
    }
    r
}

pub fn exp2<T: Flt>(p: T) -> T {
    exp(p * T::LN2)
}

pub fn log<T: Flt>(f: T, b: T) -> T {
    ln(f) / ln(b)
}

pub fn log2<T: Flt>(f: T) -> T {
    ln(f) / T::LN2
}

pub fn log10<T: Flt>(f: T) -> T {
    ln(f) / T::LN10
}
