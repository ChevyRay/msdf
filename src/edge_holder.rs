use crate::{EdgeColor, EdgeSegment, Vector2};

#[derive(Default, Clone)]
pub struct EdgeHolder {
    segment: Option<EdgeSegment>,
}

impl From<EdgeSegment> for EdgeHolder {
    fn from(val: EdgeSegment) -> Self {
        Self { segment: Some(val) }
    }
}

impl EdgeHolder {
    #[inline]
    pub fn segment(&self) -> Option<&EdgeSegment> {
        self.segment.as_ref()
    }

    #[inline]
    pub fn segment_mut(&mut self) -> Option<&mut EdgeSegment> {
        self.segment.as_mut()
    }

    #[inline]
    pub fn swap(&mut self, other: &mut EdgeHolder) {
        std::mem::swap(&mut self.segment, &mut other.segment);
    }

    #[inline]
    pub fn new(segment: EdgeSegment) -> Self {
        Self {
            segment: Some(segment),
        }
    }

    #[inline]
    pub fn linear(p0: Vector2, p1: Vector2, color: EdgeColor) -> Self {
        Self::new(EdgeSegment::linear(color, p0, p1))
    }

    #[inline]
    pub fn quadratic(p0: Vector2, p1: Vector2, p2: Vector2, color: EdgeColor) -> Self {
        Self::new(EdgeSegment::quadratic(color, p0, p1, p2))
    }

    #[inline]
    pub fn cubic(p0: Vector2, p1: Vector2, p2: Vector2, p3: Vector2, color: EdgeColor) -> Self {
        Self::new(EdgeSegment::cubic(color, p0, p1, p2, p3))
    }
}
