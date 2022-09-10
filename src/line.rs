use crate::coord::Coord;
use crate::Shape;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum LineType {
    Point,
    Horizontal,
    Vertical,
    Angled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line {
    start: Coord,
    end: Coord,
    len: usize,
    line_type: LineType,
    angle: isize,
}

impl Line {
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(start: P1, end: P2) -> Self {
        let start = start.into();
        let end = end.into();
        let line_type = if start == end {
            LineType::Point
        } else if start.x == end.x {
            LineType::Horizontal
        } else if start.y == end.y {
            LineType::Vertical
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
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn angle(&self) -> isize {
        self.angle
    }

    pub fn start(&self) -> Coord {
        self.start
    }

    pub fn end(&self) -> Coord {
        self.end
    }

    pub fn line_type(&self) -> LineType {
        self.line_type
    }
}

impl Shape for Line {
    fn from_points(points: Vec<Coord>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        Line::new(points[0], points[1])
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let point = point.into();
        match self.line_type {
            LineType::Point => self.start == point,
            LineType::Horizontal => {
                self.start.y == point.y && self.start.x <= point.x && point.x <= self.end.x
            }
            LineType::Vertical => {
                self.start.x == point.x && self.start.y <= point.y && point.y <= self.end.y
            }
            LineType::Angled => self.start.distance(point) + self.end.distance(point) == self.len,
        }
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.start, self.end]
    }

    fn center(&self) -> Coord {
        self.start.mid_point(self.end)
    }

    fn left(&self) -> isize {
        self.start.x
    }

    fn right(&self) -> isize {
        self.end.x
    }

    fn top(&self) -> isize {
        self.start.y
    }

    fn bottom(&self) -> isize {
        self.end.y
    }
}
