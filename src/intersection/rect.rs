use crate::circle::Circle;
use crate::ellipse::Ellipse;
use crate::intersection::shared::{line_rect, lines_lines, rect_circle, rect_ellipse};
use crate::intersection::IntersectsShape;
use crate::line::Line;
use crate::polygon::Polygon;
use crate::prelude::{Rect, Triangle};
use crate::Shape;

impl IntersectsShape for Rect {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        comp_rect_rect(self, rect)
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

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        rect_ellipse(self, ellipse)
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        lines_lines(&self.as_lines(), &polygon.as_lines())
    }
}

/// Returns four boolean array of
/// [
///     lhs left is left or right of both rhs left and right ,
///     lhs top is above or below of both rhs top and bottom,
///     lhs right is left or right of both rhs left and right,
///     lhs bottom is above or below of both rhs top and bottom,
/// ]
fn diff_rect_rect(lhs: &Rect, rhs: &Rect) -> [bool; 4] {
    let lhs_left = lhs.left();
    let lhs_right = lhs.right();
    let rhs_left = rhs.left();
    let rhs_right = rhs.right();
    let lhs_top = lhs.top();
    let lhs_bottom = lhs.bottom();
    let rhs_top = rhs.top();
    let rhs_bottom = rhs.bottom();
    [
        (lhs_left <= rhs_left && lhs_left <= rhs_right)
            || (lhs_left >= rhs_left && lhs_left >= rhs_right),
        (lhs_top <= rhs_top && lhs_top <= rhs_bottom)
            || (lhs_top >= rhs_top && lhs_top >= rhs_bottom),
        (lhs_right <= rhs_left && lhs_right <= rhs_right)
            || (lhs_right >= rhs_left && lhs_right >= rhs_right),
        (lhs_bottom <= rhs_bottom && lhs_bottom <= rhs_top)
            || (lhs_bottom >= rhs_bottom && lhs_bottom >= rhs_top),
    ]
}

/// Checks the diff_rect_rect(lhs, rhs) and diff_rect_rect(rhs, lhs) are opposite as this
/// means the rects overlap
///
/// If any are the same then they do not overlap unless they overlap perfectly
fn comp_rect_rect(lhs: &Rect, rhs: &Rect) -> bool {
    let lhs_results = diff_rect_rect(lhs, rhs);
    let rhs_results = diff_rect_rect(rhs, lhs);
    (lhs_results[0] != rhs_results[0]
        && lhs_results[1] != rhs_results[1]
        && lhs_results[2] != rhs_results[2]
        && lhs_results[3] != rhs_results[3])
        || (lhs.top_left() == rhs.top_left() && lhs.bottom_right() == rhs.bottom_right())
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
            assert!(left.intersects_rect(&right));
            assert!(right.intersects_rect(&left));
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
    }
}
