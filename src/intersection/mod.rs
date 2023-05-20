pub mod circle;
pub mod ellipse;
pub mod line;
pub mod polygon;
pub mod rect;
mod shared;
pub mod triangle;

use crate::prelude::*;

pub trait IntersectsShape {
    /// Returns true if `rect` intersects `self`
    #[must_use]
    fn intersects_rect(&self, rect: &Rect) -> bool;

    /// Returns true if `circle` intersects `self`
    #[must_use]
    fn intersects_circle(&self, circle: &Circle) -> bool;

    /// Returns true if `line` intersects `self`
    #[must_use]
    fn intersects_line(&self, line: &Line) -> bool;

    /// Returns true if `triangle` intersects `self`
    #[must_use]
    fn intersects_triangle(&self, triangle: &Triangle) -> bool;

    /// Returns true if `ellipse` intersects `self`
    #[must_use]
    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool;

    /// Returns true if `polygon` intersects `self`
    #[must_use]
    fn intersects_polygon(&self, polygon: &Polygon) -> bool;
}
