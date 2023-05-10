use crate::prelude::*;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use std::ops::Div;

/// Rectangle
///
/// Must have flat edges, to rotate first convert to [Polygon] using [Rect::as_polygon()]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rect {
    top_left: Coord,
    bottom_right: Coord,
}

impl IntersectsContains for Rect {}

impl Rect {
    #[must_use]
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(top_left: P1, bottom_right: P2) -> Self {
        Self {
            top_left: top_left.into(),
            bottom_right: bottom_right.into(),
        }
    }

    #[must_use]
    pub fn new_with_size<P: Into<Coord>>(start: P, width: usize, height: usize) -> Self {
        let top_left = start.into();
        let bottom_right = Coord {
            x: top_left.x + width as isize,
            y: top_left.y + height as isize,
        };
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl Rect {
    #[must_use]
    pub fn width(&self) -> usize {
        (self.bottom_right.x - self.top_left.x).unsigned_abs()
    }

    #[must_use]
    pub fn height(&self) -> usize {
        (self.bottom_right.y - self.top_left.y).unsigned_abs()
    }

    #[inline]
    #[must_use]
    pub fn top_left(&self) -> Coord {
        self.top_left
    }

    #[inline]
    #[must_use]
    pub fn bottom_right(&self) -> Coord {
        self.bottom_right
    }

    #[inline]
    #[must_use]
    pub fn is_square(&self) -> bool {
        let diff = self.bottom_right - self.top_left;
        diff.x == diff.y
    }
}

impl Shape for Rect {
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Rect::new(points[0], points[1])
    }

    fn contains(&self, point: Coord) -> bool {
        let point = point;
        (self.left()..=self.right()).contains(&point.x)
            && (self.top()..=self.bottom()).contains(&point.y)
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.top_left, self.bottom_right]
    }

    fn rotate_around(&self, degrees: isize, point: Coord) -> Self
    where
        Self: Sized,
    {
        let degrees = (degrees as f32 / 90.0).round() as isize;
        let points = rotate_points(point, &self.points(), degrees * 90);
        Self::from_points(&points)
    }

    fn center(&self) -> Coord {
        self.top_left.mid_point(self.bottom_right)
    }

    fn left(&self) -> isize {
        self.top_left.x.min(self.bottom_right.x)
    }

    fn right(&self) -> isize {
        self.top_left.x.max(self.bottom_right.x)
    }

    fn top(&self) -> isize {
        self.top_left.y.min(self.bottom_right.y)
    }

    fn bottom(&self) -> isize {
        self.top_left.y.max(self.bottom_right.y)
    }

    fn outline_pixels(&self) -> Vec<Coord> {
        let mut output = new_hash_set();

        let left = self.left();
        let right = self.right();
        let top = self.top();
        let bottom = self.bottom();

        for x in left..=right {
            output.insert(coord!(x, top));
            output.insert(coord!(x, bottom));
        }
        for y in top..=bottom {
            output.insert(coord!(left, y));
            output.insert(coord!(right, y));
        }

        output.into_iter().collect()
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        let mut output = new_hash_set();

        let left = self.left();
        let right = self.right();
        let top = self.top();
        let bottom = self.bottom();

        for y in top..=bottom {
            for x in left..=right {
                output.insert(coord!(x, y));
            }
        }

        output.into_iter().collect()
    }
}

impl Rect {
    /// Create a circle around the center to the closest edge
    #[must_use]
    pub fn as_smallest_circle(&self) -> Circle {
        let radius = self.width().div(2).min(self.height().div(2));
        Circle::new(self.center(), radius)
    }

    /// Create a circle around the center to the farthest edge
    #[must_use]
    pub fn as_biggest_circle(&self) -> Circle {
        let radius = self.width().div(2).max(self.height().div(2));
        Circle::new(self.center(), radius)
    }

    /// Create two triangles
    #[must_use]
    pub fn as_triangles(&self) -> (Triangle, Triangle) {
        let top_right = coord!(self.right(), self.top());
        let bottom_left = coord!(self.left(), self.bottom());
        (
            Triangle::new(self.top_left(), top_right, bottom_left),
            Triangle::new(self.bottom_right(), top_right, bottom_left),
        )
    }

    /// Same shape but represented as four points/lines instead of two points
    #[must_use]
    pub fn as_polygon(&self) -> Polygon {
        let top_right = coord!(self.right(), self.top());
        let bottom_left = coord!(self.left(), self.bottom());
        Polygon::new(&[self.top_left, top_right, self.bottom_right, bottom_left])
    }

    #[must_use]
    pub fn as_ellipse(&self) -> Ellipse {
        Ellipse::from_points(&self.points())
    }

    #[must_use]
    pub fn as_lines(&self) -> [Line; 4] {
        [
            Line::new(self.top_left(), self.top_right()),
            Line::new(self.top_right(), self.bottom_right()),
            Line::new(self.bottom_right(), self.bottom_left()),
            Line::new(self.bottom_left(), self.top_left()),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::check_points;

    mod rotation {
        use crate::coord::Coord;
        use crate::rect::Rect;
        use crate::Shape;

        #[test]
        fn rotate_square_around_bottom_right_corner_90_degrees_twice() {
            let square = Rect::new((0, 0), (20, 20));
            let rotated = square.rotate_around(90, coord!(20, 20));

            assert_eq!(rotated.points(), coord_vec![(40, 0), (20, 20)]);

            let rotated_again = rotated.rotate_around(90, coord!(20, 20));

            assert_eq!(rotated_again.points(), coord_vec![(40, 40), (20, 20)])
        }

        #[test]
        fn rotate_rect_around_center_90_degrees() {
            let rect = Rect::new((0, 0), (40, 20));
            let rotated = rect.rotate(90);

            assert_eq!(rotated.points(), coord_vec![(30, -10), (10, 30)]);
        }
    }

    #[test]
    fn basic_outline() {
        let rect = Rect::new((0, 0), (4, 4));
        let points = rect.outline_pixels();
        check_points(
            &[
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (0, 1),
                (4, 1),
                (0, 2),
                (4, 2),
                (0, 3),
                (4, 3),
                (0, 4),
                (1, 4),
                (2, 4),
                (3, 4),
                (4, 4),
            ],
            &points,
        );
    }

    #[test]
    fn basic_filled() {
        let rect = Rect::new((3, 2), (6, 4));
        let points = rect.filled_pixels();
        check_points(
            &[
                (3, 2),
                (4, 2),
                (5, 2),
                (6, 2),
                (3, 3),
                (4, 3),
                (5, 3),
                (6, 3),
                (3, 4),
                (4, 4),
                (5, 4),
                (6, 4),
            ],
            &points,
        );
    }
}
