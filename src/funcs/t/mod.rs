// CORDIC

// This module is still quite C-like code, not very Rusty.
// This is to get it working, but refactoring should come, and help is appreciated

use core::{f32::consts::PI as fPI, f64::consts::PI as dPI};

mod consts;
// Inverse functions CORDIC
pub mod inv;
pub use self::inv::*;

// Trig functions Taylor Series
pub mod t;
pub use self::t::*;

// Hyperbolic functions
pub mod h;
pub use self::h::*;

// shifts a to be in [b, b + 2PI]
pub fn angle_shift_f32(a: f32, b: f32) -> f32 {
    if a < b {
        b - ((b - a) % 2.0 * fPI) + 2.0 * fPI
    } else {
        b + ((a - b) % 2.0 * fPI)
    }
}

pub fn angle_shift_f64(a: f64, b: f64) -> f64 {
    if a < b {
        b - ((b - a) % 2.0 * dPI) + 2.0 * dPI
    } else {
        b + ((a - b) % 2.0 * dPI)
    }
}

use super::o::{abs_f32, abs_f64};

pub fn cbrt_f32_(x: f32, n: usize) -> f32 {
    let x_mag = abs_f32(x);
    let mut y;

    if x_mag == 0.0 {
        return 0.0;
    } else if x_mag == 1.0 {
        return 1.0;
    }

    let mut p2 = 1.0;
    if x_mag < 1.0 {
        while x_mag <= (p2 * p2 * p2) {
            p2 = p2 / 2.0
        }
        y = p2;
    } else {
        while (p2 * p2 * p2) <= x_mag {
            p2 = p2 * 2.0;
        }
        y = p2 / 2.0;
    }

    for _ in 0..n {
        p2 = p2 / 2.0;
        if (y + p2) * (y + p2) * (y + p2) <= x_mag {
            y = y + p2;
        }
    }

    if x < 0.0 {
        -y
    } else {
        y
    }
}

pub fn cbrt_f32(x: f32) -> f32 {
    cbrt_f32_(x, 20)
}

pub fn cbrt_f64_(x: f64, n: usize) -> f64 {
    let x_mag = abs_f64(x);
    let mut y;

    if x_mag == 0.0 {
        return 0.0;
    } else if x_mag == 1.0 {
        return 1.0;
    }

    let mut p2 = 1.0;
    if x_mag < 1.0 {
        while x_mag <= (p2 * p2 * p2) {
            p2 = p2 / 2.0
        }
        y = p2;
    } else {
        while (p2 * p2 * p2) <= x_mag {
            p2 = p2 * 2.0;
        }
        y = p2 / 2.0;
    }

    for _ in 0..n {
        p2 = p2 / 2.0;
        if (y + p2) * (y + p2) * (y + p2) <= x_mag {
            y = y + p2;
        }
    }

    if x < 0.0 {
        -y
    } else {
        y
    }
}

pub fn cbrt_f64(x: f64) -> f64 {
    cbrt_f64_(x, 20)
}
