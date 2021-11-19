use crate::{
    cross_product, dot_product, fabs, mix, non_zero_sign, point_bounds, sign, EdgeColor,
    EdgeSegment, SignedDistance, Vector2,
};
use num_traits::Zero;

#[derive(Clone)]
pub struct LinearSegment(pub Vector2, pub Vector2);

impl LinearSegment {
    #[inline]
    pub fn point(&self, param: f64) -> Vector2 {
        mix(self.0, self.1, param)
    }

    #[inline]
    pub fn direction(&self) -> Vector2 {
        self.1 - self.0
    }

    #[inline]
    pub fn direction_change(&self) -> Vector2 {
        Vector2::zero()
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.1 - self.0).length()
    }

    pub fn signed_distance(&self, origin: Vector2, param: &mut f64) -> SignedDistance {
        let aq = origin - self.0;
        let ab = self.1 - self.0;
        *param = dot_product(aq, ab) / dot_product(ab, ab);
        let eq = if *param > 0.5 { self.1 } else { self.0 } - origin;
        let endpoint_distance = eq.length();
        if *param > 0.0 && *param < 1.0 {
            let ortho_distance = dot_product(ab.get_orthonormal(false), aq);
            if ortho_distance.abs() < endpoint_distance.abs() {
                return SignedDistance::new(ortho_distance, 0.0);
            }
        }
        SignedDistance::new(
            non_zero_sign::<f64, f64>(cross_product(aq, ab)) * endpoint_distance,
            fabs(dot_product(ab.normalize(), eq.normalize())),
        )
    }

    pub fn scanline_intersections(&self, x: &mut [f64], dy: &mut [i32], y: f64) -> usize {
        if (y >= self.0.y && y < self.1.y) || (y >= self.1.y && y < self.0.y) {
            let param: f64 = (y - self.0.y) / (self.1.y - self.0.y);
            x[0] = mix(self.0.x, self.1.x, param);
            dy[0] = sign::<f64, i32>(self.1.y - self.0.y);
            1
        } else {
            0
        }
    }

    #[inline]
    pub fn bounds(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        point_bounds(&self.0, l, b, r, t);
        point_bounds(&self.1, l, b, r, t);
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1)
    }

    #[inline]
    pub fn move_start_point(&mut self, to: Vector2) {
        self.0 = to;
    }

    #[inline]
    pub fn move_end_point(&mut self, to: Vector2) {
        self.0 = to;
    }

    #[inline]
    pub fn split_in_thirds(&self, color: EdgeColor) -> (EdgeSegment, EdgeSegment, EdgeSegment) {
        let m1 = self.point(1.0 / 3.0);
        let m2 = self.point(2.0 / 3.0);
        (
            EdgeSegment::linear(color, self.0, m1),
            EdgeSegment::linear(color, m1, m2),
            EdgeSegment::linear(color, m2, self.1),
        )
    }
}
