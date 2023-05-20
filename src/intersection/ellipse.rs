use crate::intersection::shared::{
    ellipse_circle, line_ellipse, lines_lines, polygon_ellipse, rect_ellipse, triangle_ellipse,
};
use crate::prelude::*;

impl IntersectsShape for Ellipse {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        rect_ellipse(rect, self)
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        ellipse_circle(self, circle)
    }

    fn intersects_line(&self, line: &Line) -> bool {
        line_ellipse(line, self)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        triangle_ellipse(triangle, self)
    }

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        lines_lines(
            &self.as_polygon().as_lines(),
            &ellipse.as_polygon().as_lines(),
        )
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        polygon_ellipse(polygon, self)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn line_below_ellipse_at_zero() {
        let line = Line::new((6, 47), (26, 67));
        let ellipse = Ellipse::new((1, 2), 124, 78);
        assert!(!line.intersects_ellipse(&ellipse));
        assert!(!ellipse.intersects_line(&line));
    }

    #[test]
    fn line_intersecting_ellipse_at_zero() {
        let line = Line::new((15, 30), (35, 50));
        let ellipse = Ellipse::new((1, 2), 124, 78);
        assert!(line.intersects_ellipse(&ellipse));
        assert!(ellipse.intersects_line(&line));
    }

    #[test]
    fn line_inside_ellipse_at_zero() {
        let line = Line::new((7, 4), (27, 24));
        let ellipse = Ellipse::new((1, 2), 124, 78);
        assert!(!line.intersects_ellipse(&ellipse));
        assert!(!ellipse.intersects_line(&line));
    }

    #[test]
    fn line_inside_ellipse() {
        let line = Line::new((101, 119), (121, 139));
        let ellipse = Ellipse::new((111, 129), 82, 52);
        assert!(!line.intersects_ellipse(&ellipse));
        assert!(!ellipse.intersects_line(&line));
    }

    #[test]
    fn line_above_ellipse() {
        let line = Line::new((97, 57), (117, 77));
        let ellipse = Ellipse::new((111, 129), 82, 52);
        assert!(!line.intersects_ellipse(&ellipse));
        assert!(!ellipse.intersects_line(&line));
    }

    #[test]
    fn line_intersecting_ellipse() {
        let line = Line::new((105, 145), (125, 165));
        let ellipse = Ellipse::new((111, 129), 82, 52);
        assert!(line.intersects_ellipse(&ellipse));
        assert!(ellipse.intersects_line(&line));
    }

    #[test]
    fn line_inside_rotated_ellipse() {
        let line = Line::new((114, 90), (134, 110));
        let ellipse = Ellipse::new((147, 131), 120, 80).rotate(48);
        assert!(!line.intersects_ellipse(&ellipse));
        assert!(!ellipse.intersects_line(&line));
    }

    #[test]
    fn line_left_of_rotated_ellipse() {
        let line = Line::new((91, 145), (111, 165));
        let ellipse = Ellipse::new((147, 131), 120, 80).rotate(48);
        assert!(!line.intersects_ellipse(&ellipse));
        assert!(!ellipse.intersects_line(&line));
    }

    #[test]
    fn line_intersecting_rotated_ellipse() {
        let line = Line::new((167, 172), (187, 192));
        let ellipse = Ellipse::new((147, 131), 120, 80).rotate(48);
        assert!(line.intersects_ellipse(&ellipse));
        assert!(ellipse.intersects_line(&line));
    }
}
