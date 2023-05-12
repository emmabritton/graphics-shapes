use crate::intersection::shared::{
    line_circle, line_polygon, line_rect, line_triangle,
};
use crate::intersection::IntersectsShape;
use crate::prelude::*;
use std::cmp::Ordering;

impl IntersectsShape for Line {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        line_rect(self, rect)
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        line_circle(self, circle)
    }

    fn intersects_line(&self, line: &Line) -> bool {
        comp_line_line(self, line)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        line_triangle(self, triangle)
    }

    // fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
    //     line_ellipse(self, ellipse)
    // }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        line_polygon(self, polygon)
    }
}

fn direction(p: Coord, q: Coord, r: Coord) -> isize {
    let value = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    match value.cmp(&0) {
        Ordering::Less => 2,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn comp_line_line(lhs: &Line, rhs: &Line) -> bool {
    if lhs.contains_line(rhs) {
        return true;
    }

    let ls = lhs.start();
    let le = lhs.end();
    let rs = rhs.start();
    let re = rhs.end();

    let d1 = direction(ls, le, rs);
    let d2 = direction(ls, le, re);
    let d3 = direction(rs, re, ls);
    let d4 = direction(rs, re, le);

    (d1 != d2 && d3 != d4)
        || (d1 == 0 && rs.is_between(ls, le))
        || (d2 == 0 && re.is_between(ls, le))
        || (d3 == 0 && ls.is_between(rs, re))
        || (d4 == 0 && le.is_between(rs, re))
}

#[cfg(test)]
mod test {
    mod line_circle {
        use crate::circle::Circle;
        use crate::intersection::IntersectsShape;
        use crate::line::Line;

        #[test]
        fn line_above() {
            let line = Line::new((10, 10), (40, 10));
            let circle = Circle::new((20, 30), 8);
            assert!(!line.intersects_circle(&circle));
        }

        #[test]
        fn line_through_center_horz() {
            let line = Line::new((50, 133), (194, 133));
            let circle = Circle::new((123, 135), 15);
            assert!(line.intersects_circle(&circle));
            assert!(circle.intersects_line(&line));

            let line = Line::new((57, 134), (201, 134));
            let circle = Circle::new((123, 135), 15);
            assert!(line.intersects_circle(&circle));
            assert!(circle.intersects_line(&line));

            let line = Line::new((194, 133), (50, 133));
            let circle = Circle::new((123, 135), 15);
            assert!(line.intersects_circle(&circle));
            assert!(circle.intersects_line(&line));

            let line = Line::new((201, 134), (57, 134));
            let circle = Circle::new((123, 135), 15);
            assert!(line.intersects_circle(&circle));
            assert!(circle.intersects_line(&line));
        }

        #[test]
        fn line_left() {
            let line = Line::new((90, 115), (110, 135));
            let circle = Circle::new((129, 136), 15);
            assert!(!line.intersects_circle(&circle));
        }

        #[test]
        fn collinear_ish() {
            let line = Line::new((76, 73), (96, 93));
            let circle = Circle::new((119, 114), 15);
            assert!(!line.intersects_circle(&circle));
        }
    }

    mod line_line {
        use crate::intersection::IntersectsShape;
        use crate::prelude::Line;

        #[test]
        fn two_parallel_lines_vert() {
            let line1 = Line::new((10, 10), (10, 20));
            let line2 = Line::new((30, 10), (30, 20));
            assert!(!line1.intersects_line(&line2));
            assert!(!line2.intersects_line(&line1));
        }

        #[test]
        fn two_parallel_lines_horz() {
            let line1 = Line::new((10, 10), (30, 10));
            let line2 = Line::new((10, 30), (30, 30));
            assert!(!line1.intersects_line(&line2));
            assert!(!line2.intersects_line(&line1));
        }

        #[test]
        fn two_lines_plus() {
            let horz = Line::new((0, 15), (30, 15));
            let vert = Line::new((15, 0), (15, 30));
            assert!(vert.intersects_line(&horz));
            assert!(horz.intersects_line(&vert));
        }

        #[test]
        fn two_lines_cross() {
            let horz = Line::new((0, 0), (30, 30));
            let vert = Line::new((0, 30), (30, 0));
            assert!(vert.intersects_line(&horz));
            assert!(horz.intersects_line(&vert));
        }

        #[test]
        fn two_lines_tip() {
            let line1 = Line::new((0, 0), (30, 0));
            let line2 = Line::new((30, 0), (30, 30));
            assert!(line1.intersects_line(&line2));
            assert!(line2.intersects_line(&line1));
        }

        #[test]
        fn two_lines_overlap() {
            let line1 = Line::new((0, 0), (30, 0));
            let line2 = Line::new((0, 0), (30, 0));
            assert!(line1.intersects_line(&line2));
            assert!(line2.intersects_line(&line1));
        }

        #[test]
        fn two_angled_inline_separate() {
            let line1 = Line::new((109, 104), (129, 124));
            let line2 = Line::new((134, 129), (159, 149));
            assert!(!line1.intersects_line(&line2));
            assert!(!line2.intersects_line(&line1));
        }

        #[test]
        fn two_angled_inline_touching() {
            let line1 = Line::new((82, 72), (102, 92));
            let line2 = Line::new((96, 86), (116, 106));
            assert!(line1.intersects_line(&line2));
            assert!(line2.intersects_line(&line1));
            let line1 = Line::new((102, 92), (82, 72));
            let line2 = Line::new((116, 106), (96, 86));
            assert!(line1.intersects_line(&line2));
            assert!(line2.intersects_line(&line1));
        }
    }
}
