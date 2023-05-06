use crate::coord::Coord;
use crate::ellipse::Ellipse;
use crate::line::Line;
use crate::rect::Rect;
use crate::{new_hash_set, Shape};
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Circle {
    center: Coord,
    radius: usize,
}

impl Circle {
    #[must_use]
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
    #[inline]
    #[must_use]
    pub fn radius(&self) -> usize {
        self.radius
    }
}

impl Shape for Circle {
    /// must be [center, edge]
    fn from_points(points: &[Coord]) -> Self
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

    #[inline]
    fn center(&self) -> Coord {
        self.center
    }

    #[inline]
    fn left(&self) -> isize {
        self.center.x - (self.radius as isize)
    }

    #[inline]
    fn right(&self) -> isize {
        self.center.x + (self.radius as isize)
    }

    #[inline]
    fn top(&self) -> isize {
        self.center.y - (self.radius as isize)
    }

    #[inline]
    fn bottom(&self) -> isize {
        self.center.y + (self.radius as isize)
    }

    fn outline_pixels(&self) -> Vec<Coord> {
        let cx = self.center.x;
        let cy = self.center.y;
        let mut d = (5_isize - (self.radius as isize) * 4) / 4;
        let mut x = 0;
        let mut y = self.radius as isize;
        let mut output = new_hash_set();

        while x <= y {
            output.insert(Coord::new(cx + x, cy + y));
            output.insert(Coord::new(cx + x, cy - y));
            output.insert(Coord::new(cx - x, cy + y));
            output.insert(Coord::new(cx - x, cy - y));
            output.insert(Coord::new(cx + y, cy + x));
            output.insert(Coord::new(cx + y, cy - x));
            output.insert(Coord::new(cx - y, cy + x));
            output.insert(Coord::new(cx - y, cy - x));
            if d < 0 {
                d += 2 * x + 1
            } else {
                d += 2 * (x - y) + 1;
                y -= 1;
            }
            x += 1;
        }

        output.into_iter().collect()
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        let mut output = new_hash_set();
        let cx = self.center.x;
        let cy = self.center.y;
        let squared_radius = (self.radius * self.radius) as isize;
        for y in 0..(self.radius as isize) {
            let up = cy - y;
            let down = cy + y;
            let half_width = (((squared_radius - y * y) as f64).sqrt().round() as isize).max(0);
            for x in 0..=half_width {
                let left = cx - x;
                let right = cx + x;
                output.insert(Coord::new(left, up));
                output.insert(Coord::new(right, up));
                output.insert(Coord::new(left, down));
                output.insert(Coord::new(right, down));
            }
        }
        output.into_iter().collect()
    }
}

impl Circle {
    #[must_use]
    #[deprecated(since = "0.2.0", note = "use as_outer_rect instead")]
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    /// Rectangle that surrounds the circle
    #[must_use]
    pub fn as_outer_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    /// Rectangle that fits inside the circle
    #[must_use]
    pub fn as_inner_rect(&self) -> Rect {
        let top_left = Coord::from_angle(self.center, self.radius, 315);
        let bottom_right = Coord::from_angle(self.center, self.radius, 135);
        Rect::new(top_left, bottom_right)
    }

    /// Create line from center to top edge at 0 degrees
    #[must_use]
    pub fn as_radius_line(&self) -> Line {
        Line::new((self.center.x, self.center.y), (self.center.x, self.top()))
    }

    /// Line from left to right
    #[must_use]
    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    /// Line from top to bottom
    #[must_use]
    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.top()), (self.center.x, self.bottom()))
    }

    #[must_use]
    pub fn as_ellipse(&self) -> Ellipse {
        Ellipse::new(self.center, self.radius * 2, self.radius * 2)
    }
}
