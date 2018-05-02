use super::super::m::exp;
use Flt;

pub fn sinh<T: Flt>(x: T) -> T {
    (exp(T::TWO * x) - T::ONE) / (T::TWO * exp(x))
}

pub fn cosh<T: Flt>(x: T) -> T {
    (exp(T::TWO * x) + T::ONE) / (T::TWO * exp(x))
}

pub fn tanh<T: Flt>(x: T) -> T {
    sinh(x) / cosh(x)
}
