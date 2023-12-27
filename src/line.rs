use crate::prelude::*;
use crate::shape_box::ShapeBox;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::mem::swap;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum LineType {
    /// Single pixel
    Point,
    Horizontal,
    Vertical,
    /// Anything other vertical or horizontal, see [angle][Line::angle]
    Angled,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line {
    start: Coord,
    end: Coord,
    len: usize,
    line_type: LineType,
    angle: isize,
}

impl IntersectsContains for Line {}

impl Line {
    #[must_use]
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(start: P1, end: P2) -> Self {
        let start = start.into();
        let end = end.into();
        let line_type = if start == end {
            LineType::Point
        } else if start.x == end.x {
            LineType::Vertical
        } else if start.y == end.y {
            LineType::Horizontal
        } else {
            LineType::Angled
        };
        let len = start.distance(end);
        let angle = start.angle_to(end);
        Self {
            start,
            end,
            len,
            line_type,
            angle,
        }
    }
}

impl Line {
    #[allow(clippy::len_without_is_empty)] //use start()==end() to check that
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Angle from `start` point to `end` point
    #[inline]
    #[must_use]
    pub fn angle(&self) -> isize {
        self.angle
    }

    #[inline]
    #[must_use]
    pub fn start(&self) -> Coord {
        self.start
    }

    #[inline]
    #[must_use]
    pub fn end(&self) -> Coord {
        self.end
    }

    #[inline]
    #[must_use]
    pub fn line_type(&self) -> LineType {
        self.line_type
    }

    pub fn nearest_point<P: Into<Coord>>(&self, other: P) -> Coord {
        if self.line_type == LineType::Point {
            return self.start;
        }
        let other = other.into();
        let ba_x = (self.end.x - self.start.x) as f64;
        let ba_y = (self.end.y - self.start.y) as f64;

        let len = ba_x.powi(2) + ba_y.powi(2);
        let t =
            ((other.x - self.start.x) as f64 * ba_x + (other.y - self.start.y) as f64 * ba_y) / len;
        let t = t.clamp(0.0, 1.0);
        coord!(
            self.start.x as f64 + t * ba_x,
            self.start.y as f64 + t * ba_y
        )
    }
}

impl Shape for Line {
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        Line::new(points[0], points[1])
    }

    fn rebuild(&self, points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Line::from_points(points)
    }

    fn contains(&self, point: Coord) -> bool {
        match self.line_type {
            LineType::Point => self.start == point,
            LineType::Horizontal => {
                self.start.y == point.y && (self.left()..=self.right()).contains(&point.x)
            }
            LineType::Vertical => {
                self.start.x == point.x && (self.top()..=self.bottom()).contains(&point.y)
            }
            LineType::Angled => {
                self.start.are_collinear(self.end, point) && point.is_between(self.start, self.end)
            }
        }
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.start, self.end]
    }

    fn center(&self) -> Coord {
        self.start.mid_point(self.end)
    }

    #[inline]
    fn left(&self) -> isize {
        self.start.x.min(self.end.x)
    }

    #[inline]
    fn right(&self) -> isize {
        self.start.x.max(self.end.x)
    }

    #[inline]
    fn top(&self) -> isize {
        self.start.y.min(self.end.y)
    }

    #[inline]
    fn bottom(&self) -> isize {
        self.start.y.max(self.end.y)
    }

    /// Returns left most point x and lowest y  (could be from `start` or `end` for both)
    /// This doesn't really make sense for line as it may return (end.x, start.y) for example
    fn top_left(&self) -> Coord {
        coord!(self.left(), self.top())
    }

    /// Returns right most point x and lowest y  (could be from `start` or `end` for both)
    /// This doesn't really make sense for line as it may return (start.x, end.y) for example
    fn top_right(&self) -> Coord {
        coord!(self.right(), self.top())
    }

    /// Returns left most point x and highest y  (could be from `start` or `end` for both)
    /// This doesn't really make sense for line as it may return (start.x, end.y) for example
    fn bottom_left(&self) -> Coord {
        coord!(self.left(), self.bottom())
    }

    /// Returns right most point x and highest y  (could be from `start` or `end` for both)
    /// This doesn't really make sense for line as it may return (start.x, end.y) for example
    fn bottom_right(&self) -> Coord {
        coord!(self.right(), self.bottom())
    }

    fn outline_pixels(&self) -> Vec<Coord> {
        let mut start = self.start;
        let mut end = self.end;
        if start.x > end.x || start.y > end.y {
            swap(&mut start, &mut end);
        }
        let mut output = vec![];
        if start.x == end.x {
            for y in start.y..=end.y {
                output.push(coord!(start.x, y));
            }
        } else if start.y == end.y {
            for x in start.x..=end.x {
                output.push(coord!(x, start.y));
            }
        } else {
            let mut delta = 0;
            let x1 = start.x;
            let y1 = start.y;
            let x2 = end.x;
            let y2 = end.y;
            let dx = isize::abs(x2 - x1);
            let dy = isize::abs(y2 - y1);
            let dx2 = dx * 2;
            let dy2 = dy * 2;
            let ix: isize = if x1 < x2 { 1 } else { -1 };
            let iy: isize = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            if dx >= dy {
                loop {
                    output.push(coord!(x, y));
                    if x == x2 {
                        break;
                    }
                    x += ix;
                    delta += dy2;
                    if delta > dx {
                        y += iy;
                        delta -= dx2;
                    }
                }
            } else {
                loop {
                    output.push(coord!(x, y));
                    if y == y2 {
                        break;
                    }
                    y += iy;
                    delta += dx2;
                    if delta > dy {
                        x += ix;
                        delta -= dy2;
                    }
                }
            }
        }
        output
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        self.outline_pixels()
    }

    fn to_shape_box(&self) -> ShapeBox {
        ShapeBox::Line(self.clone())
    }
}

impl Line {
    #[must_use]
    pub fn as_rect(&self) -> Rect {
        Rect::new(self.start, self.end)
    }

    #[must_use]
    pub fn as_circle(&self) -> Circle {
        Circle::new(self.start, self.start.distance(self.end))
    }
}

#[cfg(test)]
mod test {
    use crate::line::Line;
    use crate::Shape;

    #[test]
    fn len() {
        assert_eq!(Line::new((10, 10), (20, 10)).len, 10);
        assert_eq!(Line::new((10, 10), (10, 20)).len, 10);
        assert_eq!(Line::new((10, 10), (0, 10)).len, 10);
        assert_eq!(Line::new((10, 10), (10, 0)).len, 10);
        assert_eq!(Line::new((10, 10), (0, 0)).len, 14);
        assert_eq!(Line::new((10, 10), (20, 20)).len, 14);
        assert_eq!(Line::new((10, 10), (0, 20)).len, 14);
        assert_eq!(Line::new((10, 10), (20, 0)).len, 14);
    }

    #[test]
    fn rotate_center() {
        assert_eq!(
            Line::new((10, 10), (20, 10)).rotate(90),
            Line::new((15, 5), (15, 15))
        );
        assert_eq!(
            Line::new((10, 10), (20, 10)).rotate(25),
            Line::new((10, 8), (20, 12))
        );
    }

    #[test]
    fn nearest() {
        let line = Line::new((10, 10), (20, 20));
        let point = line.nearest_point((17, 12));
        assert!(line.contains(coord!(14, 14)));
        assert_eq!(point, coord!(14, 14));
    }

    #[test]
    fn nearest_line_reversed() {
        let line = Line::new((110, 100), (40, 30));
        let point = line.nearest_point((55, 85));
        assert!(line.contains(coord!(75, 65)));
        assert_eq!(point, coord!(75, 65));
    }

    mod contains {
        use crate::line::Line;
        use crate::Shape;

        #[test]
        fn point() {
            let line = Line::new((10, 10), (10, 10));
            assert!(line.contains(coord!(10, 10)));
            assert!(!line.contains(coord!(11, 10)));
        }

        #[test]
        fn vert() {
            let line = Line::new((10, 10), (10, 20));
            assert!(line.contains(coord!(10, 14)));
            assert!(!line.contains(coord!(10, 24)));
            assert!(!line.contains(coord!(11, 14)));
        }

        #[test]
        fn horz() {
            let line = Line::new((10, 10), (0, 10));
            assert!(line.contains(coord!(5, 10)));
            assert!(!line.contains(coord!(-1, 10)));
            assert!(!line.contains(coord!(5, 11)));
        }

        #[test]
        fn angle() {
            let line = Line::new((0, 0), (10, 10));
            assert!(line.contains(coord!(5, 5)));
            assert!(!line.contains(coord!(8, 7)));
            assert!(!line.contains(coord!(11, 11)));
        }
    }

    mod outline {
        use crate::line::Line;
        use crate::Shape;

        #[test]
        fn flat_horz_right() {
            let points = Line::new((0, 0), (6, 0)).outline_pixels();
            assert_eq!(
                points,
                coord_vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]
            );
        }

        #[test]
        fn flat_vert_down() {
            let points = Line::new((0, 0), (0, 6)).outline_pixels();
            assert_eq!(
                points,
                coord_vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6)]
            );
        }

        #[test]
        fn flat_horz_left() {
            let points = Line::new((0, 0), (-6, 0)).outline_pixels();
            assert_eq!(
                points,
                coord_vec![(-6, 0), (-5, 0), (-4, 0), (-3, 0), (-2, 0), (-1, 0), (0, 0)]
            );
        }

        #[test]
        fn flat_vert_up() {
            let points = Line::new((0, 0), (0, -6)).outline_pixels();
            assert_eq!(
                points,
                coord_vec![(0, -6), (0, -5), (0, -4), (0, -3), (0, -2), (0, -1), (0, 0)]
            );
        }
    }
}
