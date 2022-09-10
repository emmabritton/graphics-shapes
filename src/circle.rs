use crate::coord::Coord;
use crate::Shape;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Circle {
    center: Coord,
    radius: usize,
}

impl Circle {
    pub fn new<P: Into<Coord>>(center: P, radius: usize) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}

impl Circle {
    /// Radius of circle
    ///
    /// Distance from center to edge
    pub fn radius(&self) -> usize {
        self.radius
    }
}

impl Shape for Circle {
    /// must be [center, edge]
    fn from_points(points: Vec<Coord>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        let radius = points[0].distance(points[1]);
        Circle::new(points[0], radius)
    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        Circle::new(self.center + delta.into(), self.radius)
    }

    fn move_to<P: Into<Coord>>(&self, point: P) -> Self {
        Circle::new(point.into(), self.radius)
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let dist = self.center.distance(point.into());
        dist <= self.radius
    }

    /// Returns [center, edge_at_0_degrees]
    fn points(&self) -> Vec<Coord> {
        vec![self.center, Coord::from_angle(self.center, self.radius, 0)]
    }

    fn center(&self) -> Coord {
        self.center
    }

    fn left(&self) -> isize {
        self.center.x - (self.radius as isize)
    }

    fn right(&self) -> isize {
        self.center.x + (self.radius as isize)
    }

    fn top(&self) -> isize {
        self.center.y - (self.radius as isize)
    }

    fn bottom(&self) -> isize {
        self.center.y + (self.radius as isize)
    }
}
