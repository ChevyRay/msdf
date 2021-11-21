use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub struct SignedDistance {
    pub distance: f64,
    pub dot: f64,
}

impl SignedDistance {
    #[inline]
    pub const fn new(distance: f64, dot: f64) -> Self {
        Self { distance, dot }
    }

    #[inline]
    pub const fn infinite() -> Self {
        Self::new(-1e240, 1.0)
    }
}

impl PartialOrd for SignedDistance {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a_dist = self.distance.abs();
        let b_dist = self.distance.abs();
        if a_dist < b_dist {
            Some(Ordering::Less)
        } else if a_dist > b_dist {
            Some(Ordering::Greater)
        } else {
            self.dot.partial_cmp(&other.dot)
        }
    }
}
