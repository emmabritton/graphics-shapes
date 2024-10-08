use crate::prelude::*;
use crate::shape_box::ShapeBox;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Shape with any number of points/line
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    points: Vec<Coord>,
    fpoints: Vec<(f32, f32)>,
    is_regular: bool,
    center: Coord,
    is_convex: bool,
}

impl IntersectsContains for Polygon {}

impl Polygon {
    #[must_use]
    pub fn new<'a, P: Into<Coord>>(points: &'a [P]) -> Self
    where
        Coord: From<&'a P>,
    {
        let points: Vec<Coord> = points.iter().map(|p| p.into()).collect();
        let fpoints = points.iter().map(|p| (p.x as f32, p.y as f32)).collect();
        let is_convex = is_convex(&points);
        let mut poly = Self {
            points: points.clone(),
            fpoints,
            center: Coord::default(),
            is_regular: false,
            is_convex,
        };
        poly.center = poly.top_left().mid_point(poly.bottom_right());
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

    /// Returns true if all sides are the same length
    #[inline]
    #[must_use]
    pub fn is_regular(&self) -> bool {
        self.is_regular
    }

    /// Returns the closest corner point to the center
    #[must_use]
    pub fn point_closest_to_center(&self) -> Coord {
        let mut list = self.points.clone();
        list.sort_by_key(|p| p.distance(self.center));
        list[0]
    }

    /// Returns the furthest corner point to the center
    #[must_use]
    pub fn point_farthest_from_center(&self) -> Coord {
        let mut list = self.points.clone();
        list.sort_by_key(|p| p.distance(self.center));
        *list.last().unwrap()
    }

    /// Returns true if any sides point inwards
    #[inline]
    #[must_use]
    pub fn is_convex(&self) -> bool {
        self.is_convex
    }
}

impl Shape for Polygon {
    fn from_points(points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Polygon::new(points)
    }

    fn rebuild(&self, points: &[Coord]) -> Self
    where
        Self: Sized,
    {
        Polygon::from_points(points)
    }

    fn contains(&self, point: Coord) -> bool {
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

    fn outline_pixels(&self) -> Vec<Coord> {
        self.as_lines()
            .iter()
            .flat_map(|line| line.outline_pixels())
            .collect()
    }

    fn filled_pixels(&self) -> Vec<Coord> {
        let mut output = vec![];
        let poly: Vec<(f32, f32)> = self
            .points
            .iter()
            .map(|c| (c.x as f32, c.y as f32))
            .collect();
        let y_start = self.top();
        let y_end = self.bottom();
        for y in y_start..y_end {
            let mut node = vec![];
            let mut node_count = 0;
            let y = y as f32;
            let mut j = poly.len() - 1;
            for i in 0..poly.len() {
                if poly[i].1 < y && poly[j].1 >= y || poly[j].1 < y && poly[i].1 >= y {
                    node.push(
                        poly[i].0
                            + (y - poly[i].1) / (poly[j].1 - poly[i].1) * (poly[j].0 - poly[i].0),
                    );
                    node_count += 1;
                }
                j = i;
            }
            let mut i = 0;
            if node_count > 0 {
                while i < (node_count - 1) {
                    if node[i] > node[i + 1] {
                        node.swap(i, i + 1);
                        i = i.saturating_sub(1);
                    } else {
                        i += 1;
                    }
                }
                for i in (0..node_count - 1).step_by(2) {
                    for x in (node[i] as isize)..(node[i + 1] as isize) {
                        output.push(coord!(x + 1, y as isize));
                    }
                }
            }
        }
        output
    }

    fn to_shape_box(&self) -> ShapeBox {
        ShapeBox::Polygon(self.clone())
    }
}

impl Polygon {
    /// Creates a circle using the point closest to the center
    #[must_use]
    pub fn as_inner_circle(&self) -> Circle {
        Circle::from_points(&[self.center, self.point_closest_to_center()])
    }

    /// Creates a circle using the point farthest to the center
    #[must_use]
    pub fn as_outer_circle(&self) -> Circle {
        Circle::from_points(&[self.center, self.point_farthest_from_center()])
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
            Some(Circle::from_points(&[self.center, self.points[0]]))
        } else {
            None
        }
    }

    /// Creates rect that contains the whole shape
    #[must_use]
    pub fn as_rect(&self) -> Rect {
        Rect::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    #[must_use]
    pub fn as_lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        let poly = self.points.clone();
        for i in 0..poly.len() - 1 {
            lines.push(Line::new(poly[i], poly[i + 1]));
        }
        lines.push(Line::new(poly[poly.len() - 1], poly[0]));
        lines
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

fn is_convex(points: &[Coord]) -> bool {
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
