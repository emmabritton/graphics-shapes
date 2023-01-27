use crate::circle::Circle;
use crate::rect::Rect;
use crate::triangle::Triangle;
use crate::{Coord, Shape};
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    points: Vec<Coord>,
    fpoints: Vec<(f32, f32)>,
    is_regular: bool,
    center: Coord,
    is_convex: bool,
}

impl Polygon {
    #[must_use]
    pub fn new<P: Into<Coord>>(points: Vec<P>) -> Self {
        let points: Vec<Coord> = points.into_iter().map(|p| p.into()).collect();
        let fpoints = points.iter().map(|p| (p.x as f32, p.y as f32)).collect();
        let is_convex = is_convex(&points);
        let mut poly = Self {
            points: points.clone(),
            fpoints,
            center: Coord::default(),
            is_regular: false,
            is_convex,
        };
        poly.center =
            Coord::new(poly.left(), poly.top()).mid_point(Coord::new(poly.right(), poly.bottom()));
        let dists: Vec<usize> = points.iter().map(|p| p.distance(poly.center)).collect();
        poly.is_regular = dists.iter().all(|dist| dist == &dists[0]);
        poly
    }
}

impl Polygon {
    #[inline]
    #[must_use]
    pub fn fpoints(&self) -> &Vec<(f32, f32)> {
        &self.fpoints
    }

    #[inline]
    #[must_use]
    pub fn is_regular(&self) -> bool {
        self.is_regular
    }

    #[must_use]
    pub fn point_closest_to_center(&self) -> Coord {
        let mut list = self.points.clone();
        list.sort_by_key(|p| p.distance(self.center));
        list[0]
    }

    #[must_use]
    pub fn point_farthest_from_center(&self) -> Coord {
        let mut list = self.points.clone();
        list.sort_by_key(|p| p.distance(self.center));
        *list.last().unwrap()
    }

    #[inline]
    #[must_use]
    pub fn is_convex(&self) -> bool {
        self.is_convex
    }
}

impl Shape for Polygon {
    fn from_points(points: Vec<Coord>) -> Self
    where
        Self: Sized,
    {
        Polygon::new(points)
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let point = point.into();
        let mut j = self.fpoints.len() - 1;
        let mut odd_number_of_nodes = false;
        let fpoint = (point.x as f32, point.y as f32);

        for i in 0..self.fpoints.len() {
            if (self.fpoints[i].1 < fpoint.1 && self.fpoints[j].1 >= fpoint.1
                || self.fpoints[j].1 < fpoint.1 && self.fpoints[i].1 >= fpoint.1)
                && (self.fpoints[i].0 <= fpoint.0 || self.fpoints[j].0 <= fpoint.0)
            {
                odd_number_of_nodes ^= self.fpoints[i].0
                    + (fpoint.1 - self.fpoints[i].1) / (self.fpoints[j].1 - self.fpoints[i].1)
                        * (self.fpoints[j].0 - self.fpoints[i].0)
                    < fpoint.0;
            }
            j = i;
        }

        odd_number_of_nodes
    }

    fn points(&self) -> Vec<Coord> {
        self.points.clone()
    }

    #[inline]
    fn center(&self) -> Coord {
        self.center
    }
}

impl Polygon {
    /// Creates a circle using the point closest to the center
    #[must_use]
    pub fn as_inner_circle(&self) -> Circle {
        Circle::from_points(vec![self.center, self.point_closest_to_center()])
    }

    /// Creates a circle using the point farthest to the center
    #[must_use]
    pub fn as_outer_circle(&self) -> Circle {
        Circle::from_points(vec![self.center, self.point_farthest_from_center()])
    }

    /// Creates a circle using the average point distance from the center
    #[must_use]
    pub fn as_avg_circle(&self) -> Circle {
        let total: usize = self.points.iter().map(|p| p.distance(self.center)).sum();
        let radius = total / self.points.len();
        Circle::new(self.center, radius)
    }

    /// If the polygon is regular then it returns a circle from center to the first point
    #[must_use]
    pub fn as_circle(&self) -> Option<Circle> {
        if self.is_regular {
            Some(Circle::from_points(vec![self.center, self.points[0]]))
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    /// Cuts shape into triangles, triangles will be from the center to the edge
    /// This only works on convex polygons
    #[must_use]
    pub fn as_triangles(&self) -> Option<Vec<Triangle>> {
        if !self.is_convex {
            return None;
        }
        let mut output = vec![];
        for coords in self.points.windows(2) {
            output.push(Triangle::new(coords[0], coords[1], self.center));
        }
        output.push(Triangle::new(
            *self.points.last().unwrap(),
            self.points[0],
            self.center,
        ));

        Some(output)
    }
}

fn is_convex(points: &Vec<Coord>) -> bool {
    let mut prev = 0;
    for i in 0..points.len() {
        let product = (points[(i + 1) % points.len()] - points[i])
            .cross_product(points[(i + 2) % points.len()] - points[i]);
        if product != 0 {
            if product * prev < 0 {
                return false;
            } else {
                prev = product;
            }
        }
    }
    true
}
