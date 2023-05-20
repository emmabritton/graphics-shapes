use crate::new_hash_set;
use crate::prelude::*;
use crate::shape_box::ShapeBox;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ellipse {
    center: Coord,
    top: Coord,
    right: Coord,
    rotation: isize,
}

impl IntersectsContains for Ellipse {}

impl Ellipse {
    #[must_use]
    pub fn new<P: Into<Coord>>(center: P, width: usize, height: usize) -> Self {
        let center = center.into();
        Self {
            center,
            top: center - (0, height / 2),
            right: center + (width / 2, 0),
            rotation: 0,
        }
    }

    pub fn new_rotated<P1: Into<Coord>, P2: Into<Coord>, P3: Into<Coord>>(
        center: P1,
        top: P2,
        right: P3,
    ) -> Self {
        let center = center.into();
        let top = top.into();
        let right = right.into();
        let angle = center.angle_to(top);
        let right = Coord::from_angle(center, center.distance(right), -angle + 90);
        let top = Coord::from_angle(center, center.distance(top), -angle);
        Self {
            center,
            top,
            right,
            rotation: angle,
        }
    }
}

impl Ellipse {
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        (self.right() - self.left()).unsigned_abs()
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        (self.bottom() - self.top()).unsigned_abs()
    }

    pub fn angle(&self) -> isize {
        self.rotation
    }

    #[inline(always)]
    fn no_rotate_point(x: isize, y: isize, _: Coord, _: isize) -> Coord {
        coord!(x, y)
    }

    #[inline(always)]
    fn rotate_point(x: isize, y: isize, center: Coord, degrees: isize) -> Coord {
        let orig = coord!(x, y);
        let offset = center.angle_to(orig) + degrees;
        Coord::from_angle(center, center.distance(orig), offset)
    }
}

impl Shape for Ellipse {
    /// must be [center, top, right]
    /// see [Ellipse::points]
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 3);
        let center = points[0];
        let top = points[1];
        let right = points[2];
        let rotation = center.angle_to(top);
        let top = Coord::from_angle(center, top.distance(center), 0);
        let right = Coord::from_angle(center, right.distance(center), 90);
        Ellipse {
            center,
            top,
            right,
            rotation,
        }
    }

    fn rebuild(&self, points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Ellipse::from_points(points)
    }

    fn translate_by(&self, delta: Coord) -> Self {
        let points: Vec<Coord> = self.points().into_iter().map(|c| c + delta).collect();
        Ellipse::from_points(&points)
    }

    fn move_to(&self, point: Coord) -> Self {
        self.move_center_to(point)
    }

    fn contains(&self, point: Coord) -> bool {
        let point = point;
        ((point.x - self.center.x) ^ 2) / ((self.width() as isize) ^ 2)
            + ((point.y - self.center.y) ^ 2) / ((self.height() as isize) ^ 2)
            <= 1
    }

    /// Returns [center, top, right]
    ///
    /// * Center is center point
    /// * Top is center - height/2, at 0 degrees
    /// * Right is center + width/2, at 90 degrees
    fn points(&self) -> Vec<Coord> {
        vec![
            self.center,
            Coord::from_angle(self.center, self.center.distance(self.top), self.rotation),
            Coord::from_angle(
                self.center,
                self.center.distance(self.right),
                self.rotation + 90,
            ),
        ]
    }

    #[inline]
    fn center(&self) -> Coord {
        self.center
    }

    #[inline]
    fn left(&self) -> isize {
        self.right.x - (self.center.distance(self.right) * 2) as isize
    }

    #[inline]
    fn right(&self) -> isize {
        self.right.x
    }

    #[inline]
    fn top(&self) -> isize {
        self.top.y
    }

    #[inline]
    fn bottom(&self) -> isize {
        self.top.y + (self.center.distance(self.top) * 2) as isize
    }

    fn outline_pixels(&self) -> Vec<Coord> {
        let center = self.center;
        let degrees = self.rotation;
        let rotate = if degrees == 0 {
            Self::no_rotate_point
        } else {
            Self::rotate_point
        };

        let center_x = self.center.x;
        let center_y = self.center.y;
        let rx = (self.width() / 2) as f32;
        let ry = (self.height() / 2) as f32;
        let mut output = new_hash_set();

        let mut x = 0;
        let mut y = ry as isize;
        let mut p1 = ry * ry - (rx * rx) * ry + (rx * rx) * (0.25);
        let mut dx = 2.0 * (ry * ry) * (x as f32);
        let mut dy = 2.0 * (rx * rx) * (y as f32);
        while dx < dy {
            output.insert(rotate(center_x + x, center_y + y, center, degrees));
            output.insert(rotate(center_x - x, center_y + y, center, degrees));
            output.insert(rotate(center_x + x, center_y - y, center, degrees));
            output.insert(rotate(center_x - x, center_y - y, center, degrees));
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
            output.insert(rotate(center_x + x, center_y + y, center, degrees));
            output.insert(rotate(center_x - x, center_y + y, center, degrees));
            output.insert(rotate(center_x + x, center_y - y, center, degrees));
            output.insert(rotate(center_x - x, center_y - y, center, degrees));
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

        output.into_iter().collect()
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        let mut output = new_hash_set();
        let height = self.height() as isize / 2;
        let width = self.width() as isize / 2;
        let height_sq = height * height;
        let width_sq = width * width;
        let limit = height_sq * width_sq;
        for y in -height..height {
            let y_amount = y * y * width_sq;
            for x in -width..width {
                if x * x * height_sq + y_amount <= limit {
                    output.insert(coord!(self.center.x + x, self.center.y + y));
                }
            }
        }
        output.into_iter().collect()
    }

    fn to_shape_box(&self) -> ShapeBox {
        ShapeBox::Ellipse(self.clone())
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

    /// Returns a circle if the ellipse height and width are the same
    #[must_use]
    pub fn as_circle(&self) -> Option<Circle> {
        if self.width() == self.height() {
            Some(Circle::new(self.center, self.width() / 2))
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_largest_circle(&self) -> Circle {
        let radius = self.width().max(self.height());
        Circle::new(self.center, radius)
    }

    #[must_use]
    pub fn as_smallest_circle(&self) -> Circle {
        let radius = self.width().min(self.height());
        Circle::new(self.center, radius)
    }

    #[must_use]
    pub fn as_polygon(&self) -> Polygon {
        let x = self.center.x as f64;
        let y = self.center.y as f64;
        let w = self.width() as f64 / 2.0;
        let h = self.height() as f64 / 2.0;

        let segments = (((w + h) / 2.0) * 20.0).sqrt().floor().max(8.0) as usize;
        let points = discretise_ellipse(x, y, w, h, segments);

        Polygon::from_points(&points).rotate(self.angle())
    }
}

fn discretise_ellipse(x: f64, y: f64, a: f64, b: f64, segments: usize) -> Vec<Coord> {
    let angle_shift = 6.29 / (segments as f64);
    let mut phi = 0.0;
    let mut vertices = vec![];
    for _ in 0..segments {
        phi += angle_shift;
        vertices.push(coord!(x + a * phi.cos(), y + b * phi.sin()));
    }

    vertices
}

#[cfg(test)]
mod test {
    use crate::circle::Circle;
    use crate::ellipse::Ellipse;
    use crate::Shape;

    #[test]
    fn check_circle_ellipse() {
        let ellipse = Ellipse::new((40, 40), 20, 20);
        let gen_circle = ellipse.as_circle().unwrap();
        let expected = Circle::new((40, 40), 10);
        assert_eq!(gen_circle, expected);
    }

    #[test]
    fn translate() {
        let ellipse = Ellipse::new((40, 40), 20, 20);
        let moved = ellipse.move_center_to(coord!(10, 10));
        assert_eq!(ellipse.width(), moved.width());
        assert_eq!(ellipse.height(), moved.height());
        assert_eq!(ellipse.angle(), moved.angle());
    }

    #[test]
    fn to_from_points() {
        let ellipse = Ellipse::new((100, 100), 30, 60);
        assert_eq!(ellipse.center, coord!(100, 100));
        assert_eq!(ellipse.top, coord!(100, 70));
        assert_eq!(ellipse.right, coord!(115, 100));
        let points = ellipse.points();
        assert_eq!(points, coord_vec![(100, 100), (100, 70), (115, 100)]);
        let gen_ellipse = Ellipse::from_points(&points);
        assert_eq!(ellipse.width(), gen_ellipse.width());
        assert_eq!(ellipse.height(), gen_ellipse.height());
        assert_eq!(ellipse, gen_ellipse);
    }

    #[test]
    fn rotation() {
        let ellipse = Ellipse::new((100, 100), 200, 50);
        assert_eq!(
            ellipse.points(),
            vec![coord!(100, 100), coord!(100, 75), coord!(200, 100)]
        );
        assert_eq!(ellipse.center, coord!(100, 100));
        assert_eq!(ellipse.top, coord!(100, 75));
        assert_eq!(ellipse.right, coord!(200, 100));
        assert_eq!(ellipse.rotation, 0);
        let rotated = ellipse.rotate(90);
        assert_eq!(
            rotated.points(),
            vec![coord!(100, 100), coord!(125, 100), coord!(100, 200)]
        );
        assert_eq!(rotated.center, coord!(100, 100));
        assert_eq!(rotated.top, coord!(100, 75));
        assert_eq!(rotated.right, coord!(200, 100));
        assert_eq!(rotated.rotation, 90);
    }

    #[test]
    fn move_center() {
        let ellipse = Ellipse::new((100, 100), 20, 20);
        let moved = ellipse.move_center_to(coord!(50, 50));

        assert_eq!(moved.center, coord!(50, 50));

        let ellipse = Ellipse::new((100, 100), 20, 20);
        let moved = ellipse.move_center_to(coord!(120, 50));

        assert_eq!(moved.center, coord!(120, 50));
    }

    #[test]
    fn move_center_rotated() {
        let ellipse = Ellipse::new((100, 100), 20, 20).rotate(45);
        let moved = ellipse.move_center_to(coord!(50, 50));

        assert_eq!(ellipse.angle(), 45);
        assert_eq!(moved.angle(), 45);
        assert_eq!(moved.center, coord!(50, 50));
    }
}
