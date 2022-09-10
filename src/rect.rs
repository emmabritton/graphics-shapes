use crate::coord::Coord;
use crate::{rotate_points, Shape};
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rect {
    top_left: Coord,
    bottom_right: Coord,
}

impl Rect {
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(top_left: P1, bottom_right: P2) -> Self {
        Self {
            top_left: top_left.into(),
            bottom_right: bottom_right.into(),
        }
    }

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
    pub fn width(&self) -> usize {
        (self.bottom_right.x - self.top_left.x).unsigned_abs()
    }

    pub fn height(&self) -> usize {
        (self.bottom_right.y - self.top_left.y).unsigned_abs()
    }

    pub fn top_left(&self) -> Coord {
        self.top_left
    }

    pub fn bottom_right(&self) -> Coord {
        self.bottom_right
    }

    pub fn is_square(&self) -> bool {
        let diff = self.bottom_right - self.top_left;
        diff.x == diff.y
    }
}

impl Shape for Rect {
    fn from_points(points: Vec<Coord>) -> Self
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
        Self::from_points(points)
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
}
