use crate::intersection::shared::{line_polygon, lines_lines, polygon_circle};
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

    // fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
    //     polygon_ellipse(self, ellipse)
    // }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        lines_lines(&self.as_lines(), &polygon.as_lines())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn circle_on_line() {
        let polygon = Polygon::new(&[(50, 48), (206, 48), (206, 204), (128, 126), (50, 204)]);
        let circle = Circle::new((113, 135), 15);
        assert!(polygon.intersects_circle(&circle));
        assert!(circle.intersects_polygon(&polygon));
    }

    #[test]
    fn circle_below() {
        let polygon = Polygon::new(&[(44, 36), (222, 36), (222, 214), (133, 125), (44, 214)]);
        let circle = Circle::new((115, 180), 15);
        assert!(!polygon.intersects_circle(&circle));
        assert!(!circle.intersects_polygon(&polygon));
    }

    #[test]
    fn circle_on_top_line() {
        let polygon = Polygon::new(&[(44, 36), (222, 36), (222, 214), (133, 125), (44, 214)]);
        let circle = Circle::new((72, 38), 15);
        assert!(polygon.intersects_circle(&circle));
        assert!(circle.intersects_polygon(&polygon));
    }

    #[test]
    fn circle_inside() {
        let polygon = Polygon::new(&[(44, 36), (222, 36), (222, 214), (133, 125), (44, 214)]);
        let circle = Circle::new((128, 99), 15);
        assert!(!polygon.intersects_circle(&circle));
        assert!(!circle.intersects_polygon(&polygon));
    }
}
