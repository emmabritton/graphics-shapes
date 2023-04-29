use std::collections::HashSet;
use crate::circle::Circle;
use crate::coord::Coord;
use crate::ellipse::Ellipse;
use crate::polygon::Polygon;
use crate::triangle::Triangle;
use crate::{rotate_points, Shape};
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

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let point = point.into();
        self.top_left.x <= point.x
            && self.bottom_right.x > point.x
            && self.top_left.y <= point.y
            && self.bottom_right.y > point.y
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.top_left, self.bottom_right]
    }

    fn rotate_around<P: Into<Coord>>(&self, degrees: isize, point: P) -> Self
    where
        Self: Sized,
    {
        let degrees = (degrees as f32 / 90.0).round() as isize;
        let points = rotate_points(point.into(), &self.points(), degrees * 90);
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

    fn outline_points(&self) -> Vec<Coord> {
        let mut output = HashSet::new();

        let left = self.left();
        let right = self.right();
        let top = self.top();
        let bottom = self.bottom();

        for x in left..=right {
            output.insert(Coord::new(x, top));
            output.insert(Coord::new(x, bottom));
        }
        for y in top..=bottom {
            output.insert(Coord::new(left, y));
            output.insert(Coord::new(right, y));
        }

        output.into_iter().collect()
    }

    fn filled_points(&self) -> Vec<Coord> {
        let mut output = HashSet::new();

        let left = self.left();
        let right = self.right();
        let top = self.top();
        let bottom = self.bottom();

        for y in top..=bottom {
            for x in left..=right {
                output.insert(Coord::new(x, y));
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
        let top_right = Coord::new(self.right(), self.top());
        let bottom_left = Coord::new(self.left(), self.bottom());
        (
            Triangle::new(self.top_left(), top_right, bottom_left),
            Triangle::new(self.bottom_right(), top_right, bottom_left),
        )
    }

    /// Same shape but represented as four points/lines instead of two points
    #[must_use]
    pub fn as_polygon(&self) -> Polygon {
        let top_right = Coord::new(self.right(), self.top());
        let bottom_left = Coord::new(self.left(), self.bottom());
        Polygon::new(&[self.top_left, top_right, self.bottom_right, bottom_left])
    }

    #[must_use]
    pub fn as_ellipse(&self) -> Ellipse {
        Ellipse::from_points(&self.points())
    }
}

#[cfg(test)]
mod test {
    use crate::test::check_points;
    use super::*;

    #[test]
    fn basic_outline() {
        let rect = Rect::new((0, 0), (4, 4));
        let points = rect.outline_points();
        check_points(&[
            (0, 0), (1, 0), (2, 0), (3, 0), (4, 0),
            (0, 1), (4, 1),
            (0, 2), (4, 2),
            (0, 3), (4, 3),
            (0, 4), (1, 4), (2, 4), (3, 4), (4, 4),
        ], &points);
    }

    #[test]
    fn basic_filled() {
        let rect = Rect::new((3, 2), (6, 4));
        let points = rect.filled_points();
        check_points(&[
            (3, 2), (4, 2) , (5, 2), (6, 2),
            (3, 3), (4, 3) , (5, 3), (6, 3),
            (3, 4), (4, 4) , (5, 4), (6, 4),
        ], &points);
    }
}