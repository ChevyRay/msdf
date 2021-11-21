use crate::{
    cross_product, dot_product, fabs, non_zero_sign, EdgeSegment, EdgeSelector, SignedDistance,
    Vector2, DISTANCE_DELTA_FACTOR,
};

//PseudoDistanceSelector
pub struct PseudoDistanceSelector {
    min_true_distance: SignedDistance,
    min_negative_pseudo_distance: f64,
    min_positive_pseudo_distance: f64,
    near_edge_index: Option<usize>,
    near_edge_param: f64,
    p: Vector2,
}

#[derive(Default)]
pub struct PseudoDistanceCache {
    point: Vector2,
    abs_distance: f64,
    a_domain_distance: f64,
    b_domain_distance: f64,
    a_pseudo_distance: f64,
    b_pseudo_distance: f64,
}

#[inline]
fn get_pseudo_distance(distance: &mut f64, ep: Vector2, edge_dir: Vector2) -> bool {
    let ts = dot_product(ep, edge_dir);
    if ts > 0.0 {
        let pseudo_distance = cross_product(ep, edge_dir);
        if fabs(pseudo_distance) < fabs(*distance) {
            *distance = pseudo_distance;
            return true;
        }
    }
    return false;
}

impl EdgeSelector for PseudoDistanceSelector {
    type Distance = f64;
    type Cache = PseudoDistanceCache;

    fn reset(&mut self, p: Vector2) {
        let delta = DISTANCE_DELTA_FACTOR * (p - self.p).length();
        self.reset(delta);
        self.p = p;
    }

    fn distance(&self, edges: &[EdgeSegment]) -> Self::Distance {
        self.compute_distance(self.p, edges)
    }

    fn add_edge(
        &mut self,
        cache: &mut Self::Cache,
        edges: &[EdgeSegment],
        prev_edge: usize,
        edge: usize,
        next_edge: usize,
    ) {
        if self.is_edge_relevant(cache, &edges[edge], self.p) {
            let mut param = 0.0;
            let distance = edges[edge].signed_distance(self.p, &mut param);
            self.add_edge_true_distance(edge, &distance, param);

            cache.point = self.p;
            cache.abs_distance = fabs(distance.distance);

            let ap = self.p - edges[edge].point(0.0);
            let bp = self.p - edges[edge].point(1.0);
            let a_dir = edges[edge].direction(0.0).normalize_allow_zero();
            let b_dir = edges[edge].direction(1.0).normalize_allow_zero();
            let prev_dir = edges[prev_edge].direction(1.0).normalize_allow_zero();
            let next_dir = edges[next_edge].direction(0.0).normalize_allow_zero();
            let add = dot_product(ap, (prev_dir + a_dir).normalize_allow_zero());
            let bdd = -dot_product(bp, (b_dir + next_dir).normalize_allow_zero());

            if add > 0.0 {
                let mut pd = distance.distance;
                if get_pseudo_distance(&mut pd, ap, -a_dir) {
                    pd = -pd;
                    self.add_edge_pseudo_distance(pd);
                }
                cache.a_pseudo_distance = pd;
            }
            if bdd > 0.0 {
                let mut pd = distance.distance;
                if get_pseudo_distance(&mut pd, bp, b_dir) {
                    self.add_edge_pseudo_distance(pd);
                }
                cache.b_pseudo_distance = pd;
            }
            cache.a_domain_distance = add;
            cache.b_domain_distance = bdd;
        }
    }

    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl PseudoDistanceSelector {
    fn reset(&mut self, delta: f64) {
        self.min_true_distance.distance +=
            non_zero_sign::<f64, f64>(self.min_true_distance.distance) * delta;
        self.min_negative_pseudo_distance = -fabs(self.min_true_distance.distance);
        self.min_positive_pseudo_distance = fabs(self.min_true_distance.distance);
        self.near_edge_index = None;
        self.near_edge_param = 0.0;
    }

    fn is_edge_relevant(
        &self,
        cache: &PseudoDistanceCache,
        edge: &EdgeSegment,
        p: Vector2,
    ) -> bool {
        let delta = DISTANCE_DELTA_FACTOR * (p - cache.point).length();
        if cache.abs_distance - delta <= fabs(self.min_true_distance.distance) {
            return true;
        }
        if fabs(cache.a_domain_distance) < delta {
            return true;
        }
        if fabs(cache.b_domain_distance) < delta {
            return true;
        }
        if cache.a_domain_distance > 0.0
            && if cache.a_pseudo_distance < 0.0 {
                cache.a_pseudo_distance + delta >= self.min_negative_pseudo_distance
            } else {
                cache.a_pseudo_distance - delta <= self.min_positive_pseudo_distance
            }
        {
            return true;
        }
        if cache.b_domain_distance > 0.0
            && if cache.b_pseudo_distance < 0.0 {
                cache.b_pseudo_distance + delta >= self.min_negative_pseudo_distance
            } else {
                cache.b_pseudo_distance - delta <= self.min_positive_pseudo_distance
            }
        {
            return true;
        }
        false
    }

    fn add_edge_true_distance(&mut self, edge_index: usize, distance: &SignedDistance, param: f64) {
        if distance < &self.min_true_distance {
            self.min_true_distance = distance.clone();
            self.near_edge_index = Some(edge_index);
            self.near_edge_param = param;
        }
    }

    fn add_edge_pseudo_distance(&mut self, distance: f64) {
        if distance <= 0.0 && distance > self.min_negative_pseudo_distance {
            self.min_negative_pseudo_distance = distance;
        }
        if distance >= 0.0 && distance < self.min_positive_pseudo_distance {
            self.min_positive_pseudo_distance = distance;
        }
    }

    fn merge(&mut self, other: &PseudoDistanceSelector) {
        if other.min_true_distance < self.min_true_distance {
            self.min_true_distance = other.min_true_distance.clone();
            self.near_edge_index = other.near_edge_index;
            self.near_edge_param = other.near_edge_param;
        }
        if other.min_negative_pseudo_distance > self.min_negative_pseudo_distance {
            self.min_negative_pseudo_distance = other.min_negative_pseudo_distance;
        }
        if other.min_positive_pseudo_distance < self.min_positive_pseudo_distance {
            self.min_positive_pseudo_distance = other.min_positive_pseudo_distance;
        }
    }

    fn compute_distance(&self, p: Vector2, edges: &[EdgeSegment]) -> f64 {
        let mut min_distance = if self.min_true_distance.distance < 0.0 {
            self.min_negative_pseudo_distance
        } else {
            self.min_positive_pseudo_distance
        };
        if let Some(near_edge_index) = self.near_edge_index {
            let mut distance = self.min_true_distance.clone();
            edges[near_edge_index].distance_to_pseudo_distance(
                &mut distance,
                p,
                self.near_edge_param,
            );
            if fabs(distance.distance) < fabs(min_distance) {
                min_distance = distance.distance;
            }
        }
        min_distance
    }

    fn true_distance(&self) -> SignedDistance {
        self.min_true_distance.clone()
    }
}
