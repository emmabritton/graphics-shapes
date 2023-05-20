use crate::prelude::*;

impl ContainsShape for Ellipse {
    fn contains_circle(&self, circle: &Circle) -> bool {
        self.as_polygon().contains_circle(circle)
    }

    fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
        self.as_polygon().contains_polygon(&ellipse.as_polygon())
    }
}
