use crate::{acos, cos, fabs, pow, sqrt};
use std::f64::consts::TAU;

const TOO_LARGE_RATIO: f64 = 1e12;

pub fn solve_quadratic(x: &mut [f64], a: f64, b: f64, c: f64) -> Option<usize> {
    if a == 0.0 || fabs(b) + fabs(c) > TOO_LARGE_RATIO * fabs(a) {
        if b == 0.0 || fabs(c) > TOO_LARGE_RATIO * fabs(b) {
            return if c == 0.0 { None } else { Some(0) };
        }
        x[0] = -c / b;
        return Some(1);
    }
    let dscr = b * b - 4.0 * a * c;
    if dscr > 0.0 {
        let dscr = sqrt(dscr);
        x[0] = (-b + dscr) / (2.0 * a);
        x[1] = (-b - dscr) / (2.0 * a);
        Some(2)
    } else if dscr == 0.0 {
        x[0] = -b / (2.0 * a);
        Some(1)
    } else {
        Some(0)
    }
}

#[inline]
fn solve_cubic_normed(x: &mut [f64], a: f64, b: f64, c: f64) -> usize {
    let a2 = a * a;
    let q = (a2 - 3.0 * b) / 9.0;
    let r = (a * (2.0 * a2 - 9.0 * b) + 27.0 * c) / 54.0;
    let r2 = r * r;
    let q3 = q * q * q;
    if r2 < q3 {
        let mut t = r / sqrt(q3);
        if t < -1.0 {
            t = -1.0;
        }
        if t > 1.0 {
            t = 1.0;
        }
        let t = acos(t);
        let a = a / 3.0;
        let q = -2.0 * sqrt(q);
        x[0] = q * cos(t / 3.0) - a;
        x[1] = q * cos((t + TAU) / 3.0) - a;
        x[2] = q * cos((t - TAU) / 3.0) - a;
        3
    } else {
        let mut aa = -pow(fabs(r) + sqrt(r2 - q3), 1.0 / 3.0);
        if r < 0.0 {
            aa = -aa;
        }
        let bb = if a == 0.0 { 0.0 } else { q / aa };
        let a = a / 3.0;
        x[0] = (aa + bb) - a;
        x[1] = -0.5 * (aa + bb) - a;
        x[2] = 0.5 * sqrt(3.0) * (aa - bb);
        if fabs(x[2]) < 1e-14 {
            2
        } else {
            1
        }
    }
}

pub fn solve_cubic(x: &mut [f64], a: f64, b: f64, c: f64, d: f64) -> Option<usize> {
    if a != 0.0 {
        let bn = b / a;
        let cn = c / a;
        let dn = d / a;
        if fabs(bn) < TOO_LARGE_RATIO && fabs(cn) < TOO_LARGE_RATIO && fabs(dn) < TOO_LARGE_RATIO {
            return Some(solve_cubic_normed(x, bn, cn, dn));
        }
    }
    solve_quadratic(x, b, c, d)
}
