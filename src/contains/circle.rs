use crate::prelude::*;

impl ContainsShape for Circle {
    fn contains_circle(&self, circle: &Circle) -> bool {
        let dist = self.center().distance(circle.center()) as isize;
        let max = (self.radius() as isize) - (circle.radius() as isize);
        dist < max.abs()
    }

    fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
        self.contains_polygon(&ellipse.as_polygon())
    }
}
