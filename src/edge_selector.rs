use crate::{EdgeSegment, Vector2};

/*struct MultiDistance {
    r: f64,
    g: f64,
    b: f64,
}

struct MultiAndTrueDistance {
    multi: MultiDistance,
    a: f64,
}*/

pub(crate) const DISTANCE_DELTA_FACTOR: f64 = 1.001;

pub trait EdgeSelector {
    type Distance;
    type Cache;

    fn reset(&mut self, p: Vector2);
    fn distance(&self, edges: &[EdgeSegment]) -> Self::Distance;
    fn add_edge(
        &mut self,
        cache: &mut Self::Cache,
        edges: &[EdgeSegment],
        prev_edge: usize,
        edge: usize,
        next_edge: usize,
    );
    fn merge(&mut self, other: &Self);
}
