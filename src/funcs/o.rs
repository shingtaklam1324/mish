use Flt;

pub fn abs<T: Flt>(f: T) -> T {
    if f < T::ZERO {
        -f
    } else {
        f
    }
}

pub fn floor<T: Flt>(f: T) -> T {
    f - abs(f % T::ONE)
}

pub fn ceil<T: Flt>(f: T) -> T {
    floor(f) + T::ONE
}

pub fn trunc<T: Flt>(f: T) -> T {
    if f < T::ZERO {
        ceil(f)
    } else {
        floor(f)
    }
}

pub fn round<T: Flt>(f: T) -> T {
    match (abs(f) % T::ONE < T::HALF, f < T::ZERO) {
        (true, true) => ceil(f),
        (true, false) => floor(f),
        (false, true) => floor(f),
        (false, false) => ceil(f),
    }
}

pub fn fract<T: Flt>(f: T) -> T {
    f % T::ONE
}
