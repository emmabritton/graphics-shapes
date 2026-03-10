use crate::prelude::*;

impl ContainsShape for Circle {
    fn contains_circle(&self, circle: &Circle) -> bool {
        let dist = self.center().distance(circle.center()) as isize;
        dist + circle.radius() as isize <= self.radius() as isize
    }

    fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
        self.contains_polygon(&ellipse.as_polygon())
    }
}
