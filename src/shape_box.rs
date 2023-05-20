use crate::prelude::*;

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeBox {
    Line(Line),
    Rect(Rect),
    Triangle(Triangle),
    Circle(Circle),
    Ellipse(Ellipse),
    Polygon(Polygon),
}

macro_rules! per_shape_0 {
    ($shape_box: expr, $method: path) => {
        match $shape_box {
            ShapeBox::Line(shape) => $method(shape),
            ShapeBox::Rect(shape) => $method(shape),
            ShapeBox::Circle(shape) => $method(shape),
            ShapeBox::Triangle(shape) => $method(shape),
            ShapeBox::Ellipse(shape) => $method(shape),
            ShapeBox::Polygon(shape) => $method(shape),
        }
    };
}

macro_rules! per_shape_1 {
    ($shape_box: expr, $method: path, $param1: expr) => {
        match $shape_box {
            ShapeBox::Line(shape) => $method(shape, $param1),
            ShapeBox::Rect(shape) => $method(shape, $param1),
            ShapeBox::Circle(shape) => $method(shape, $param1),
            ShapeBox::Triangle(shape) => $method(shape, $param1),
            ShapeBox::Ellipse(shape) => $method(shape, $param1),
            ShapeBox::Polygon(shape) => $method(shape, $param1),
        }
    };
}

impl Shape for ShapeBox {
    fn from_points(_: &[Coord]) -> Self
    where
        Self: Sized,
    {
        unimplemented!("ShapeBox can't be constructed from points")
    }

    fn rebuild(&self, points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        match self {
            ShapeBox::Line(_) => ShapeBox::Line(Line::from_points(points)),
            ShapeBox::Rect(_) => ShapeBox::Rect(Rect::from_points(points)),
            ShapeBox::Triangle(_) => ShapeBox::Triangle(Triangle::from_points(points)),
            ShapeBox::Circle(_) => ShapeBox::Circle(Circle::from_points(points)),
            ShapeBox::Ellipse(_) => ShapeBox::Ellipse(Ellipse::from_points(points)),
            ShapeBox::Polygon(_) => ShapeBox::Polygon(Polygon::from_points(points)),
        }
    }

    fn contains(&self, point: Coord) -> bool {
        per_shape_1!(self, Shape::contains, point)
    }

    fn points(&self) -> Vec<Coord> {
        per_shape_0!(self, Shape::points)
    }

    fn center(&self) -> Coord {
        per_shape_0!(self, Shape::center)
    }

    fn outline_pixels(&self) -> Vec<Coord> {
        per_shape_0!(self, Shape::outline_pixels)
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        per_shape_0!(self, Shape::filled_pixels)
    }

    /// Same as `clone()`
    fn to_shape_box(&self) -> ShapeBox {
        self.clone()
    }
}

impl IntersectsShape for ShapeBox {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        per_shape_1!(self, IntersectsShape::intersects_rect, rect)
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        per_shape_1!(self, IntersectsShape::intersects_circle, circle)
    }

    fn intersects_line(&self, line: &Line) -> bool {
        per_shape_1!(self, IntersectsShape::intersects_line, line)
    }

    fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        per_shape_1!(self, IntersectsShape::intersects_triangle, triangle)
    }

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        per_shape_1!(self, IntersectsShape::intersects_ellipse, ellipse)
    }

    fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        per_shape_1!(self, IntersectsShape::intersects_polygon, polygon)
    }
}

impl ContainsShape for ShapeBox {
    fn contains_rect(&self, rect: &Rect) -> bool {
        per_shape_1!(self, ContainsShape::contains_rect, rect)
    }

    fn contains_circle(&self, circle: &Circle) -> bool {
        per_shape_1!(self, ContainsShape::contains_circle, circle)
    }

    fn contains_line(&self, line: &Line) -> bool {
        per_shape_1!(self, ContainsShape::contains_line, line)
    }

    fn contains_triangle(&self, triangle: &Triangle) -> bool {
        per_shape_1!(self, ContainsShape::contains_triangle, triangle)
    }

    fn contains_ellipse(&self, ellipse: &Ellipse) -> bool {
        per_shape_1!(self, ContainsShape::contains_ellipse, ellipse)
    }

    fn contains_polygon(&self, polygon: &Polygon) -> bool {
        per_shape_1!(self, ContainsShape::contains_polygon, polygon)
    }
}

impl IntersectsContains for ShapeBox {}

macro_rules! shapebox_shape {
    ($shape: ty, $variant: path) => {
        impl From<$shape> for ShapeBox {
            fn from(value: $shape) -> Self {
                $variant(value)
            }
        }
    };
}

shapebox_shape!(Line, ShapeBox::Line);
shapebox_shape!(Rect, ShapeBox::Rect);
shapebox_shape!(Triangle, ShapeBox::Triangle);
shapebox_shape!(Circle, ShapeBox::Circle);
shapebox_shape!(Ellipse, ShapeBox::Ellipse);
shapebox_shape!(Polygon, ShapeBox::Polygon);

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::shape_box::ShapeBox;

    #[test]
    fn basic() {
        let rect_box = ShapeBox::from(Rect::new((10, 10), (30, 30)));
        assert!(rect_box.contains(coord!(12, 15)));
        assert!(rect_box.intersects_line(&Line::new((18, 0), (0, 30))));
    }
}
