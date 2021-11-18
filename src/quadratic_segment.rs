use crate::{
    cross_product, dot_product, fabs, log, mix, non_zero_sign, point_bounds, solve_cubic,
    solve_quadratic, sqrt, EdgeColor, EdgeSegment, SignedDistance, Vector2,
};
use num_traits::Zero;

#[derive(Clone)]
pub struct QuadraticSegment(pub Vector2, pub Vector2, pub Vector2);

impl QuadraticSegment {
    #[inline]
    pub fn point(&self, param: f64) -> Vector2 {
        mix(
            mix(self.0, self.1, param),
            mix(self.1, self.2, param),
            param,
        )
    }

    #[inline]
    pub fn direction(&self, param: f64) -> Vector2 {
        let tangent = mix(self.1 - self.0, self.2 - self.1, param);
        if tangent.is_zero() {
            self.2 - self.0
        } else {
            tangent
        }
    }

    #[inline]
    pub fn direction_change(&self) -> Vector2 {
        (self.2 - self.1) - (self.1 - self.0)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        let ab = self.1 - self.0;
        let br = self.2 - self.1 - ab;
        let abab = dot_product(ab, ab);
        let abbr = dot_product(ab, br);
        let brbr = dot_product(br, br);
        let ab_len = sqrt(abab);
        let br_len = sqrt(brbr);
        let crs = cross_product(ab, br);
        let h = sqrt(abab + abbr + abbr + brbr);
        return (br_len * ((abbr + brbr) * h - abbr * ab_len)
            + crs * crs * log((br_len * h + abbr + brbr) / (br_len * ab_len + abbr)))
            / (brbr * br_len);
    }

    pub fn signed_distance(&self, origin: Vector2) -> SignedDistance {
        let qa = self.0 - origin;
        let ab = self.1 - self.0;
        let br = self.2 - self.1 - ab;
        let a = dot_product(br, br);
        let b = 3.0 * dot_product(ab, br);
        let c = 2.0 * dot_product(ab, ab) + dot_product(qa, br);
        let d = dot_product(qa, ab);
        let mut t = [0.0, 0.0, 0.0];
        let solutions = solve_cubic(&mut t, a, b, c, d).unwrap_or(0);

        let mut ep_dir = self.direction(0.0);
        let mut min_distance = non_zero_sign::<f64, f64>(cross_product(ep_dir, qa)) * qa.length();
        let mut param = -dot_product(qa, ep_dir) / dot_product(ep_dir, ep_dir);
        {
            ep_dir = self.direction(1.0);
            let distance = (self.2 - origin).length();
            if distance < fabs(min_distance) {
                min_distance =
                    non_zero_sign::<f64, f64>(cross_product(ep_dir, self.2 - origin)) * distance;
                param = dot_product(origin - self.1, ep_dir) / dot_product(ep_dir, ep_dir);
            }
        }
        for i in 0..solutions {
            if t[i] > 0.0 && t[i] < 1.0 {
                let qe = self.0 + ab * 2.0 * t[i] + br * t[i] * t[i] - origin;
                let distance = qe.length();
                if distance <= fabs(min_distance) {
                    min_distance =
                        non_zero_sign::<f64, f64>(cross_product(self.direction(t[i]), qe))
                            * distance;
                    param = t[i];
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
                    (self.2 - origin).normalize(),
                )),
            )
        }
    }

    pub fn scanline_intersections(&self, x: &mut [f64], dy: &mut [i32], y: f64) -> usize {
        let mut total = 0;
        let mut next_dy = if y > self.0.y { 1 } else { -1 };
        x[total] = self.0.x;
        if self.0.y == y {
            if self.0.y < self.1.y || (self.0.y == self.1.y && self.0.y < self.2.y) {
                dy[total] = 1;
                total += 1;
            } else {
                next_dy = 1;
            }
        }
        {
            let ab = self.1 - self.0;
            let br = self.2 - self.1 - ab;
            let mut t = [0.0, 0.0];
            let solutions = solve_quadratic(&mut t, br.y, 2.0 * ab.y, self.0.y - y).unwrap_or(0);
            if solutions >= 2 && t[0] > t[1] {
                t.swap(0, 1);
            }
            let mut i = 0;
            while i < solutions && total < 2 {
                if t[i] >= 0.0 && t[i] <= 1.0 {
                    x[total] = self.0.x + 2.0 * t[i] * ab.x + t[i] * t[i] * br.x;
                    if (next_dy as f64) * (ab.y + t[i] * br.y) >= 0.0 {
                        dy[total] = next_dy;
                        total += 1;
                        next_dy = -next_dy;
                    }
                }
                i += 1;
            }
        }
        if self.2.y == y {
            if next_dy > 0 && total > 0 {
                total -= 1;
                next_dy = -1;
            }
            if (self.2.y < self.1.y || (self.2.y == self.1.y && self.2.y < self.0.y)) && total < 2 {
                x[total] = self.2.x;
                if next_dy < 0 {
                    dy[total] = -1;
                    total += 1;
                    next_dy = 1;
                }
            }
        }

        let dir = if y <= self.2.y { 1 } else { -1 };
        if next_dy != dir {
            if total > 0 {
                total -= 1;
            } else {
                if fabs(self.2.y - y) < fabs(self.0.y - y) {
                    x[total] = self.2.x;
                }
                dy[total] = next_dy;
                total += 1;
            }
        }
        total
    }

    #[inline]
    pub fn bounds(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        point_bounds(&self.0, l, b, r, t);
        point_bounds(&self.2, l, b, r, t);
        let bot = (self.1 - self.0) - (self.2 - self.1);
        if bot.x != 0.0 {
            let param = (self.1.x - self.0.x) / bot.x;
            if param > 0.0 && param < 1.0 {
                point_bounds(&self.point(param), l, b, r, t);
            }
        }
        if bot.y != 0.0 {
            let param = (self.1.y - self.0.y) / bot.y;
            if param > 0.0 && param < 1.0 {
                point_bounds(&self.point(param), l, b, r, t);
            }
        }
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.0, &mut self.2);
    }

    #[inline]
    pub fn move_start_point(&mut self, to: Vector2) {
        let orig_s_dir = self.0 - self.1;
        let orig_p1 = self.1;
        self.1 += (self.2 - self.1)
            * (cross_product(self.0 - self.1, to - self.0)
                / cross_product(self.0 - self.1, self.2 - self.1));
        self.0 = to;
        if dot_product(orig_s_dir, self.0 - self.1) < 0.0 {
            self.1 = orig_p1;
        }
    }

    #[inline]
    pub fn move_end_point(&mut self, to: Vector2) {
        let orig_e_dir = self.2 - self.1;
        let orig_p1 = self.1;
        self.1 += (self.0 - self.1)
            * (cross_product(self.2 - self.1, to - self.2)
                / cross_product(self.2 - self.1, self.0 - self.1));
        self.2 = to;
        if dot_product(orig_e_dir, self.2 - self.1) < 0.0 {
            self.1 = orig_p1;
        }
    }

    #[inline]
    pub fn split_in_thirds(&self, color: EdgeColor) -> (EdgeSegment, EdgeSegment, EdgeSegment) {
        let m1 = self.point(1.0 / 3.0);
        let m2 = self.point(2.0 / 3.0);
        (
            EdgeSegment::quadratic(color, self.0, mix(self.0, self.1, 1.0 / 3.0), m1),
            EdgeSegment::quadratic(
                color,
                m1,
                mix(
                    mix(self.0, self.1, 5.0 / 9.0),
                    mix(self.1, self.2, 4.0 / 9.0),
                    0.5,
                ),
                m2,
            ),
            EdgeSegment::quadratic(color, m2, mix(self.1, self.2, 2.0 / 3.0), self.2),
        )
    }
}
