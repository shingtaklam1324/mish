pub fn abs_f32(f: f32) -> f32 {
    if f < 0.0 {
        -f
    } else {
        f
    }
}

pub fn abs_f64(f: f64) -> f64 {
    if f < 0.0 {
        -f
    } else {
        f
    }
}

pub fn floor_f32(f: f32) -> f32 {
    f - abs_f32(f % 1.0)
}

pub fn floor_f64(f: f64) -> f64 {
    f - abs_f64(f % 1.0)
}

pub fn ceil_f32(f: f32) -> f32 {
    floor_f32(f) + 1.0
}

pub fn ceil_f64(f: f64) -> f64 {
    floor_f64(f) + 1.0
}

pub fn trunc_f32(f: f32) -> f32 {
    if f < 0.0 {
        ceil_f32(f)
    } else {
        floor_f32(f)
    }
}

pub fn trunc_f64(f: f64) -> f64 {
    if f < 0.0 {
        ceil_f64(f)
    } else {
        floor_f64(f)
    }
}

pub fn round_f32(f: f32) -> f32 {
    match (abs_f32(f) % 1.0 < 0.5, f < 0.0) {
        (true, true) => ceil_f32(f),
        (true, false) => floor_f32(f),
        (false, true) => floor_f32(f),
        (false, false) => ceil_f32(f),
    }
}

pub fn round_f64(f: f64) -> f64 {
    match (abs_f64(f) % 1.0 < 0.5, f < 0.0) {
        (true, true) => ceil_f64(f),
        (true, false) => floor_f64(f),
        (false, true) => floor_f64(f),
        (false, false) => ceil_f64(f),
    }
}

pub fn fract_f32(f: f32) -> f32 {
    f % 1.0
}

pub fn fract_f64(f: f64) -> f64 {
    f % 1.0
}
