use crate::{
    fabs, non_zero_sign, EdgeSegment, EdgeSelector, SignedDistance, Vector2, DISTANCE_DELTA_FACTOR,
};

pub struct TrueDistanceSelector {
    p: Vector2,
    min_distance: SignedDistance,
}

#[derive(Default)]
pub struct TrueDistanceCache {
    point: Vector2,
    abs_distance: f64,
}

impl EdgeSelector for TrueDistanceSelector {
    type Distance = f64;
    type Cache = TrueDistanceCache;

    #[inline]
    fn reset(&mut self, p: Vector2) {
        let delta = DISTANCE_DELTA_FACTOR * (p - self.p).length();
        self.min_distance.distance += non_zero_sign::<f64, f64>(self.min_distance.distance) * delta;
        self.p = p;
    }

    #[inline]
    fn distance(&self, _edges: &[EdgeSegment]) -> Self::Distance {
        self.min_distance.distance
    }

    fn add_edge(
        &mut self,
        cache: &mut Self::Cache,
        edges: &[EdgeSegment],
        _prev_edge: usize,
        edge: usize,
        _next_edge: usize,
    ) {
        let delta = DISTANCE_DELTA_FACTOR * (self.p - cache.point).length();
        if cache.abs_distance - delta <= fabs(self.min_distance.distance) {
            let mut _dummy = 0.0;
            let distance = edges[edge].signed_distance(self.p, &mut _dummy);
            cache.point = self.p;
            cache.abs_distance = fabs(distance.distance);
            if distance < self.min_distance {
                self.min_distance = distance;
            }
        }
    }

    #[inline]
    fn merge(&mut self, other: &Self) {
        if other.min_distance < self.min_distance {
            self.min_distance = other.min_distance.clone();
        }
    }
}
