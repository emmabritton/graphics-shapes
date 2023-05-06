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
use fnv::FnvHashSet;

pub mod circle;
#[macro_use]
pub mod coord;
pub mod ellipse;
mod general_math;
pub mod lerp;
pub mod line;
pub mod polygon;
pub mod rect;
pub mod triangle;

pub mod prelude {
    pub use crate::circle::Circle;
    pub use crate::coord::*;
    pub use crate::ellipse::Ellipse;
    pub use crate::line::Line;
    pub use crate::polygon::Polygon;
    pub use crate::rect::Rect;
    pub use crate::triangle::Triangle;
    pub use crate::Shape;
}

pub trait Shape {
    /// create this shape from a list of points (corners of a shape or tips of a line)
    #[must_use]
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized;

    /// change every point by +`delta`
    #[must_use]
    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self
    where
        Self: Sized,
    {
        let delta = delta.into();
        let points: Vec<Coord> = self.points().iter().map(|p| *p + delta).collect();
        Self::from_points(&points)
    }

    /// moves the shapes first point to `point`
    /// (and changes every other point to match their original distance and angle)
    ///
    /// As this moves self.points()[0] the result might be unexpected if the shape was created
    /// right to left and/or bottom to top
    #[must_use]
    fn move_to<P: Into<Coord>>(&self, point: P) -> Self
    where
        Self: Sized,
    {
        let diff = (point.into()) - self.points()[0];
        self.translate_by(diff)
    }

    /// moves the shapes center to `point`
    /// (and changes every other point to match their original distance and angle)
    ///
    /// As this moves relative to self.points()[0] the result might be unexpected if the shape was created
    /// right to left and/or bottom to top
    #[must_use]
    fn move_center_to<P: Into<Coord>>(&self, point: P) -> Self
    where
        Self: Sized,
    {
        let diff = (point.into()) - (self.center() - self.points()[0]);
        self.translate_by(diff)
    }

    /// Returns true if the shape contains point
    #[must_use]
    fn contains<P: Into<Coord>>(&self, point: P) -> bool;

    /// Points(corners/ends) the shape is made of
    #[must_use]
    fn points(&self) -> Vec<Coord>;

    /// Rotate shape around it's center
    #[must_use]
    fn rotate(&self, degrees: isize) -> Self
    where
        Self: Sized,
    {
        self.rotate_around(degrees, self.center())
    }

    /// Rotate shape around a point
    #[must_use]
    fn rotate_around<P: Into<Coord>>(&self, degrees: isize, point: P) -> Self
    where
        Self: Sized,
    {
        let points = rotate_points(point.into(), &self.points(), degrees);
        Self::from_points(&points)
    }

    /// Center of shape
    #[must_use]
    fn center(&self) -> Coord;

    /// x of the left most point
    #[must_use]
    fn left(&self) -> isize {
        self.points().iter().map(|p| p.x).min().unwrap()
    }

    /// x of the right most point
    #[must_use]
    fn right(&self) -> isize {
        self.points().iter().map(|p| p.x).max().unwrap()
    }

    /// y of the top most point
    #[must_use]
    fn top(&self) -> isize {
        self.points().iter().map(|p| p.y).min().unwrap()
    }

    /// y of the bottom most point
    #[must_use]
    fn bottom(&self) -> isize {
        self.points().iter().map(|p| p.y).max().unwrap()
    }

    #[must_use]
    fn top_left(&self) -> Coord {
        Coord::new(self.left(), self.top())
    }

    #[must_use]
    fn top_right(&self) -> Coord {
        Coord::new(self.right(), self.top())
    }

    #[must_use]
    fn bottom_left(&self) -> Coord {
        Coord::new(self.left(), self.bottom())
    }

    #[must_use]
    fn bottom_right(&self) -> Coord {
        Coord::new(self.right(), self.bottom())
    }

    /// Scale the shape by factor (around the center, so the change will be uniform)
    #[must_use]
    fn scale(&self, factor: f32) -> Self
    where
        Self: Sized,
    {
        self.scale_around(factor, self.center())
    }

    /// Scale the shape by factor around point
    #[must_use]
    fn scale_around<P: Into<Coord>>(&self, factor: f32, point: P) -> Self
    where
        Self: Sized,
    {
        let points = scale_points(point.into(), &self.points(), factor);
        Self::from_points(&points)
    }

    /// The coords for drawing the shape outline, the points may be in any order
    /// This should be cached rather than called per frame
    #[must_use]
    fn outline_pixels(&self) -> Vec<Coord>;

    /// The coords for drawing the filled shape, the points may be in any order
    /// This should be cached rather than called per frame
    #[must_use]
    fn filled_pixels(&self) -> Vec<Coord>;
}

fn new_hash_set() -> FnvHashSet<Coord> {
    FnvHashSet::default()
}

#[cfg(test)]
mod test {
    use crate::coord::Coord;

    pub fn check_points(expected: &[(isize, isize)], actual: &[Coord]) {
        let mut expected: Vec<Coord> = expected
            .iter()
            .map(|(x, y)| Coord::new(*x as isize, *y as isize))
            .collect();
        let mut unexpected = vec![];
        for point in actual {
            if let Some(i) = expected.iter().position(|p| p == point) {
                expected.remove(i);
            } else {
                unexpected.push(point);
            }
        }
        let mut message = String::new();
        if !expected.is_empty() {
            message.push_str(&format!("Points not found: {:?}", expected));
        }
        if !unexpected.is_empty() {
            message.push_str(&format!("Points unexpectedly found: {:?}", unexpected));
        }
        if !message.is_empty() {
            panic!("{message}");
        }
    }
}
