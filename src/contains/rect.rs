use crate::prelude::*;

impl ContainsShape for Rect {
    fn contains_circle(&self, circle: &Circle) -> bool {
        let dist = self.center().distance(circle.center()) - circle.radius() / 2;
        let max = self.width().max(self.height()) / 2;
        dist < max
    }

    fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
        if self.contains(ellipse.center()) {
            self.intersects_ellipse(ellipse)
        } else {
            false
        }
    }
}
