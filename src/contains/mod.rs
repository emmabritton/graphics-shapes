pub mod circle;
// pub mod ellipse;
pub mod line;
pub mod polygon;
pub mod rect;
pub mod triangle;

use crate::prelude::*;

/// A shape counts as contained if it is fully inside
pub trait ContainsShape {
    /// Returns true if `self` contains `rect`
    #[must_use]
    fn contains_rect(&self, rect: &Rect) -> bool
    where
        Self: Shape + Sized,
    {
        contains_points(self, rect)
    }

    /// Returns true if `self` contains `circle`
    #[must_use]
    fn contains_circle(&self, circle: &Circle) -> bool
    where
        Self: Shape + Sized,
    {
        contains_points(self, circle)
    }

    /// Returns true if `self` contains `line`
    #[must_use]
    fn contains_line(&self, line: &Line) -> bool
    where
        Self: Shape + Sized,
    {
        contains_points(self, line)
    }

    /// Returns true if `self` contains `triangle`
    #[must_use]
    fn contains_triangle(&self, triangle: &Triangle) -> bool
    where
        Self: Shape + Sized,
    {
        contains_points(self, triangle)
    }

    // /// Returns true if `self` contains `ellipse`
    // #[must_use]
    // fn contains_ellipse(&self, ellipse: &Ellipse) -> bool
    // where
    //     Self: Shape + Sized,
    // {
    //     contains_points(self, ellipse)
    // }

    /// Returns true if `self` contains `polygon`
    #[must_use]
    fn contains_polygon(&self, polygon: &Polygon) -> bool
    where
        Self: Shape + Sized,
    {
        contains_points(self, polygon)
    }
}

#[inline]
fn contains_points(shape: &dyn Shape, other: &dyn Shape) -> bool {
    for point in other.points() {
        if !shape.contains(point) {
            return false;
        }
    }
    true
}
