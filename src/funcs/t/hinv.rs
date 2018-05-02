use super::super::m::{ln_f32, ln_f64, sqrt_f32, sqrt_f64};
use super::super::o::{abs_f32, abs_f64};

pub fn arsinh_f32(x: f32) -> f32 {
    ln_f32(x + sqrt_f32(x * x + 1.0))
}

pub fn arsinh_f64(x: f64) -> f64 {
    ln_f64(x + sqrt_f64(x * x + 1.0))
}

pub fn arcosh_f32(x: f32) -> f32 {
    if x < 1.0 {
        panic!("")
    }
    ln_f32(x + sqrt_f32(x * x - 1.0))
}

pub fn arcosh_f64(x: f64) -> f64 {
    if x < 1.0 {
        panic!("")
    }
    ln_f64(x + sqrt_f64(x * x - 1.0))
}

pub fn artanh_f32(x: f32) -> f32 {
    if abs_f32(x) > 1.0 {
        panic!("")
    }
    0.5 * ln_f32((1.0 + x) / (1.0 - x))
}

pub fn artanh_f64(x: f64) -> f64 {
    if abs_f64(x) > 1.0 {
        panic!("")
    }
    0.5 * ln_f64((1.0 + x) / (1.0 - x))
}
