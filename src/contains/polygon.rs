use crate::contains::contains_points;
use crate::prelude::*;

impl ContainsShape for Polygon {
    fn contains_rect(&self, rect: &Rect) -> bool {
        contains_points(self, rect) && !self.intersects_rect(rect)
    }

    fn contains_circle(&self, circle: &Circle) -> bool {
        contains_points(self, circle) && !self.intersects_circle(circle)
    }

    fn contains_line(&self, line: &Line) -> bool {
        contains_points(self, line) && !self.intersects_line(line)
    }

    fn contains_triangle(&self, triangle: &Triangle) -> bool {
        contains_points(self, triangle) && !self.intersects_triangle(triangle)
    }

    fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
        contains_points(self, ellipse) && !self.intersects_ellipse(ellipse)
    }

    fn contains_polygon(&self, polygon: &Polygon) -> bool {
        contains_points(self, polygon) && !self.intersects_polygon(polygon)
    }
}
