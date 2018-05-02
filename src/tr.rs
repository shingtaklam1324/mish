use core::ops;

pub trait Flt:
    PartialEq
    + PartialOrd
    + ops::Neg<Output = Self>
    + Sized
    + Copy
    + ops::Sub<Output = Self>
    + ops::Rem<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Add<Output = Self>
    + From<i16>
{
    const ZERO: Self;
    const NAN: Self;
    const INF: Self;
    const NEG_INF: Self;
    const HALF: Self;
    const ONE: Self;
    const TWO: Self;
    const LN2: Self;
    const LN10: Self;
    const PI: Self;
    const PI2: Self;

    const C6: Self;
    const C120: Self;
    const C5040: Self;
    const C362880: Self;

    const C3: Self;
    const C2D15: Self;
    const C17D315: Self;
}

use core::{f32::{self as f, consts as fc},
           f64::{self as d, consts as dc}};

impl Flt for f32 {
    const ZERO: Self = 0.0;
    const NAN: Self = f::NAN;
    const INF: Self = f::INFINITY;
    const NEG_INF: Self = f::NEG_INFINITY;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const LN2: Self = fc::LN_2;
    const LN10: Self = fc::LN_10;
    const PI: Self = fc::PI;
    const PI2: Self = fc::FRAC_PI_2;

    const C6: Self = 6.0;
    const C120: Self = 120.0;
    const C5040: Self = 5040.0;
    const C362880: Self = 362880.0;
    const C3: Self = 3.0;
    const C2D15: Self = 2.0 / 15.0;
    const C17D315: Self = 17.0 / 315.0;
}

impl Flt for f64 {
    const ZERO: Self = 0.0;
    const NAN: Self = d::NAN;
    const INF: Self = d::INFINITY;
    const NEG_INF: Self = d::NEG_INFINITY;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const LN2: Self = dc::LN_2;
    const LN10: Self = dc::LN_10;
    const PI: Self = dc::PI;
    const PI2: Self = dc::FRAC_PI_2;

    const C6: Self = 6.0;
    const C120: Self = 120.0;
    const C5040: Self = 5040.0;
    const C362880: Self = 362880.0;
    const C3: Self = 3.0;
    const C2D15: Self = 2.0 / 15.0;
    const C17D315: Self = 17.0 / 315.0;
}
