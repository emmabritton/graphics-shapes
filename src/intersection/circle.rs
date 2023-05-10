use crate::intersection::shared::{line_circle, polygon_circle, rect_circle, triangle_circle};
use crate::prelude::*;

impl IntersectsShape for Circle {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        rect_circle(rect, self)
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        let max = circle.radius().max(self.radius());
        let dist = circle.center().distance(self.center());
        dist <= max
    }

    fn intersects_line(&self, line: &Line) -> bool {
        line_circle(line, self)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        triangle_circle(triangle, self)
    }

    fn intersects_ellipse(&self, _ellipse: &Ellipse) -> bool {
        todo!()
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        polygon_circle(polygon, self)
    }
}
