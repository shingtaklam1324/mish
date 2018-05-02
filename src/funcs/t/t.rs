use core::{f32::consts::{FRAC_PI_2 as fPI2, PI as fPI},
           f64::consts::{FRAC_PI_2 as dPI2, PI as dPI}};

pub fn sin_f32(x: f32) -> f32 {
    if x < -fPI2 {
        sin_f32(-fPI - x)
    } else if x > fPI2 {
        sin_f32(fPI - x)
    } else {
        x - (x * x * x) / 6.0 + (x * x * x * x * x) / 120.0 - (x * x * x * x * x * x * x) / 5040.0
    }
}

pub fn sin_f64(x: f64) -> f64 {
    if x < -dPI2 {
        sin_f64(-dPI - x)
    } else if x > dPI2 {
        sin_f64(dPI - x)
    } else {
        x - (x * x * x) / 6.0 + (x * x * x * x * x) / 120.0 - (x * x * x * x * x * x * x) / 5040.0
    }
}

pub fn cos_f32(x: f32) -> f32 {
    sin_f32(fPI2 - x)
}

pub fn cos_f64(x: f64) -> f64 {
    sin_f64(dPI2 - x)
}

pub fn tan_f32(x: f32) -> f32 {
    sin_f32(x) / cos_f32(x)
}

pub fn tan_f64(x: f64) -> f64 {
    sin_f64(x) / cos_f64(x)
}
