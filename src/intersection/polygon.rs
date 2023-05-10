use crate::intersection::shared::{line_polygon, lines_lines, polygon_circle, polygon_ellipse};
use crate::prelude::*;

impl IntersectsShape for Polygon {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        lines_lines(&self.as_lines(), &rect.as_lines())
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        polygon_circle(self, circle)
    }

    fn intersects_line(&self, line: &Line) -> bool {
        line_polygon(line, self)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        lines_lines(&self.as_lines(), &triangle.as_lines())
    }

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        polygon_ellipse(self, ellipse)
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        lines_lines(&self.as_lines(), &polygon.as_lines())
    }
}
