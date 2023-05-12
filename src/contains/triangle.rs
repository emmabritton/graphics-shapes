use crate::prelude::*;

impl ContainsShape for Triangle {
    fn contains_circle(&self, circle: &Circle) -> bool {
        if self.contains(circle.center()) {
            self.intersects_circle(circle)
        } else {
            false
        }
    }

    // fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
    //     if self.contains(ellipse.center()) {
    //         self.intersects_ellipse(ellipse)
    //     } else {
    //         false
    //     }
    // }
}
