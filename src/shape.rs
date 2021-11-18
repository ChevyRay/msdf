use crate::Contour;

pub struct Bounds {
    pub l: f64,
    pub b: f64,
    pub r: f64,
    pub t: f64,
}

pub struct Shape {
    contours: Vec<Contour>,
    inverse_y_axis: bool,
}

impl Shape {
    pub fn add_contour(&mut self, contour: Contour) {
        self.contours.push(contour);
    }

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
}
