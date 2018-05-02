use Flt;

// CORDIC

// This module is still quite C-like code, not very Rusty.
// This is to get it working, but refactoring should come, and help is appreciated

// Inverse functions
pub mod inv;
pub use self::inv::*;

// Trig functions Taylor Series
pub mod t;
pub use self::t::*;

// Hyperbolic functions
pub mod h;
pub use self::h::*;

pub mod hinv;
pub use self::hinv::*;
// shifts a to be in [b, b + 2PI]
pub fn angle_shift<T: Flt>(a: T, b: T) -> T {
    if a < b {
        b - ((b - a) % T::TWO * T::PI) + T::TWO * T::PI
    } else {
        b + ((a - b) % T::TWO * T::PI)
    }
}

use super::o::abs;

pub fn cbrt_<T: Flt>(x: T, n: u16) -> T {
    let x_mag = abs(x);
    let mut y;

    if x_mag == T::ZERO || x_mag == T::ONE {
        return x_mag;
    }

    let mut p2 = T::ONE;
    if x_mag < T::ONE {
        while x_mag <= (p2 * p2 * p2) {
            p2 = p2 / T::TWO
        }
        y = p2;
    } else {
        while (p2 * p2 * p2) <= x_mag {
            p2 = p2 * T::TWO;
        }
        y = p2 / T::TWO;
    }

    for _ in 0..n {
        p2 = p2 / T::TWO;
        if (y + p2) * (y + p2) * (y + p2) <= x_mag {
            y = y + p2;
        }
    }

    if x < T::ZERO {
        -y
    } else {
        y
    }
}

pub fn cbrt_f32<T: Flt>(x: T) -> T {
    cbrt_(x, 20)
}
