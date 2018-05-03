extern crate mish;
#[macro_use]
extern crate quickcheck;

fn almost_f64(a: f64, b: f64, e: f64) -> bool {
    let c = (a - b).abs() < e;
    if !c {
        println!("Expected {} got {}", b, a);
    }
    c
}

// These are the confidence in the functions, with the range at the end

quickcheck! {
    fn sin_f64(x: f64) -> bool {
        if x > 1.57 || x < -1.57 {
            true
        } else {
            almost_f64(mish::sin(x), x.sin(), 2E-3)
        }
    }
}

quickcheck! {
    fn cos_f64(x: f64) -> bool {
        if x > 3.14 || x < 0.0 {
            true
        } else {
            almost_f64(mish::cos(x), x.cos(), 2E-3)
        }
    }
}
quickcheck! {
    fn tan_f64(x: f64) -> bool {
        if x > 1.57 || x < -1.57 {
            true
        } else {
            almost_f64(mish::tan(x), x.tan(), 8E-3)
        }
    }
}

quickcheck! {
    fn asin_sin_f64(x: f64) -> bool {
        if x > 1.57 || x < -1.57 {
            true
        } else {
            almost_f64(mish::asin(mish::sin(x)), x, 5E-3)
        }
    }
}

// ACOS
// ATAN

quickcheck! {
    fn ln_f64(x: f64) -> bool {
        if !mish::is_fin(x) || x <= 0.0 {
            true
        } else {
            almost_f64(mish::ln(x), x.ln(), 1E-1)
        }
    }
}
