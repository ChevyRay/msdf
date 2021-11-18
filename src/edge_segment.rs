use crate::{EdgeColor, LinearSegment, QuadraticSegment, SignedDistance, Vector2};

pub struct EdgeSegment {
    pub color: EdgeColor,
    pub segment: Segment,
}

pub enum Segment {
    Linear(LinearSegment),
    Quadratic(QuadraticSegment),
}

impl EdgeSegment {
    #[inline]
    pub fn linear(color: EdgeColor, p0: Vector2, p1: Vector2) -> Self {
        Self {
            color,
            segment: Segment::Linear(LinearSegment(p0, p1)),
        }
    }

    #[inline]
    pub fn quadratic(color: EdgeColor, p0: Vector2, p1: Vector2, p2: Vector2) -> Self {
        Self {
            color,
            segment: Segment::Quadratic(QuadraticSegment(
                p0,
                if p1 == p0 || p1 == p2 {
                    (p0 + p2) * 0.5
                } else {
                    p1
                },
                p2,
            )),
        }
    }

    #[inline]
    pub fn point(&self, param: f64) -> Vector2 {
        match &self.segment {
            Segment::Linear(seg) => seg.point(param),
            Segment::Quadratic(seg) => seg.point(param),
        }
    }

    #[inline]
    pub fn direction(&self, param: f64) -> Vector2 {
        match &self.segment {
            Segment::Linear(seg) => seg.direction(),
            Segment::Quadratic(seg) => seg.direction(param),
        }
    }

    #[inline]
    pub fn direction_change(&self, _param: f64) -> Vector2 {
        match &self.segment {
            Segment::Linear(seg) => seg.direction_change(),
            Segment::Quadratic(seg) => seg.direction_change(),
        }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        match &self.segment {
            Segment::Linear(seg) => seg.length(),
            Segment::Quadratic(seg) => seg.length(),
        }
    }

    #[inline]
    pub fn signed_distance(&self, origin: Vector2) -> SignedDistance {
        match &self.segment {
            Segment::Linear(seg) => seg.signed_distance(origin),
            Segment::Quadratic(seg) => seg.signed_distance(origin),
        }
    }

    #[inline]
    pub fn scanline_intersections(&self, x: &mut [f64], dy: &mut [i32], y: f64) -> usize {
        match &self.segment {
            Segment::Linear(seg) => seg.scanline_intersections(x, dy, y),
            Segment::Quadratic(seg) => seg.scanline_intersections(x, dy, y),
        }
    }

    #[inline]
    pub fn bounds(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        match &self.segment {
            Segment::Linear(seg) => seg.bounds(l, b, r, t),
            Segment::Quadratic(seg) => seg.bounds(l, b, r, t),
        }
    }

    #[inline]
    pub fn reverse(&mut self) {
        match &mut self.segment {
            Segment::Linear(seg) => seg.reverse(),
            Segment::Quadratic(seg) => seg.reverse(),
        }
    }

    #[inline]
    pub fn move_start_point(&mut self, to: Vector2) {
        match &mut self.segment {
            Segment::Linear(seg) => seg.move_start_point(to),
            Segment::Quadratic(seg) => seg.move_start_point(to),
        }
    }

    #[inline]
    pub fn move_end_point(&mut self, to: Vector2) {
        match &mut self.segment {
            Segment::Linear(seg) => seg.move_end_point(to),
            Segment::Quadratic(seg) => seg.move_end_point(to),
        }
    }

    #[inline]
    pub fn split_in_thirds(&self) -> (Self, Self, Self) {
        match &self.segment {
            Segment::Linear(seg) => seg.split_in_thirds(self.color),
            Segment::Quadratic(seg) => seg.split_in_thirds(self.color),
        }
    }
}

#[inline]
pub(crate) fn point_bounds(p: &Vector2, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
    if p.x < *l {
        *l = p.x;
    }
    if p.y < *b {
        *b = p.y;
    }
    if p.x > *r {
        *r = p.x;
    }
    if p.y > *t {
        *t = p.y;
    }
}
