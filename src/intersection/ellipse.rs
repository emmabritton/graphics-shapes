use crate::intersection::shared::{line_ellipse, polygon_ellipse, rect_ellipse, triangle_ellipse};
use crate::prelude::*;

impl IntersectsShape for Ellipse {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        rect_ellipse(rect, self)
    }

    fn intersects_circle(&self, _circle: &Circle) -> bool {
        todo!()
    }

    fn intersects_line(&self, line: &Line) -> bool {
        line_ellipse(line, self)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        triangle_ellipse(triangle, self)
    }

    fn intersects_ellipse(&self, _ellipse: &Ellipse) -> bool {
        todo!()
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        polygon_ellipse(polygon, self)
    }
}
