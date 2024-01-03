use crate::new_hash_set;
use crate::prelude::*;
use crate::shape_box::ShapeBox;
use fnv::FnvHashSet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TriangleAngleType {
    Acute,
    Right,
    Obtuse,
    Equiangular,
    Other,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TriangleSideType {
    Isosceles,
    Scalene,
    Equilateral,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Triangle {
    points: [Coord; 3],
    angles: [isize; 3],
    angle_type: TriangleAngleType,
    side_type: TriangleSideType,
    center: Coord,
}

impl IntersectsContains for Triangle {}

impl Triangle {
    #[must_use]
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
            center: coord!(0, 0),
        };
        triangle.center = coord!(triangle.left(), triangle.top())
            .mid_point(coord!(triangle.right(), triangle.bottom()));
        triangle
    }
}

impl Triangle {
    #[inline]
    #[must_use]
    pub fn angles(&self) -> [isize; 3] {
        self.angles
    }

    #[inline]
    #[must_use]
    pub fn angle_type(&self) -> &TriangleAngleType {
        &self.angle_type
    }

    #[inline]
    #[must_use]
    pub fn side_type(&self) -> &TriangleSideType {
        &self.side_type
    }
}

impl Shape for Triangle {
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Triangle::new(points[0], points[1], points[2])
    }

    fn rebuild(&self, points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Triangle::from_points(points)
    }

    fn contains(&self, point: Coord) -> bool {
        let p1 = coord!(
            self.points[1].x - self.points[0].x,
            self.points[1].y - self.points[0].y,
        );
        let p2 = coord!(
            self.points[2].x - self.points[0].x,
            self.points[2].y - self.points[0].y,
        );
        let q = coord!(point.x - self.points[0].x, point.y - self.points[0].y);

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

    fn outline_pixels(&self) -> Vec<Coord> {
        self.as_lines()
            .iter()
            .flat_map(|line| line.outline_pixels())
            .collect()
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        let mut output = new_hash_set();
        let mut sorted_points = self.points.to_vec();
        sorted_points.sort_by_key(|c| c.y);
        let points = [
            (sorted_points[0].x as f32, sorted_points[0].y as f32),
            (sorted_points[1].x as f32, sorted_points[1].y as f32),
            (sorted_points[2].x as f32, sorted_points[2].y as f32),
        ];
        if points[1].1 == points[2].1 {
            draw_flat_bottom(&mut output, points);
        } else if points[0].1 == points[1].1 {
            draw_flat_top(&mut output, points);
        } else {
            let p = (
                points[0].0
                    + ((points[1].1 - points[0].1) / (points[2].1 - points[0].1))
                        * (points[2].0 - points[0].0),
                points[1].1,
            );
            draw_flat_bottom(&mut output, [points[0], points[1], p]);
            draw_flat_top(&mut output, [points[1], p, points[2]]);
        }
        output.into_iter().collect()
    }

    fn to_shape_box(&self) -> ShapeBox {
        ShapeBox::Triangle(self.clone())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnglePosition {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Top,
    Right,
    Bottom,
    Left,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FlatSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl Triangle {
    #[must_use]
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    #[must_use]
    pub fn as_lines(&self) -> [Line; 3] {
        let points = self.points();
        [
            Line::new(points[0], points[1]),
            Line::new(points[1], points[2]),
            Line::new(points[2], points[0]),
        ]
    }

    /// Create an Isosceles Right Angle Triangle
    #[must_use]
    pub fn right_angle<P: Into<Coord>>(
        angle_coord: P,
        size: usize,
        angle_position: AnglePosition,
    ) -> Triangle {
        let size = size as isize;
        let point = angle_coord.into();
        let left = point.x - size;
        let right = point.x + size;
        let top = point.y - size;
        let bottom = point.y + size;
        let half_size = size / 2;
        match angle_position {
            AnglePosition::TopLeft => Triangle::new(point, (right, point.y), (point.x, bottom)),
            AnglePosition::BottomRight => Triangle::new(point, (left, point.y), (point.x, top)),
            AnglePosition::BottomLeft => Triangle::new(point, (right, point.y), (point.x, top)),
            AnglePosition::TopRight => Triangle::new(point, (left, point.y), (point.x, bottom)),
            AnglePosition::Top => Triangle::new(
                point,
                point + (-half_size, half_size),
                point + (half_size, half_size),
            ),
            AnglePosition::Right => Triangle::new(
                point,
                point - (half_size, half_size),
                point + (-half_size, half_size),
            ),
            AnglePosition::Bottom => Triangle::new(
                point,
                point - (half_size, half_size),
                point + (half_size, -half_size),
            ),
            AnglePosition::Left => Triangle::new(
                point,
                point + (half_size, -half_size),
                point + (half_size, half_size),
            ),
        }
    }

    /// Create an equilateral triangle with width and height of [size] around [center]
    /// The top left would be (center.x - size / 2, center.y + size / 2) and bottom right (center.x + size / 2, center.y + size / 2)
    #[must_use]
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

pub fn draw_flat_bottom(output: &mut FnvHashSet<Coord>, points: [(f32, f32); 3]) {
    let slope1 = (points[1].0 - points[0].0) / (points[1].1 - points[0].1);
    let slope2 = (points[2].0 - points[0].0) / (points[2].1 - points[0].1);
    let mut x1 = points[0].0;
    let mut x2 = points[0].0;
    for y in (points[0].1 as usize)..(points[1].1 as usize) {
        let start = x1.min(x2) as isize;
        let end = x1.max(x2) as isize + 1;
        let line_points = Line::new((start, y as isize), (end, y as isize)).outline_pixels();
        for point in line_points {
            output.insert(point);
        }
        x1 += slope1;
        x2 += slope2;
    }
}

pub fn draw_flat_top(output: &mut FnvHashSet<Coord>, points: [(f32, f32); 3]) {
    let slope1 = (points[2].0 - points[0].0) / (points[2].1 - points[0].1);
    let slope2 = (points[2].0 - points[1].0) / (points[2].1 - points[1].1);
    let mut x1 = points[2].0;
    let mut x2 = points[2].0;
    for y in ((points[0].1 as usize)..(points[2].1 as usize)).rev() {
        let start = x1.min(x2) as usize;
        let end = x1.max(x2) as usize + 1;
        let line_points = Line::new((start, y), (end, y)).outline_pixels();
        for point in line_points {
            output.insert(point);
        }
        x1 -= slope1;
        x2 -= slope2;
    }
}

#[cfg(test)]
mod test {
    use crate::triangle::{AnglePosition, FlatSide, Triangle};
    use crate::Shape;

    #[test]
    fn right_angle_triangles() {
        let triangle = Triangle::right_angle((100, 100), 100, AnglePosition::TopLeft);
        assert_eq!(
            triangle.points[0],
            (100, 100).into(),
            "topleft - main angle"
        );
        assert_eq!(triangle.points[1], (200, 100).into(), "topleft - same y");
        assert_eq!(triangle.points[2], (100, 200).into(), "topleft - same x");

        let triangle = Triangle::right_angle((100, 100), 100, AnglePosition::BottomRight);
        assert_eq!(
            triangle.points[0],
            (100, 100).into(),
            "bottomright - main angle"
        );
        assert_eq!(triangle.points[1], (0, 100).into(), "bottomright - same y");
        assert_eq!(triangle.points[2], (100, 0).into(), "bottomright - same x");

        let triangle = Triangle::right_angle((100, 100), 100, AnglePosition::TopRight);
        assert_eq!(
            triangle.points[0],
            (100, 100).into(),
            "topright - main angle"
        );
        assert_eq!(triangle.points[1], (0, 100).into(), "topright - same y");
        assert_eq!(triangle.points[2], (100, 200).into(), "topright - same x");

        let triangle = Triangle::right_angle((100, 100), 100, AnglePosition::BottomLeft);
        assert_eq!(
            triangle.points[0],
            (100, 100).into(),
            "bottomleft - main angle"
        );
        assert_eq!(triangle.points[1], (200, 100).into(), "bottomleft - same y");
        assert_eq!(triangle.points[2], (100, 0).into(), "bottomleft - same x");
    }

    #[test]
    fn check_moving() {
        let triangle = Triangle::equilateral((50, 50), 10, FlatSide::Left);
        assert_eq!(triangle.center, coord!(50, 50));
        assert_eq!(triangle.points[0], coord!(45, 45));
        let moved = triangle.move_to(coord!(30, 30));
        assert_eq!(moved.points[0], coord!(30, 30));

        let moved = triangle.move_center_to(coord!(130, 230));
        assert_eq!(moved.center, coord!(130, 230));
        assert_eq!(moved.points[0], coord!(125, 225));
    }
}
