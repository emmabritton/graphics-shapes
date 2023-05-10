use crate::intersection::shared::{line_triangle, lines_lines, triangle_circle, triangle_ellipse};
use crate::prelude::*;

impl IntersectsShape for Triangle {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        lines_lines(&rect.as_lines(), &self.as_lines())
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        triangle_circle(self, circle)
    }

    fn intersects_line(&self, line: &Line) -> bool {
        line_triangle(line, self)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        lines_lines(&triangle.as_lines(), &self.as_lines())
    }

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        triangle_ellipse(self, ellipse)
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        lines_lines(&polygon.as_lines(), &self.as_lines())
    }
}
