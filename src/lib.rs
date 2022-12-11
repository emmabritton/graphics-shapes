//! Shapes for Graphics
//!
//! Provides shapes for simple graphics
//!
//! ```
//! # use graphics_shapes::coord::Coord;
//! # use graphics_shapes::rect::Rect;
//! # use graphics_shapes::Shape;
//! # use graphics_shapes::triangle::Triangle;
//! let rect = Rect::new((10,10),(20,20));
//! assert!(rect.contains((15,15)));
//! let triangle = Triangle::new((34,5),(12,30),(9,10));
//! let rotated = triangle.rotate(45);
//!
//! let start = Coord::new(20,130);
//! let dist = start.distance((30,130));
//!```

#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::coord::Coord;
use crate::general_math::{rotate_points, scale_points};

pub mod circle;
pub mod coord;
pub mod ellipse;
mod general_math;
pub mod lerp;
pub mod line;
pub mod polygon;
pub mod rect;
pub mod triangle;

pub trait Shape {
    /// create this shape from a list of points
    fn from_points(points: Vec<Coord>) -> Self
    where
        Self: Sized;

    /// change every point by +`delta`
    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self
    where
        Self: Sized,
    {
        let delta = delta.into();
        let points = self.points().iter().map(|p| *p + delta).collect();
        Self::from_points(points)
    }

    /// moves the shapes first point to `point`
    /// (and changes every other point to match their original distance and angle)
    ///
    /// As this moves self.point[0] the result might be unexpected if the shape was created
    /// right to left and/or bottom to top
    fn move_to<P: Into<Coord>>(&self, point: P) -> Self
    where
        Self: Sized,
    {
        let point = point.into();
        let points = self.points().iter().map(|p| *p - point).collect();
        Self::from_points(points)
    }

    /// returns true if the shape contains point
    fn contains<P: Into<Coord>>(&self, point: P) -> bool;

    /// points(corners) the shape is made of
    fn points(&self) -> Vec<Coord>;

    fn rotate(&self, degrees: isize) -> Self
    where
        Self: Sized,
    {
        self.rotate_around(degrees, self.center())
    }

    fn rotate_around<P: Into<Coord>>(&self, degrees: isize, point: P) -> Self
    where
        Self: Sized,
    {
        let points = rotate_points(point.into(), &self.points(), degrees);
        Self::from_points(points)
    }
    /// center of shape
    fn center(&self) -> Coord;

    /// x of the left most point
    fn left(&self) -> isize {
        self.points().iter().map(|p| p.x).min().unwrap()
    }

    /// x of the right most point
    fn right(&self) -> isize {
        self.points().iter().map(|p| p.x).max().unwrap()
    }

    /// y of the top most point
    fn top(&self) -> isize {
        self.points().iter().map(|p| p.y).min().unwrap()
    }

    /// y of the bottom most point
    fn bottom(&self) -> isize {
        self.points().iter().map(|p| p.y).max().unwrap()
    }

    /// scale the shape by factor (around the center, so the change will be uniform)
    fn scale(&self, factor: f32) -> Self
    where
        Self: Sized,
    {
        self.scale_around(factor, self.center())
    }

    /// scale the shape by factor around point
    fn scale_around<P: Into<Coord>>(&self, factor: f32, point: P) -> Self
    where
        Self: Sized,
    {
        let points = scale_points(point.into(), &self.points(), factor);
        Self::from_points(points)
    }
}

pub trait Intersects<T> {
    /// returns true if `shape` intersects this shape
    fn intersects(&self, shape: T) -> bool;
}
