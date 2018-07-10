use super::super::m::powi;
use Flt;

/// This is very good for -pi/2 < x < pi/2
/// It gets worse outside this interval
fn sin_maclaurin<T: Flt>(x: T) -> T {
    x - powi(x, 3) / T::C6 + powi(x, 5) / T::C120 - powi(x, 7) / T::C5040 + powi(x, 9) / T::C362880
}

pub fn cos<T: Flt>(x: T) -> T {
    let abs_x = if x < T::ZERO { -x } else { x };
    // Between 0 and 2*pi
    let standard_position = abs_x % (T::TWO * T::PI);
    // Make the argument to sin_maclaurin be between -pi/2 and pi/2
    if standard_position < T::PI {
        sin_maclaurin(T::PI2 - standard_position)
    } else {
        sin_maclaurin(standard_position - T::PI2 - T::PI)
    }
}

pub fn sin<T: Flt>(x: T) -> T {
    cos(T::PI2 - x)
}

pub fn tan<T: Flt>(x: T) -> T {
    x + powi(x, 3) / T::C3 + T::C2D15 * powi(x, 5) + T::C17D315 * powi(x, 7)
}
