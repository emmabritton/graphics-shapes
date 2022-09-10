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
