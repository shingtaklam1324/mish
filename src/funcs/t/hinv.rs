use super::super::m::{ln, sqrt};
use Flt;

pub fn arsinh<T: Flt>(x: T) -> T {
    ln(x + sqrt(x * x + T::ONE))
}

pub fn arcosh_f32<T: Flt>(x: T) -> T {
    ln(x + sqrt(x * x - T::ONE))
}

pub fn artanh<T: Flt>(x: T) -> T {
    T::HALF * ln((T::ONE + x) / (T::ONE - x))
}
