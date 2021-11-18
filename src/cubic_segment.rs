use crate::{
    cross_product, dot_product, fabs, mix, non_zero_sign, point_bounds, sign, solve_cubic,
    solve_quadratic, sqrt, EdgeColor, EdgeSegment, SignedDistance, Vector2,
};
use num_traits::Zero;

const CUBIC_SEARCH_STARTS: usize = 4;
const CUBIC_SEARCH_STEPS: usize = 4;

#[derive(Clone)]
pub struct CubicSegment(pub Vector2, pub Vector2, pub Vector2, pub Vector2);

impl CubicSegment {
    #[inline]
    pub fn point(&self, param: f64) -> Vector2 {
        let p12 = mix(self.1, self.2, param);
        return mix(
            mix(mix(self.0, self.1, param), p12, param),
            mix(p12, mix(self.2, self.3, param), param),
            param,
        );
    }

    #[inline]
    pub fn direction(&self, param: f64) -> Vector2 {
        let tangent = mix(
            mix(self.1 - self.0, self.2 - self.1, param),
            mix(self.2 - self.1, self.3 - self.2, param),
            param,
        );
        if tangent.is_zero() {
            if param == 0.0 {
                return self.2 - self.0;
            }
            if param == 1.0 {
                return self.3 - self.1;
            }
        }
        tangent
    }

    #[inline]
    pub fn direction_change(&self, param: f64) -> Vector2 {
        mix(
            (self.2 - self.1) - (self.1 - self.0),
            (self.3 - self.2) - (self.2 - self.1),
            param,
        )
    }

    pub fn signed_distance(&self, origin: Vector2) -> SignedDistance {
        let qa = self.0 - origin;
        let ab = self.1 - self.0;
        let br = self.2 - self.1 - ab;
        let as_ = (self.3 - self.2) - (self.2 - self.1) - br;

        let mut ep_dir = self.direction(0.0);
        let mut min_distance = non_zero_sign::<f64, f64>(cross_product(ep_dir, qa)) * qa.length();
        let mut param = -dot_product(qa, ep_dir) / dot_product(ep_dir, ep_dir);
        {
            ep_dir = self.direction(1.0);
            let distance = (self.3 - origin).length();
            if distance < fabs(min_distance) {
                min_distance =
                    non_zero_sign::<f64, f64>(cross_product(ep_dir, self.3 - origin)) * distance;
                param =
                    dot_product(ep_dir - (self.3 - origin), ep_dir) / dot_product(ep_dir, ep_dir);
            }
        }
        for i in 0..=CUBIC_SEARCH_STARTS {
            let mut t = (i as f64) / (CUBIC_SEARCH_STARTS as f64);
            let mut qe = qa + ab * 3.0 * t + br * 3.0 * t * t + as_ * t * t * t;
            for _ in 0..CUBIC_SEARCH_STEPS {
                let d1 = as_ * 3.0 * t * t + br * 6.0 * t + ab * 3.0;
                let d2 = as_ * 6.0 * t + br * 6.0;
                t -= dot_product(qe, d1) / (dot_product(d1, d1) + dot_product(qe, d2));
                if t <= 0.0 || t >= 1.0 {
                    break;
                }
                qe = qa + ab * 3.0 * t + br * 3.0 * t * t + as_ * t * t * t;
                let distance = qe.length();
                if distance < fabs(min_distance) {
                    min_distance =
                        non_zero_sign::<f64, f64>(cross_product(self.direction(t), qe)) * distance;
                    param = t;
                }
            }
        }

        if param >= 0.0 && param <= 1.0 {
            SignedDistance::new(min_distance, 0.0)
        } else if param < 0.5 {
            SignedDistance::new(
                min_distance,
                fabs(dot_product(self.direction(0.0).normalize(), qa.normalize())),
            )
        } else {
            SignedDistance::new(
                min_distance,
                fabs(dot_product(
                    self.direction(1.0).normalize(),
                    (self.3 - origin).normalize(),
                )),
            )
        }
    }

    pub fn scanline_intersections(&self, x: &mut [f64], dy: &mut [i32], y: f64) -> usize {
        let mut total = 0;
        let mut next_dy = if y > self.0.y { 1 } else { -1 };
        x[total] = self.0.x;
        if self.0.y == y {
            if self.0.y < self.1.y
                || (self.0.y == self.1.y
                    && (self.0.y < self.2.y || (self.0.y == self.2.y && self.0.y < self.3.y)))
            {
                dy[total] = 1;
                total += 1;
            } else {
                next_dy = 1;
            }
        }
        {
            let ab = self.1 - self.0;
            let br = self.2 - self.1 - ab;
            let as_ = (self.3 - self.2) - (self.2 - self.1) - br;
            let mut t = [0.0, 0.0, 0.0];
            let solutions =
                solve_cubic(&mut t, as_.y, 3.0 * br.y, 3.0 * ab.y, self.0.y - y).unwrap_or(0);
            if solutions >= 2 {
                if t[0] > t[1] {
                    t.swap(0, 1);
                }
                if solutions >= 3 && t[1] > t[2] {
                    t.swap(1, 2);
                    if t[0] > t[1] {
                        t.swap(0, 1);
                    }
                }
            }
            let mut i = 0;
            while i < solutions && total < 3 {
                if t[i] >= 0.0 && t[i] <= 1.0 {
                    x[total] = self.0.x
                        + 3.0 * t[i] * ab.x
                        + 3.0 * t[i] * t[i] * br.x
                        + t[i] * t[i] * t[i] * as_.x;
                    if (next_dy as f64) * (ab.y + 2.0 * t[i] * br.y + t[i] * t[i] * as_.y) >= 0.0 {
                        dy[total] = next_dy;
                        total += 1;
                        next_dy = -next_dy;
                    }
                }
                i += 1;
            }
        }
        if self.3.y == y {
            if next_dy > 0 && total > 0 {
                total -= 1;
                next_dy = -1;
            }
            if (self.3.y < self.2.y
                || (self.3.y == self.2.y
                    && (self.3.y < self.1.y || (self.3.y == self.1.y && self.3.y < self.0.y))))
                && total < 3
            {
                x[total] = self.3.x;
                if next_dy < 0 {
                    dy[total] = -1;
                    total += 1;
                    next_dy = 1;
                }
            }
        }
        let test = if y >= self.3.y { 1 } else { -1 };
        if next_dy != test {
            if total > 0 {
                total -= 1;
            } else {
                if fabs(self.3.y - y) < fabs(self.0.y - y) {
                    x[total] = self.3.x;
                }
                dy[total] = next_dy;
                total += 1;
            }
        }
        return total;
    }

    #[inline]
    pub fn bounds(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        point_bounds(&self.0, l, b, r, t);
        point_bounds(&self.3, l, b, r, t);
        let a0 = self.1 - self.0;
        let a1 = (self.2 - self.1 - a0) * 2.0;
        let a2 = self.3 - self.2 * 3.0 + self.1 * 3.0 - self.0;
        let mut params = [0.0, 0.0];
        let solutions = solve_quadratic(&mut params, a2.x, a1.x, a0.x).unwrap_or(0);
        for &param in &params[..solutions] {
            if param > 0.0 && param < 1.0 {
                point_bounds(&self.point(param), l, b, r, t);
            }
        }
        let solutions = solve_quadratic(&mut params, a2.y, a1.y, a0.y).unwrap_or(0);
        for &param in &params[..solutions] {
            if param > 0.0 && param < 1.0 {
                point_bounds(&self.point(param), l, b, r, t);
            }
        }
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.0, &mut self.3);
        std::mem::swap(&mut self.1, &mut self.2);
    }

    #[inline]
    pub fn move_start_point(&mut self, to: Vector2) {
        self.1 += to - self.0;
        self.0 = to;
    }

    #[inline]
    pub fn move_end_point(&mut self, to: Vector2) {
        self.2 += to - self.3;
        self.3 = to;
    }

    #[inline]
    pub fn split_in_thirds(&self, color: EdgeColor) -> (EdgeSegment, EdgeSegment, EdgeSegment) {
        (
            EdgeSegment::cubic(
                color,
                self.0,
                {
                    if self.0 == self.1 {
                        self.0
                    } else {
                        mix(self.0, self.1, 1.0 / 3.0)
                    }
                },
                mix(
                    mix(self.0, self.1, 1.0 / 3.0),
                    mix(self.1, self.2, 1.0 / 3.0),
                    1.0 / 3.0,
                ),
                self.point(1.0 / 3.00),
            ),
            EdgeSegment::cubic(
                color,
                self.point(1.0 / 3.0),
                mix(
                    mix(
                        mix(self.0, self.1, 1.0 / 3.0),
                        mix(self.1, self.2, 1.0 / 3.0),
                        1.0 / 3.0,
                    ),
                    mix(
                        mix(self.1, self.2, 1.0 / 3.0),
                        mix(self.2, self.3, 1.0 / 3.0),
                        1.0 / 3.0,
                    ),
                    2.0 / 3.0,
                ),
                mix(
                    mix(
                        mix(self.0, self.1, 2.0 / 3.00),
                        mix(self.1, self.2, 2.0 / 3.0),
                        2.0 / 3.0,
                    ),
                    mix(
                        mix(self.1, self.2, 2.0 / 3.0),
                        mix(self.2, self.3, 2.0 / 3.0),
                        2.0 / 3.0,
                    ),
                    1.0 / 3.0,
                ),
                self.point(2.0 / 3.0),
            ),
            EdgeSegment::cubic(
                color,
                self.point(2.0 / 3.0),
                mix(
                    mix(self.1, self.2, 2.0 / 3.0),
                    mix(self.2, self.3, 2.0 / 3.0),
                    2.0 / 3.0,
                ),
                {
                    if self.2 == self.3 {
                        self.3
                    } else {
                        mix(self.2, self.3, 2.0 / 3.0)
                    }
                },
                self.3,
            ),
        )
    }

    pub fn deconverge(&mut self, param: i32, amount: f64) {
        let dir = self.direction(param as f64);
        let normal = dir.get_orthonormal(true);
        let h = dot_product(self.direction_change(param as f64) - dir, normal);
        if param == 0 {
            self.1 += (dir + normal * sign::<f64, f64>(h) * sqrt(fabs(h))) * amount;
        } else if param == 1 {
            self.2 -= (dir - normal * sign::<f64, f64>(h) * sqrt(fabs(h))) * amount;
        }
    }
}
