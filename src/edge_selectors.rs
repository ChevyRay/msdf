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
    type DistanceType;

    fn distance(&self) -> Self::DistanceType;
}

pub struct TrueDistanceSelector {}
