use crate::rect::Rect;
use crate::{Coord, Shape};
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TriangleAngleType {
    Acute,
    Right,
    Obtuse,
    Equiangular,
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TriangleSideType {
    Isosceles,
    Scalene,
    Equilateral,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Triangle {
    points: [Coord; 3],
    angles: [isize; 3],
    angle_type: TriangleAngleType,
    side_type: TriangleSideType,
    center: Coord,
}

impl Triangle {
    pub fn new<P1: Into<Coord>, P2: Into<Coord>, P3: Into<Coord>>(
        point1: P1,
        point2: P2,
        point3: P3,
    ) -> Self {
        let points = [point1.into(), point2.into(), point3.into()];
        let angles = [
            points[0].angle_to(points[1]),
            points[1].angle_to(points[2]),
            points[2].angle_to(points[0]),
        ];
        let angle_type = if angles.iter().any(|a| a.abs() == 90) {
            TriangleAngleType::Right
        } else if angles.iter().all(|a| a.abs() < 90) {
            TriangleAngleType::Acute
        } else if angles[0].abs() == angles[1].abs() && angles[1].abs() == angles[2].abs() {
            TriangleAngleType::Equiangular
        } else if angles.iter().any(|a| a.abs() > 90) {
            TriangleAngleType::Obtuse
        } else {
            TriangleAngleType::Other
        };
        let side_type = if points[0].distance(points[1]) == points[1].distance(points[2])
            && points[2].distance(points[0]) == points[1].distance(points[2])
        {
            TriangleSideType::Equilateral
        } else if points[0].distance(points[1]) == points[1].distance(points[2])
            || points[2].distance(points[0]) == points[1].distance(points[2])
            || points[2].distance(points[0]) == points[0].distance(points[1])
        {
            TriangleSideType::Isosceles
        } else {
            TriangleSideType::Scalene
        };
        let mut triangle = Self {
            points,
            angles,
            angle_type,
            side_type,
            center: Coord::new(0, 0),
        };
        triangle.center = Coord::new(triangle.left(), triangle.top())
            .mid_point(Coord::new(triangle.right(), triangle.bottom()));
        triangle
    }
}

impl Triangle {
    #[inline]
    pub fn angles(&self) -> [isize; 3] {
        self.angles
    }
    #[inline]
    pub fn angle_type(&self) -> &TriangleAngleType {
        &self.angle_type
    }
    #[inline]
    pub fn side_type(&self) -> &TriangleSideType {
        &self.side_type
    }
}

impl Shape for Triangle {
    fn from_points(points: Vec<Coord>) -> Self
        where
            Self: Sized,
    {
        Triangle::new(points[0], points[1], points[2])
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let point = point.into();
        let p1 = Coord::new(
            self.points[1].x - self.points[0].x,
            self.points[1].y - self.points[0].y,
        );
        let p2 = Coord::new(
            self.points[2].x - self.points[0].x,
            self.points[2].y - self.points[0].y,
        );
        let q = Coord::new(point.x - self.points[0].x, point.y - self.points[0].y);

        let s = q.cross_product(p2) as f32 / p1.cross_product(p2) as f32;
        let t = p1.cross_product(q) as f32 / p1.cross_product(p2) as f32;

        s >= 0.0 && t >= 0.0 && (s + t) <= 1.0
    }

    fn points(&self) -> Vec<Coord> {
        self.points.to_vec()
    }

    #[inline]
    fn center(&self) -> Coord {
        self.center
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnglePosition {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FlatSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl Triangle {
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    pub fn right_angle<P: Into<Coord>>(
        angle_coord: P,
        width: usize,
        height: usize,
        angle_position: AnglePosition,
    ) -> Triangle {
        let point = angle_coord.into();
        let width = width as isize;
        let height = height as isize;
        let left = point.x - width;
        let right = point.x + width;
        let top = point.y - height;
        let bottom = point.y + height;
        match angle_position {
            AnglePosition::TopLeft => Triangle::new(point, (right, point.y), (point.x, bottom)),
            AnglePosition::BottomRight => Triangle::new(point, (left, point.y), (point.x, top)),
            AnglePosition::BottomLeft => Triangle::new(point,  (right, point.y),(point.x, top)),
            AnglePosition::TopRight => Triangle::new(point, (left, point.y), (point.x, bottom))
        }
    }

    /// Create an equilateral triangle with width and height of [size] around [center]
    /// The top left would be (center.x - size / 2, center.y + size / 2) and bottom right (center.x + size / 2, center.y + size / 2)
    pub fn equilateral<P: Into<Coord>>(center: P, size: usize, flat_side: FlatSide) -> Triangle {
        let point = center.into();
        let dist = (size / 2) as isize;
        let left = point.x - dist;
        let right = point.x + dist;
        let top = point.y - dist;
        let bottom = point.y + dist;
        match flat_side {
            FlatSide::Top => Triangle::new((left, top), (right, top), (point.x, bottom)),
            FlatSide::Bottom => Triangle::new((left, bottom), (right, bottom), (point.x, top)),
            FlatSide::Left => Triangle::new((left, top), (left, bottom), (right, point.y)),
            FlatSide::Right => Triangle::new((right, top), (right, bottom), (left, point.y)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::triangle::{AnglePosition, Triangle};

    #[test]
    fn right_angle_triangles() {
        let triangle = Triangle::right_angle((100, 100), 100, 100, AnglePosition::TopLeft);
        assert_eq!(triangle.points[0], (100, 100).into(), "topleft - main angle");
        assert_eq!(triangle.points[1], (200, 100).into(), "topleft - same y");
        assert_eq!(triangle.points[2], (100, 200).into(), "topleft - same x");

        let triangle = Triangle::right_angle((100, 100), 100, 100, AnglePosition::BottomRight);
        assert_eq!(triangle.points[0], (100, 100).into(), "bottomright - main angle");
        assert_eq!(triangle.points[1], (0, 100).into(), "bottomright - same y");
        assert_eq!(triangle.points[2], (100, 0).into(), "bottomright - same x");

        let triangle = Triangle::right_angle((100, 100), 100, 100, AnglePosition::TopRight);
        assert_eq!(triangle.points[0], (100, 100).into(), "topright - main angle");
        assert_eq!(triangle.points[1], (0, 100).into(), "topright - same y");
        assert_eq!(triangle.points[2], (100, 200).into(), "topright - same x");

        let triangle = Triangle::right_angle((100, 100), 100, 100, AnglePosition::BottomLeft);
        assert_eq!(triangle.points[0], (100, 100).into(), "bottomleft - main angle");
        assert_eq!(triangle.points[1], (200, 100).into(), "bottomleft - same y");
        assert_eq!(triangle.points[2], (100, 0).into(), "bottomleft - same x");
    }
}