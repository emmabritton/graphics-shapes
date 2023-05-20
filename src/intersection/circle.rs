use crate::intersection::shared::{
    ellipse_circle, line_circle, polygon_circle, rect_circle, triangle_circle,
};
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

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        ellipse_circle(ellipse, self)
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        polygon_circle(polygon, self)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn lines_all_directions() {
        let line_tl = Line::new((0, 0), (20, 20));
        let line_br = Line::new((20, 20), (0, 0));
        let line_bl = Line::new((0, 20), (20, 0));
        let line_tr = Line::new((20, 0), (0, 20));
        let circle = Circle::new((10, 10), 4);

        assert!(circle.intersects_line(&line_tl));
        assert!(circle.intersects_line(&line_tr));
        assert!(circle.intersects_line(&line_bl));
        assert!(circle.intersects_line(&line_br));
    }

    #[test]
    fn poly_part() {
        let line = Line::new((128, 126), (50, 204));
        let circle = Circle::new((113, 135), 15);

        assert!(circle.intersects_line(&line));
    }
}
