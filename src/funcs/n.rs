use core::{f32::NAN as fNAN, f64::NAN as dNAN};

pub fn is_nan_f32(f: f32) -> bool {
    f == fNAN
}

pub fn is_nan_f64(f: f64) -> bool {
    f == dNAN
}

use core::{f32::{INFINITY as fI, NEG_INFINITY as fNI}, f64::{INFINITY as dI, NEG_INFINITY as dNI}};

pub fn is_inf_f32(f: f32) -> bool {
    f == fI || f == fNI
}

pub fn is_inf_f64(f: f64) -> bool {
    f == dI || f == dNI
}

pub fn is_fin_f32(f: f32) -> bool {
    !(is_inf_f32(f) || is_nan_f32(f))
}

pub fn is_fin_f64(f: f64) -> bool {
    !(is_inf_f64(f) || is_nan_f64(f))
}

pub fn signum_f32(f: f32) -> f32 {
    if f == fNAN {
        fNAN
    } else if (f > 0.0) || (f == 0.0) || (f == fI) {
        1.0
    } else {
        0.0
    }
}

pub fn signum_f64(f: f64) -> f64 {
    if f == dNAN {
        dNAN
    } else if (f > 0.0) || (f == 0.0) || (f == dI) {
        1.0
    } else {
        0.0
    }
}
