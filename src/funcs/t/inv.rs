use super::super::o::{abs_f32, abs_f64};
use super::consts;

pub fn acos_f32_(t: f32, n: usize) -> f32 {
    let mut t = t;
    if 1.0 < abs_f32(t) {
        panic!("")
    }
    let mut theta = 0.0;
    let mut x1 = 1.0;
    let mut y1 = 0.0;
    let mut p2 = 1.0;
    let mut x2;
    let mut y2;
    let mut sz2;
    let mut sigma;
    let mut angle = consts::LUT_A_F32[0];

    for j in 0..n {
        if y1 < 0.0 {
            sz2 = -1.0;
        } else {
            sz2 = 1.0;
        }

        if t <= x1 {
            sigma = sz2;
        } else {
            sigma = -sz2;
        }

        if j < 60 {
            angle = consts::LUT_A_F32[j]
        } else {
            angle = angle / 2.0
        }
        for _ in 0..2 {
            x2 = x1 - sigma * p2 * y1;
            y2 = sigma * p2 * x1 + y1;
            x1 = x2;
            y1 = y2;
        }
        theta = theta + 2.0 * sigma * angle;
        t = t + t * p2 * p2;
        p2 = p2 / 2.0;
    }
    theta
}

pub fn acos_f32(t: f32) -> f32 {
    acos_f32_(t, 25)
}

pub fn acos_f64_(t: f64, n: usize) -> f64 {
    let mut t = t;
    if 1.0 < abs_f64(t) {
        panic!("")
    }
    let mut theta = 0.0;
    let mut x1 = 1.0;
    let mut y1 = 0.0;
    let mut p2 = 1.0;
    let mut x2;
    let mut y2;
    let mut sz2;
    let mut sigma;
    let mut angle = consts::LUT_A_F64[0];

    for j in 0..n {
        if y1 < 0.0 {
            sz2 = -1.0;
        } else {
            sz2 = 1.0;
        }

        if t <= x1 {
            sigma = sz2;
        } else {
            sigma = -sz2;
        }

        if j < 60 {
            angle = consts::LUT_A_F64[j]
        } else {
            angle = angle / 2.0
        }
        for _ in 0..2 {
            x2 = x1 - sigma * p2 * y1;
            y2 = sigma * p2 * x1 + y1;
            x1 = x2;
            y1 = y2;
        }
        theta = theta + 2.0 * sigma * angle;
        t = t + t * p2 * p2;
        p2 = p2 / 2.0;
    }
    theta
}

pub fn acos_f64(t: f64) -> f64 {
    acos_f64_(t, 25)
}

pub fn asin_f32_(t: f32, n: usize) -> f32 {
    let mut t = t;
    if 1.0 < abs_f32(t) {
        panic!("")
    }
    let mut theta = 0.0;
    let mut x1 = 1.0;
    let mut y1 = 0.0;
    let mut p2 = 1.0;
    let mut x2;
    let mut y2;
    let mut sz1;
    let mut sigma;
    let mut angle = consts::LUT_A_F32[0];

    for j in 0..n {
        if x1 < 0.0 {
            sz1 = -1.0;
        } else {
            sz1 = 1.0;
        }

        if y1 <= t {
            sigma = sz1;
        } else {
            sigma = -sz1;
        }

        if j < 60 {
            angle = consts::LUT_A_F32[j]
        } else {
            angle = angle / 2.0
        }
        for _ in 0..2 {
            x2 = x1 - sigma * p2 * y1;
            y2 = sigma * p2 * x1 + y1;
            x1 = x2;
            y1 = y2;
        }
        theta = theta + 2.0 * sigma * angle;
        t = t + t * p2 * p2;
        p2 = p2 / 2.0;
    }
    theta
}

pub fn asin_f32(t: f32) -> f32 {
    asin_f32_(t, 25)
}

pub fn asin_f64_(t: f64, n: usize) -> f64 {
    let mut t = t;
    if 1.0 < abs_f64(t) {
        panic!("")
    }
    let mut theta = 0.0;
    let mut x1 = 1.0;
    let mut y1 = 0.0;
    let mut p2 = 1.0;
    let mut x2;
    let mut y2;
    let mut sz1;
    let mut sigma;
    let mut angle = consts::LUT_A_F64[0];

    for j in 0..n {
        if x1 < 0.0 {
            sz1 = -1.0;
        } else {
            sz1 = 1.0;
        }

        if y1 <= t {
            sigma = sz1;
        } else {
            sigma = -sz1;
        }

        if j < 60 {
            angle = consts::LUT_A_F64[j]
        } else {
            angle = angle / 2.0
        }
        for _ in 0..2 {
            x2 = x1 - sigma * p2 * y1;
            y2 = sigma * p2 * x1 + y1;
            x1 = x2;
            y1 = y2;
        }
        theta = theta + 2.0 * sigma * angle;
        t = t + t * p2 * p2;
        p2 = p2 / 2.0;
    }
    theta
}

pub fn asin_f64(t: f64) -> f64 {
    asin_f64_(t, 25)
}

// angle = y/x
pub fn atan_f32_(x: f32, y: f32, n: usize) -> f32 {
    let mut x1 = x;
    let mut y1 = y;
    let mut x2;
    let mut y2;
    let sf;
    let mut sigma;
    let mut theta = 0.0;
    let mut p2 = 1.0;
    let mut angle = consts::LUT_A_F32[0];

    if x1 < 0.0 && y1 < 0.0 {
        x1 = -x1;
        y1 = -y1;
    }

    if x1 < 0.0 {
        x1 = -x1;
        sf = -1.0;
    } else if y1 < 0.0 {
        y1 = -y1;
        sf = -1.0
    } else {
        sf = 1.0
    }

    for j in 0..n {
        if y1 <= 0.0 {
            sigma = 1.0
        } else {
            sigma = -1.0
        }

        if j < 60 {
            angle = consts::LUT_A_F32[j]
        } else {
            angle = angle / 2.0
        }

        x2 = x1 - sigma * p2 * y1;
        y2 = sigma * p2 * x1 + y1;
        theta = theta - sigma * angle;

        x1 = x2;
        y1 = y2;

        p2 = p2 / 2.0;
    }

    sf * theta
}

pub fn atan_f32(x: f32, y: f32) -> f32 {
    atan_f32_(x, y, 25)
}

pub fn atan_f64_(x: f64, y: f64, n: usize) -> f64 {
    let mut x1 = x;
    let mut y1 = y;
    let mut x2;
    let mut y2;
    let sf;
    let mut sigma;
    let mut theta = 0.0;
    let mut p2 = 1.0;
    let mut angle = consts::LUT_A_F64[0];

    if x1 < 0.0 && y1 < 0.0 {
        x1 = -x1;
        y1 = -y1;
    }

    if x1 < 0.0 {
        x1 = -x1;
        sf = -1.0;
    } else if y1 < 0.0 {
        y1 = -y1;
        sf = -1.0
    } else {
        sf = 1.0
    }

    for j in 0..n {
        if y1 <= 0.0 {
            sigma = 1.0
        } else {
            sigma = -1.0
        }

        if j < 60 {
            angle = consts::LUT_A_F64[j]
        } else {
            angle = angle / 2.0
        }

        x2 = x1 - sigma * p2 * y1;
        y2 = sigma * p2 * x1 + y1;
        theta = theta - sigma * angle;

        x1 = x2;
        y1 = y2;

        p2 = p2 / 2.0;
    }

    sf * theta
}

pub fn atan_f64(x: f64, y: f64) -> f64 {
    atan_f64_(x, y, 25)
}
