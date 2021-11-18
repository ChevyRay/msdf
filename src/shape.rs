use crate::{dot_product, mix, sqrt, Contour, EdgeHolder, Intersection, Scanline, Segment};

const DECONVERGENCE_FACTOR: f64 = 0.000001;
const CORNER_DOT_EPSILON: f64 = 0.000001;

pub struct Bounds {
    pub l: f64,
    pub b: f64,
    pub r: f64,
    pub t: f64,
}

impl Bounds {
    #[inline]
    pub const fn new(l: f64, b: f64, r: f64, t: f64) -> Self {
        Self { l, b, r, t }
    }
}

#[derive(Default)]
pub struct Shape {
    contours: Vec<Contour>,
    inverse_y_axis: bool,
}

impl Shape {
    #[inline]
    pub fn add_contour(&mut self, contour: Contour) {
        self.contours.push(contour);
    }

    #[inline]
    pub fn add_empty_contour(&mut self) -> &mut Contour {
        self.add_contour(Contour::default());
        self.contours.last_mut().unwrap()
    }

    pub fn validate(&self) -> bool {
        for contour in &self.contours {
            if !contour.edges.is_empty() {
                let mut corner = contour.edges.last().unwrap().point(1.0);
                for edge in &contour.edges {
                    if edge.segment().is_none() {
                        return false;
                    }
                    if edge.point(0.0) != corner {
                        return false;
                    }
                    corner = edge.point(1.0);
                }
            }
        }
        return true;
    }

    pub fn normalize(&mut self) {
        for contour in &mut self.contours {
            if contour.edges.len() == 1 {
                let (part0, part1, part2) = contour.edges[0].split_in_thirds();
                contour.clear();
                contour.add_edge(EdgeHolder::new(part0));
                contour.add_edge(EdgeHolder::new(part1));
                contour.add_edge(EdgeHolder::new(part2));
            } else if !contour.edges.is_empty() {
                let mut prev_i = contour.edges.len() - 1;
                for curr_i in 0..contour.edges.len() {
                    let prev_dir = contour.edges[prev_i].direction(1.0).normalize();
                    let curr_dir = contour.edges[curr_i].direction(0.0).normalize();
                    if dot_product(prev_dir, curr_dir) < CORNER_DOT_EPSILON - 1.0 {
                        deconverge_edge(&mut contour.edges[prev_i], 1);
                        deconverge_edge(&mut contour.edges[curr_i], 0);
                    }
                    prev_i = curr_i;
                }
            }
        }
    }

    #[inline]
    pub fn bound(&self, l: &mut f64, b: &mut f64, r: &mut f64, t: &mut f64) {
        for contour in &self.contours {
            contour.bound(l, b, r, t);
        }
    }

    #[inline]
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
        for contour in &self.contours {
            contour.bound_miters(l, b, r, t, border, miter_limit, polarity);
        }
    }

    pub fn get_bounds(&self, border: f64, miter_limit: f64, polarity: f64) -> Bounds {
        const LARGE_VALUE: f64 = 1e240;
        let mut bounds = Bounds::new(LARGE_VALUE, LARGE_VALUE, -LARGE_VALUE, -LARGE_VALUE);
        self.bound(&mut bounds.l, &mut bounds.b, &mut bounds.r, &mut bounds.t);
        if border > 0.0 {
            bounds.l -= border;
            bounds.b -= border;
            bounds.r += border;
            bounds.t += border;
            if miter_limit > 0.0 {
                self.bound_miters(
                    &mut bounds.l,
                    &mut bounds.b,
                    &mut bounds.r,
                    &mut bounds.t,
                    border,
                    miter_limit,
                    polarity,
                );
            }
        }
        bounds
    }

    pub fn scanline(&self, line: &mut Scanline, y: f64) {
        line.clear();
        let mut x = [0.0; 3];
        let mut dy = [0; 3];
        for contour in &self.contours {
            for edge in &contour.edges {
                let n = edge.scanline_intersections(&mut x, &mut dy, y);
                for i in 0..n {
                    line.add_intersection(Intersection::new(x[i], dy[i]));
                }
            }
        }
        line.preprocess();
    }

    #[inline]
    pub fn edge_count(&self) -> usize {
        self.contours.iter().map(|c| c.edges.len()).sum()
    }

    pub fn orient_contours(&mut self) {
        let ratio = 0.5 * (sqrt(5.0) - 1.0);

        let mut orientations = Vec::new();
        orientations.resize(self.contours.len(), 0i32);

        let mut intersections = Vec::new();

        for i in 0..self.contours.len() {
            if orientations[i] == 0 && !self.contours[i].empty() {
                let y0 = self.contours[i].edges.first().unwrap().point(0.0).y;
                let mut y1 = y0;

                for edge in &self.contours[i].edges {
                    if y0 != y1 {
                        break;
                    }
                    y1 = edge.point(1.0).y;
                }
                for edge in &self.contours[i].edges {
                    if y0 != y1 {
                        break;
                    }
                    y1 = edge.point(ratio).y;
                }

                let y = mix(y0, y1, ratio);
                let mut x = [0.0; 3];
                let mut dy = [0; 3];

                for j in 0..self.contours.len() {
                    for edge in &self.contours[j].edges {
                        let n = edge.scanline_intersections(&mut x, &mut dy, y);
                        for k in 0..n {
                            intersections.push(OrientIntersection {
                                x: x[k],
                                direction: dy[k],
                                contour_index: j,
                            })
                        }
                    }
                }

                intersections.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

                for j in 1..intersections.len() {
                    if intersections[j].x == intersections[j - 1].x {
                        intersections[j].direction = 0;
                        intersections[j - 1].direction = 0;
                    }
                }

                for j in 0..intersections.len() {
                    if intersections[j].direction != 0 {
                        let jj = j as i32;
                        if intersections[j].direction > 0 {
                            orientations[intersections[j].contour_index] += 2 * ((jj & 1) ^ 1) - 1;
                        } else {
                            orientations[intersections[j].contour_index] += 2 * ((jj & 1) ^ 0) - 1;
                        }
                    }
                }

                intersections.clear();
            }
        }
        // Reverse contours that have the opposite orientation
        for i in 0..self.contours.len() {
            if orientations[i] < 0 {
                self.contours[i].reverse();
            }
        }
    }
}

struct OrientIntersection {
    x: f64,
    direction: i32,
    contour_index: usize,
}

#[inline]
fn deconverge_edge(edge: &mut EdgeHolder, param: i32) {
    let color = edge.color;
    let new_segment = if let Some(seg) = edge.segment_mut() {
        match &mut seg.segment {
            Segment::Linear(_) => return,
            Segment::Quadratic(seg) => seg.convert_to_cubic(color),
            Segment::Cubic(seg) => {
                seg.deconverge(param, DECONVERGENCE_FACTOR);
                return;
            }
        }
    } else {
        return;
    };
    edge.set_segment(new_segment);
}
