use crate::{fabs, non_zero_sign, EdgeSegment, SignedDistance, Vector2};

const DISTANCE_DELTA_FACTOR: f64 = 1.001;

struct MultiDistance {
    r: f64,
    g: f64,
    b: f64,
}

struct MultiAndTrueDistance {
    multi: MultiDistance,
    a: f64,
}

pub trait EdgeSelector {
    type Distance;
    type Cache;

    fn distance(&self) -> Self::Distance;
    fn add_edge(
        &mut self,
        cache: &mut Cache,
        prev_edge: &EdgeSegment,
        edge: &EdgeSegment,
        next_edge: &EdgeSegment,
    );
    fn merge(&mut self, other: &Self);
}

pub struct TrueDistanceSelector {
    p: Vector2,
    min_distance: SignedDistance,
}

#[derive(Default)]
pub struct TrueDistanceEdgeCache {
    point: Vector2,
    abs_distance: f64,
}

/*impl TrueDistanceSelector {
    #[inline]
    pub fn reset(&mut self, p: Vector2) {
        let delta = DISTANCE_DELTA_FACTOR * (p - self.p).length();
        self.min_distance.distance += non_zero_sign::<f64, f64>(self.min_distance.distance) * delta;
        self.p = p;
    }

    //void TrueDistanceSelector::addEdge(EdgeCache &cache, const EdgeSegment *prevEdge, const EdgeSegment *edge, const EdgeSegment *nextEdge) {
    pub fn add_edge(
        &mut self,
        cache: &mut TrueDistanceEdgeCache,
        prev_edge: &EdgeSegment,
        edge: &EdgeSegment,
        next_edge: &EdgeSegment,
    ) {
        let delta = DISTANCE_DELTA_FACTOR * (self.p - cache.point).length();
        if cache.abs_distance - delta <= fabs(self.min_distance.distance) {
            let mut _dummy = 0.0;
            let distance = edge.signed_distance(p, &mut dummy);
            cache.point = p;
            cache.abs_distance = fabs(distance.distance);
            if distance < self.min_distance {
                self.min_distance = distance;
            }
        }
    }
}*/
