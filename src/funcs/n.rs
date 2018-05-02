use Flt;

pub fn is_nan<T: Flt>(f: T) -> bool {
    f == T::NAN
}

pub fn is_inf<T: Flt>(f: T) -> bool {
    f == T::INF || f == T::NEG_INF
}

pub fn is_fin<T: Flt>(f: T) -> bool {
    !(is_inf(f) || is_nan(f))
}

pub fn signum<T: Flt>(f: T) -> T {
    if is_nan(f) {
        T::NAN
    } else if f < T::ZERO {
        -T::ONE
    } else {
        T::ONE
    }
}
