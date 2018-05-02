use super::super::m::powi;
use Flt;

pub fn sin<T: Flt>(x: T) -> T {
    x - powi(x, 3) / T::C6 + powi(x, 5) / T::C120 - powi(x, 7) / T::C5040 + powi(x, 9) / T::C362880
}

pub fn cos<T: Flt>(x: T) -> T {
    sin(T::PI2 - x)
}

pub fn tan<T: Flt>(x: T) -> T {
    x + powi(x, 3) / T::C3 + T::C2D15 * powi(x, 5) + T::C17D315 * powi(x, 7)
}
