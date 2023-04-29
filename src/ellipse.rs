use crate::circle::Circle;
use crate::coord::Coord;
use crate::line::Line;
use crate::rect::Rect;
use crate::Shape;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ellipse {
    center: Coord,
    width: usize,
    height: usize,
}

impl Ellipse {
    #[must_use]
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
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Shape for Ellipse {
    /// must be [top_left, bottom_right]
    fn from_points(points: &[Coord]) -> Self
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
        ((point.x - self.center.x) ^ 2) / ((self.width as isize) ^ 2)
            + ((point.y - self.center.y) ^ 2) / ((self.height as isize) ^ 2)
            <= 1
    }

    /// Returns [top_left, bottom_right]
    fn points(&self) -> Vec<Coord> {
        vec![
            Coord::new(self.left(), self.top()),
            Coord::new(self.right(), self.bottom()),
        ]
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

    fn outline_points(&self) -> Vec<Coord> {
        let center_x = self.center.x;
        let center_y = self.center.y;
        let rx = (self.width / 2) as f32;
        let ry = (self.height / 2) as f32;
        let mut output = vec![];

        let mut x = 0;
        let mut y = ry as isize;
        let mut p1 = ry * ry - (rx * rx) * ry + (rx * rx) * (0.25);
        let mut dx = 2.0 * (ry * ry) * (x as f32);
        let mut dy = 2.0 * (rx * rx) * (y as f32);
        while dx < dy {
            output.push(Coord::new(center_x + x, center_y + y));
            output.push(Coord::new(center_x - x, center_y + y));
            output.push(Coord::new(center_x + x, center_y - y));
            output.push(Coord::new(center_x - x, center_y - y));
            if p1 < 0.0 {
                x += 1;
                dx = 2.0 * (ry * ry) * (x as f32);
                p1 += dx + (ry * ry);
            } else {
                x += 1;
                y -= 1;
                dx = 2.0 * (ry * ry) * (x as f32);
                dy = 2.0 * (rx * rx) * (y as f32);
                p1 += dx - dy + (ry * ry);
            }
        }
        let mut p2 = (ry * ry) * ((x as f32) + 0.5) * ((x as f32) + 0.5)
            + (rx * rx) * ((y as f32) - 1.0) * ((y as f32) - 1.0)
            - (rx * rx) * (ry * ry);

        while y >= 0 {
            output.push(Coord::new(center_x + x, center_y + y));
            output.push(Coord::new(center_x - x, center_y + y));
            output.push(Coord::new(center_x + x, center_y - y));
            output.push(Coord::new(center_x - x, center_y - y));
            if p2 > 0.0 {
                y -= 1;
                dy = 2.0 * (rx * rx) * (y as f32);
                p2 -= dy + (rx * rx);
            } else {
                x += 1;
                y -= 1;
                dy -= 2.0 * (rx * rx);
                dx += 2.0 * (ry * ry);
                p2 += dx - dy + (rx * rx);
            }
        }

        output
    }

    fn filled_points(&self) -> Vec<Coord> {
        let mut output = vec![];
        let height = self.height as isize / 2;
        let width = self.width as isize / 2;
        let height_sq = height * height;
        let width_sq = width * width;
        let limit = height_sq * width_sq;
        for y in -height..height {
            let y_amount = y * y * width_sq;
            for x in -width..width {
                if x * x * height_sq + y_amount <= limit {
                    output.push(Coord::new(self.center.x + x, self.center.y + y));
                }
            }
        }
        output
    }
}

impl Ellipse {
    #[must_use]
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    #[must_use]
    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    #[must_use]
    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.top()), (self.center.x, self.bottom()))
    }

    /// Create line from center to top edge at 0 degrees
    #[must_use]
    pub fn as_radius_line(&self) -> Line {
        Line::new((self.center.x, self.center.y), (self.center.x, self.top()))
    }

    #[must_use]
    pub fn as_circle(&self) -> Option<Circle> {
        if self.width == self.height {
            Some(Circle::new(self.center, self.width / 2))
        } else {
            None
        }
    }
}
