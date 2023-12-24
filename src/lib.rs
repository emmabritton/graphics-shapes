//! Shapes for Graphics
//!
//! Provides shapes for simple graphics
//!
//! ```
//! # use graphics_shapes::coord::Coord;
//! # use graphics_shapes::rect::Rect;
//! # use graphics_shapes::{coord, Shape};
//! # use graphics_shapes::triangle::Triangle;
//! let rect = Rect::new((10,10),(20,20));
//! assert!(rect.contains(coord!(15,15)));
//! let triangle = Triangle::new((34,5),(12,30),(9,10));
//! let rotated = triangle.rotate(45);
//!
//! let start = coord!(20,130);
//! let dist = start.distance((30,130));
//!```

#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::coord::Coord;
use crate::general_math::{rotate_points, scale_points};
use crate::prelude::*;
use crate::shape_box::ShapeBox;
use fnv::FnvHashSet;
use std::any::Any;

pub mod circle;
#[macro_use]
pub mod coord;
pub mod contains;
pub mod ellipse;
pub mod general_math;
pub mod intersection;
pub mod lerp;
pub mod line;
pub mod polygon;
pub mod rect;
pub mod shape_box;
pub mod triangle;

pub mod prelude {
    pub use crate::circle::*;
    pub use crate::contains::ContainsShape;
    pub use crate::coord;
    pub use crate::coord::*;
    pub use crate::ellipse::*;
    pub use crate::intersection::IntersectsShape;
    pub use crate::line::*;
    pub use crate::polygon::*;
    pub use crate::rect::*;
    pub use crate::triangle::*;
    pub use crate::IntersectsContains;
    pub use crate::Shape;
}

pub trait AnyToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AnyToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait Shape: AnyToAny {
    /// create this shape from a list of points (corners of a shape or tips of a line)
    #[must_use]
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized;

    #[must_use]
    fn rebuild(&self, points: &[Coord]) -> Self
    where
        Self: Sized;

    /// change every point by +`delta`
    #[must_use]
    fn translate_by(&self, delta: Coord) -> Self
    where
        Self: Sized,
    {
        let points: Vec<Coord> = self.points().iter().map(|p| *p + delta).collect();
        self.rebuild(&points)
    }

    /// moves the shapes first point to `point`
    /// (and changes every other point to match their original distance and angle)
    ///
    /// As this moves self.points()[0] the result might be unexpected if the shape was created
    /// right to left and/or bottom to top
    #[must_use]
    fn move_to(&self, point: Coord) -> Self
    where
        Self: Sized,
    {
        let diff = (point) - self.points()[0];
        self.translate_by(diff)
    }

    /// Moves the shapes center to `point`
    /// (and changes every other point to match their original distance and angle)
    ///
    /// As this moves relative to self.points()[0] the result might be unexpected if the shape was created
    /// right to left and/or bottom to top
    #[must_use]
    fn move_center_to(&self, point: Coord) -> Self
    where
        Self: Sized,
    {
        let diff = point - self.center();
        self.translate_by(diff)
    }

    /// Returns true if the shape contains point
    #[must_use]
    fn contains(&self, point: Coord) -> bool;

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
    fn rotate_around(&self, degrees: isize, point: Coord) -> Self
    where
        Self: Sized,
    {
        let points = rotate_points(point, &self.points(), degrees);
        self.rebuild(&points)
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
        coord!(self.left(), self.top())
    }

    #[must_use]
    fn top_right(&self) -> Coord {
        coord!(self.right(), self.top())
    }

    #[must_use]
    fn bottom_left(&self) -> Coord {
        coord!(self.left(), self.bottom())
    }

    #[must_use]
    fn bottom_right(&self) -> Coord {
        coord!(self.right(), self.bottom())
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
    fn scale_around(&self, factor: f32, point: Coord) -> Self
    where
        Self: Sized,
    {
        let points = scale_points(point, &self.points(), factor);
        self.rebuild(&points)
    }

    /// The coords for drawing the shape outline, the points may be in any order
    /// This should be cached rather than called per frame
    #[must_use]
    fn outline_pixels(&self) -> Vec<Coord>;

    /// The coords for drawing the filled shape, the points may be in any order
    /// This should be cached rather than called per frame
    #[must_use]
    fn filled_pixels(&self) -> Vec<Coord>;

    #[must_use]
    fn to_shape_box(&self) -> ShapeBox;
}

//Separate so `Shape`s don't have to implement Contains and Intersects
pub trait IntersectsContains: Shape + ContainsShape + IntersectsShape + Sized {
    /// Returns
    /// * Some(true) if `self` contains `other`
    /// * Some(false) if `self` does not contain `other`
    /// * None if `other` isn't a supported `Shape`
    #[must_use]
    fn contains_shape(&self, other: &dyn Shape) -> Option<bool> {
        if let Some(line) = other.as_any().downcast_ref::<Line>() {
            return Some(self.contains_line(line));
        }
        if let Some(rect) = other.as_any().downcast_ref::<Rect>() {
            return Some(self.contains_rect(rect));
        }
        if let Some(triangle) = other.as_any().downcast_ref::<Triangle>() {
            return Some(self.contains_triangle(triangle));
        }
        if let Some(polygon) = other.as_any().downcast_ref::<Polygon>() {
            return Some(self.contains_polygon(polygon));
        }
        if let Some(circle) = other.as_any().downcast_ref::<Circle>() {
            return Some(self.contains_circle(circle));
        }
        if let Some(ellipse) = other.as_any().downcast_ref::<Ellipse>() {
            return Some(self.contains_ellipse(ellipse));
        }
        if let Some(shapebox) = other.as_any().downcast_ref::<ShapeBox>() {
            return Some(match shapebox {
                ShapeBox::Line(line) => self.contains_line(line),
                ShapeBox::Rect(rect) => self.contains_rect(rect),
                ShapeBox::Triangle(triangle) => self.contains_triangle(triangle),
                ShapeBox::Circle(circle) => self.contains_circle(circle),
                ShapeBox::Ellipse(ellipse) => self.contains_ellipse(ellipse),
                ShapeBox::Polygon(polygon) => self.contains_polygon(polygon),
            });
        }
        None
    }

    /// Returns
    /// * Some(true) if `self` intersects `other`
    /// * Some(false) if `self` does not intersects `other`
    /// * None if `other` isn't a supported `Shape`
    #[must_use]
    fn intersects_shape(&self, other: &dyn Shape) -> Option<bool> {
        if let Some(line) = other.as_any().downcast_ref::<Line>() {
            return Some(self.intersects_line(line));
        }
        if let Some(rect) = other.as_any().downcast_ref::<Rect>() {
            return Some(self.intersects_rect(rect));
        }
        if let Some(triangle) = other.as_any().downcast_ref::<Triangle>() {
            return Some(self.intersects_triangle(triangle));
        }
        if let Some(polygon) = other.as_any().downcast_ref::<Polygon>() {
            return Some(self.intersects_polygon(polygon));
        }
        if let Some(circle) = other.as_any().downcast_ref::<Circle>() {
            return Some(self.intersects_circle(circle));
        }
        if let Some(ellipse) = other.as_any().downcast_ref::<Ellipse>() {
            return Some(self.intersects_ellipse(ellipse));
        }
        if let Some(shapebox) = other.as_any().downcast_ref::<ShapeBox>() {
            return Some(match shapebox {
                ShapeBox::Line(line) => self.intersects_line(line),
                ShapeBox::Rect(rect) => self.intersects_rect(rect),
                ShapeBox::Triangle(triangle) => self.intersects_triangle(triangle),
                ShapeBox::Circle(circle) => self.intersects_circle(circle),
                ShapeBox::Ellipse(ellipse) => self.intersects_ellipse(ellipse),
                ShapeBox::Polygon(polygon) => self.intersects_polygon(polygon),
            });
        }
        None
    }
}

fn new_hash_set() -> FnvHashSet<Coord> {
    FnvHashSet::default()
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    pub fn check_points(expected: &[(isize, isize)], actual: &[Coord]) {
        let mut expected: Vec<Coord> = expected.iter().map(|(x, y)| coord!(*x, *y)).collect();
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

    #[test]
    fn generic_contains() {
        let outer = Rect::new((0, 0), (10, 10));
        let inner = Line::new((2, 2), (4, 4));

        assert!(outer.contains_line(&inner));
        assert_eq!(outer.contains_shape(&inner), Some(true));

        let outside = Line::new((-3, 200), (-1, -1));
        assert!(!outer.contains_line(&outside));
        assert_eq!(outer.contains_shape(&outside), Some(false));
    }

    #[test]
    fn shapebox_intersects() {
        let line = Line::new((10, 10), (20, 20));
        let rect = Rect::new((5, 5), (15, 15));
        let shape_box = rect.to_shape_box();
        assert_eq!(line.intersects_shape(&rect), Some(true));
        assert_eq!(line.intersects_shape(&shape_box), Some(true));
    }
}
