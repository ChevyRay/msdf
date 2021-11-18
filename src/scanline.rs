use crate::min;

#[derive(Copy, Clone)]
pub enum FillRule {
    NonZero,
    Odd,
    Positive,
    Negative,
}

pub fn interpret_fill_rule(intersections: i32, fill_rule: FillRule) -> bool {
    match fill_rule {
        FillRule::NonZero => intersections != 0,
        FillRule::Odd => (intersections & 1) != 0,
        FillRule::Positive => intersections > 0,
        FillRule::Negative => intersections < 0,
    }
}

pub struct Intersection {
    pub x: f64,
    pub direction: i32,
}

impl Intersection {
    pub fn new(x: f64, direction: i32) -> Self {
        Self { x, direction }
    }
}

pub struct Scanline {
    intersections: Vec<Intersection>,
    last_index: usize,
}

impl Scanline {
    pub fn clear(&mut self) {
        self.intersections.clear();
    }

    pub fn add_intersection(&mut self, intersection: Intersection) {
        self.intersections.push(intersection);
    }

    pub fn overlap(a: &Scanline, b: &Scanline, x_from: f64, x_to: f64, fill_rule: FillRule) -> f64 {
        let mut total = 0.0;
        let mut a_inside = false;
        let mut b_inside = false;
        let mut ai = 0;
        let mut bi = 0;
        let mut ax = if a.intersections.is_empty() {
            x_to
        } else {
            a.intersections[ai].x
        };
        let mut bx = if b.intersections.is_empty() {
            x_to
        } else {
            b.intersections[bi].x
        };
        while ax < x_from || bx < x_from {
            let x_next = min(ax, bx);
            if ax == x_next && ai < a.intersections.len() {
                a_inside = interpret_fill_rule(a.intersections[ai].direction, fill_rule);
                ai += 1;
                ax = if ai < a.intersections.len() {
                    a.intersections[ai].x
                } else {
                    x_to
                };
            }
            if bx == x_next && bi < b.intersections.len() {
                b_inside = interpret_fill_rule(b.intersections[bi].direction, fill_rule);
                bi += 1;
                bx = if bi < b.intersections.len() {
                    b.intersections[bi].x
                } else {
                    x_to
                };
            }
        }
        let mut x = x_from;
        while ax < x_to || bx < x_to {
            let x_next = min(ax, bx);
            if a_inside == b_inside {
                total += x_next - x;
            }
            if ax == x_next && ai < a.intersections.len() {
                a_inside = interpret_fill_rule(a.intersections[ai].direction, fill_rule);
                ai += 1;
                ax = if ai < a.intersections.len() {
                    a.intersections[ai].x
                } else {
                    x_to
                };
            }
            if bx == x_next && bi < b.intersections.len() {
                b_inside = interpret_fill_rule(b.intersections[bi].direction, fill_rule);
                bi += 1;
                bx = if bi < b.intersections.len() {
                    b.intersections[bi].x
                } else {
                    x_to
                };
            }
            x = x_next;
        }
        if a_inside == b_inside {
            total += x_to - x;
        }
        total
    }

    pub fn preprocess(&mut self) {
        self.last_index = 0;
        if !self.intersections.is_empty() {
            self.intersections
                .sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
            let mut total_direction = 0;
            for intersection in &mut self.intersections {
                total_direction += intersection.direction;
                intersection.direction = total_direction;
            }
        }
    }

    pub fn move_to(&mut self, x: f64) -> Option<usize> {
        if self.intersections.is_empty() {
            return None;
        }
        let mut index = self.last_index;
        if x < self.intersections[index].x {
            loop {
                if index == 0 {
                    self.last_index = 0;
                    return None;
                }
                index -= 1;
                if x < self.intersections[index].x {
                    break;
                }
            }
        } else {
            while index < self.intersections.len() - 1 && x >= self.intersections[index + 1].x {
                index += 1;
            }
        }
        self.last_index = index;
        Some(index)
    }

    pub fn count_intersections(&mut self, x: f64) -> usize {
        if let Some(n) = self.move_to(x) {
            n + 1
        } else {
            0
        }
    }

    pub fn sum_intersections(&mut self, x: f64) -> i32 {
        if let Some(index) = self.move_to(x) {
            return self.intersections[index].direction;
        } else {
            0
        }
    }

    pub fn filled(&mut self, x: f64, fill_rule: FillRule) -> bool {
        interpret_fill_rule(self.sum_intersections(x), fill_rule)
    }
}
