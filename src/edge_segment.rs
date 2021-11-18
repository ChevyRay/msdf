use crate::{EdgeColor, LinearSegment, SignedDistance, Vector2};

pub struct EdgeSegment {
    pub color: EdgeColor,
    pub segment: Segment,
}

pub enum Segment {
    Linear(LinearSegment),
}

impl EdgeSegment {
    pub fn linear(color: EdgeColor, a: Vector2, b: Vector2) -> Self {
        Self {
            color,
            segment: Segment::Linear(LinearSegment(a, b)),
        }
    }

    #[inline]
    pub fn point(&self, param: f64) -> Vector2 {
        match &self.segment {
            Segment::Linear(seg) => seg.point(param),
        }
    }

    #[inline]
    pub fn direction(&self, _param: f64) -> Vector2 {
        match &self.segment {
            Segment::Linear(seg) => seg.direction(),
        }
    }

    #[inline]
    pub fn direction_change(&self, _param: f64) -> Vector2 {
        match &self.segment {
            Segment::Linear(seg) => seg.direction_change(),
        }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        match &self.segment {
            Segment::Linear(seg) => seg.length(),
        }
    }

    #[inline]
    pub fn signed_distance(&self, origin: Vector2) -> SignedDistance {
        match &self.segment {
            Segment::Linear(seg) => seg.signed_distance(origin),
        }
    }

    #[inline]
    pub fn scanline_intersections(&self, x: &mut [f64], dy: &mut [i32], y: f64) -> usize {
        match &self.segment {
            Segment::Linear(seg) => seg.scanline_intersections(x, dy, y),
        }
    }

    #[inline]
    pub fn bounds(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        match &self.segment {
            Segment::Linear(seg) => seg.bounds(l, b, r, t),
        }
    }

    #[inline]
    pub fn reverse(&mut self) {
        match &mut self.segment {
            Segment::Linear(seg) => seg.reverse(),
        }
    }

    #[inline]
    pub fn move_start_point(&mut self, to: Vector2) {
        match &mut self.segment {
            Segment::Linear(seg) => seg.move_start_point(to),
        }
    }

    #[inline]
    pub fn move_end_point(&mut self, to: Vector2) {
        match &mut self.segment {
            Segment::Linear(seg) => seg.move_end_point(to),
        }
    }

    #[inline]
    pub fn split_in_thirds(&self) -> (Self, Self, Self) {
        match &self.segment {
            Segment::Linear(seg) => seg.split_in_thirds(self.color),
        }
    }
}
