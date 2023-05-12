use crate::intersection::shared::*;
use crate::prelude::*;

impl IntersectsShape for Rect {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        lines_lines(&self.as_lines(), &rect.as_lines())
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        rect_circle(self, circle)
    }

    fn intersects_line(&self, line: &Line) -> bool {
        line_rect(line, self)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        lines_lines(&self.as_lines(), &triangle.as_lines())
    }

    // fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
    //     rect_ellipse(self, ellipse)
    // }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        lines_lines(&self.as_lines(), &polygon.as_lines())
    }
}

#[cfg(test)]
mod test {
    mod rect_rect {
        use crate::intersection::IntersectsShape;
        use crate::rect::Rect;

        #[test]
        fn non_intersecting() {
            let left = Rect::new((10, 10), (20, 20));
            let right = Rect::new((30, 10), (40, 20));
            assert!(!left.intersects_rect(&right));
            assert!(!right.intersects_rect(&left));
        }

        #[test]
        fn fully_overlapping() {
            let left = Rect::new((10, 10), (20, 20));
            let right = Rect::new((10, 10), (20, 20));
            assert!(left.intersects_rect(&right));
            assert!(right.intersects_rect(&left));
        }

        #[test]
        fn inside() {
            let left = Rect::new((10, 10), (40, 40));
            let right = Rect::new((20, 20), (30, 30));
            assert!(!left.intersects_rect(&right));
            assert!(!right.intersects_rect(&left));
        }

        #[test]
        fn wide_and_tall() {
            let left = Rect::new((30, 10), (40, 60));
            let right = Rect::new((0, 30), (60, 40));
            assert!(left.intersects_rect(&right));
            assert!(right.intersects_rect(&left));
        }

        #[test]
        fn wide_and_tall_separated() {
            let left = Rect::new((30, 10), (40, 60));
            let right = Rect::new((100, 30), (160, 40));
            assert!(!left.intersects_rect(&right));
            assert!(!right.intersects_rect(&left));
        }

        #[test]
        fn partly_inside() {
            let left = Rect::new((10, 10), (70, 70));
            let right = Rect::new((30, 30), (100, 50));
            assert!(left.intersects_rect(&right));
            assert!(right.intersects_rect(&left));
        }

        #[test]
        fn partly_inside_separated() {
            let left = Rect::new((10, 10), (70, 70));
            let right = Rect::new((130, 30), (200, 50));
            assert!(!left.intersects_rect(&right));
            assert!(!right.intersects_rect(&left));
        }

        #[test]
        fn off_by_one() {
            let left = Rect::new((10, 10), (20, 20));
            let right = Rect::new((11, 10), (21, 20));
            assert!(left.intersects_rect(&right));
            assert!(right.intersects_rect(&left));
        }
    }
}
