use Flt;

pub fn asin<T: Flt>(t: T) -> T {
    asin_(t, 25)
}

pub fn asin_<T: Flt>(t: T, n: u16) -> T {
    if t == T::ONE {
        return T::PI2;
    } else if t == -T::ONE {
        return -T::PI2;
    }
    let mut low = -T::PI2;
    let mut high = T::PI2;
    for _ in 0..n {
        let r = (low + high) / T::TWO;
        if super::sin(r) < t {
            low = r;
        } else {
            high = r;
        }
    }
    (low + high) / T::TWO
}

pub fn acos<T: Flt>(t: T) -> T {
    acos_(t, 25)
}

pub fn acos_<T: Flt>(t: T, n: u16) -> T {
    if t == T::ONE {
        return T::PI2;
    } else if t == -T::ONE {
        return -T::PI2;
    }
    let mut low = T::ZERO;
    let mut high = T::PI;
    for _ in 0..n {
        let r = (low + high) / T::TWO;
        if super::cos(r) < t {
            low = r;
        } else {
            high = r;
        }
    }
    (low + high) / T::TWO
}

pub fn atan<T: Flt>(t: T) -> T {
    atan_(t, 25)
}

pub fn atan_<T: Flt>(t: T, n: u16) -> T {
    if t == T::ONE {
        return T::PI2;
    } else if t == -T::ONE {
        return -T::PI2;
    }
    let mut low = -T::PI2;
    let mut high = T::PI2;
    for _ in 0..n {
        let r = (low + high) / T::TWO;
        if super::tan(r) < t {
            low = r;
        } else {
            high = r;
        }
    }
    (low + high) / T::TWO
}
