use super::super::m::{exp_f32, exp_f64};

pub fn sinh_f32(x: f32) -> f32 {
    (exp_f32(2.0 * x) - 1.0) / (2.0 * exp_f32(x))
}

pub fn sinh_f64(x: f64) -> f64 {
    (exp_f64(2.0 * x) - 1.0) / (2.0 * exp_f64(x))
}

pub fn cosh_f32(x: f32) -> f32 {
    (exp_f32(2.0 * x) + 1.0) / (2.0 * exp_f32(x))
}

pub fn cosh_f64(x: f64) -> f64 {
    (exp_f64(2.0 * x) + 1.0) / (2.0 * exp_f64(x))
}

pub fn tanh_f32(x: f32) -> f32 {
    sinh_f32(x) / cosh_f32(x)
}

pub fn tanh_f64(x: f64) -> f64 {
    sinh_f64(x) / cosh_f64(x)
}
