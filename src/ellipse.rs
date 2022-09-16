use crate::coord::Coord;
use crate::line::Line;
use crate::rect::Rect;
use crate::Shape;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use crate::circle::Circle;

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ellipse {
    center: Coord,
    width: usize,
    height: usize,
}

impl Ellipse {
    pub fn new<P: Into<Coord>>(center: P, width: usize, height: usize) -> Self {
        Self {
            center: center.into(),
            width,
            height,
        }
    }
}

impl Ellipse {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Shape for Ellipse {
    /// must be [top_left, bottom_right]
    fn from_points(points: Vec<Coord>) -> Self
        where
            Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        let width = points[1].x - points[0].x;
        let height = points[1].y - points[0].x;
        let center = points[0].mid_point(points[1]);
        Ellipse::new(center, width.max(0) as usize, height.max(0) as usize)
    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        Ellipse::new(self.center + delta.into(), self.width, self.height)
    }

    fn move_to<P: Into<Coord>>(&self, point: P) -> Self {
        Ellipse::new(point.into(), self.width, self.height)
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let point = point.into();
        (point.x - self.center.x) ^ 2 / (self.width as isize) ^ 2 + (point.y - self.center.y) ^ 2 / (self.height as isize) ^ 2 <= 1
    }

    /// Returns [top_left, bottom_right]
    fn points(&self) -> Vec<Coord> {
        vec![Coord::new(self.left(), self.top()), Coord::new(self.right(), self.bottom())]
    }

    #[inline]
    fn center(&self) -> Coord {
        self.center
    }

    #[inline]
    fn left(&self) -> isize {
        self.center.x - (self.width as isize) / 2
    }

    #[inline]
    fn right(&self) -> isize {
        self.center.x + (self.width as isize) / 2
    }

    #[inline]
    fn top(&self) -> isize {
        self.center.y - (self.height as isize) / 2
    }

    #[inline]
    fn bottom(&self) -> isize {
        self.center.y + (self.height as isize) / 2
    }
}

impl Ellipse {
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.top()), (self.center.x, self.bottom()))
    }

    /// Create line from center to right edge at 0 degrees
    pub fn as_radius_line(&self) -> Line {
        Line::new((self.center.x, self.center.y), (self.right(), self.center.y))
    }

    pub fn as_circle(&self) -> Option<Circle> {
        if self.width == self.height {
            Some(Circle::new(self.center, self.width / 2))
        } else {
            None
        }
    }
}
