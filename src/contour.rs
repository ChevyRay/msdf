use crate::{cross_product, dot_product, min, point_bounds, sign, sqrt, EdgeHolder, Vector2};

#[derive(Default)]
pub struct Contour {
    pub(crate) edges: Vec<EdgeHolder>,
}

impl Contour {
    pub fn empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }

    pub fn add_edge(&mut self, edge: EdgeHolder) {
        self.edges.push(edge);
    }

    pub fn add_empty_edge(&mut self) -> &mut EdgeHolder {
        self.add_edge(EdgeHolder::default());
        self.edges.last_mut().unwrap()
    }

    pub fn bound(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        for edge in &self.edges {
            edge.bound(l, b, r, t);
        }
    }

    pub fn bound_miters(
        &self,
        l: &mut f64,
        b: &mut f64,
        r: &mut f64,
        t: &mut f64,
        border: f64,
        miter_limit: f64,
        polarity: f64,
    ) {
        if self.edges.is_empty() {
            return;
        }
        let mut prev_dir = self
            .edges
            .last()
            .unwrap()
            .direction(1.0)
            .normalize_allow_zero();
        for edge in &self.edges {
            let dir = -edge.direction(0.0).normalize_allow_zero();
            if polarity * cross_product(prev_dir, dir) >= 0.0 {
                let mut miter_length = miter_limit;
                let q = 0.5 * (1.0 - dot_product(prev_dir, dir));
                if q > 0.0 {
                    miter_length = min(1.0 / sqrt(q), miter_limit);
                }
                let miter = edge.point(0.0)
                    + (prev_dir + dir).normalize_allow_zero() * border * miter_length;
                point_bounds(&miter, l, b, r, t);
            }
            prev_dir = edge.direction(1.0).normalize_allow_zero();
        }
    }

    pub fn winding(&self) -> i32 {
        if self.edges.is_empty() {
            return 0;
        }
        let mut total = 0.0;
        if self.edges.len() == 1 {
            let a = self.edges[0].point(0.0);
            let b = self.edges[0].point(1.0 / 3.0);
            let c = self.edges[0].point(2.0 / 3.0);
            total += shoelace(&a, &b);
            total += shoelace(&b, &c);
            total += shoelace(&c, &a);
        } else if self.edges.len() == 2 {
            let a = self.edges[0].point(0.0);
            let b = self.edges[0].point(0.5);
            let c = self.edges[1].point(0.0);
            let d = self.edges[1].point(0.5);
            total += shoelace(&a, &b);
            total += shoelace(&b, &c);
            total += shoelace(&c, &d);
            total += shoelace(&d, &a);
        } else {
            let mut prev = self.edges.last().unwrap().point(0.0);
            for edge in &self.edges {
                let cur = edge.point(0.0);
                total += shoelace(&prev, &cur);
                prev = cur;
            }
        }
        return sign(total);
    }

    pub fn reverse(&mut self) {
        let len = self.edges.len();
        let mut i = len / 2;
        while i > 0 {
            self.edges.swap(i - 1, len - i);
            i -= 1;
        }
        for edge in &mut self.edges {
            edge.reverse();
        }
    }
}

fn shoelace(a: &Vector2, b: &Vector2) -> f64 {
    (b.x - a.x) * (a.y + b.y)
}
