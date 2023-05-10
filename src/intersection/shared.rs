use crate::intersection::IntersectsShape;
use crate::line::LineType;
use crate::prelude::{Circle, Coord, Ellipse, Line, Polygon, Rect, Shape, Triangle};

pub fn line_circle(line: &Line, circle: &Circle) -> bool {
    let top_left = line.top_left() - circle.center();
    let bottom_right = line.bottom_right() - circle.center();
    let dx = bottom_right.x - top_left.x;
    let dy = bottom_right.y - top_left.y;
    let dr = ((dx * dx + dy * dy) as f64).sqrt();
    let d = (top_left.x * bottom_right.y - bottom_right.x * top_left.y) as f64;
    let r = circle.radius() as f64;

    let result = r * r * dr * dr - d * d;
    result >= 0.0
}

pub fn line_triangle(line: &Line, triangle: &Triangle) -> bool {
    for tri_line in triangle.as_lines() {
        if tri_line.intersects_line(line) {
            return true;
        }
    }
    false
}

pub fn line_rect(line: &Line, rect: &Rect) -> bool {
    for rect_line in rect.as_lines() {
        if rect_line.intersects_line(line) {
            return true;
        }
    }
    false
}

pub fn line_polygon(line: &Line, poly: &Polygon) -> bool {
    for poly_line in poly.as_lines() {
        if poly_line.intersects_line(line) {
            return true;
        }
    }
    false
}

pub fn lines_lines(lhs: &[Line], rhs: &[Line]) -> bool {
    for l in lhs {
        for r in rhs {
            if l.intersects_line(r) {
                return true;
            }
        }
    }
    false
}

pub fn line_ellipse(line: &Line, ellipse: &Ellipse) -> bool {
    let mut rect = ellipse.as_rect();
    if rect.width() == 1 || rect.height() == 1 {
        return line.contains(rect.top_left());
    }
    if line.line_type() == LineType::Point {
        return ellipse.contains(line.start());
    }
    let dist = rect.center();
    rect = rect.move_center_to(coord!(0, 0));
    let line = line.translate_by(-dist);

    let major = (rect.width() / 2) as isize;
    let minor = (rect.height() / 2) as isize;

    let start = line.start();
    let end = line.end();
    let a = (end.x - start.x) * (end.x - start.x) / major / major
        + (end.y - start.y) * (end.y - start.y) / minor / minor;
    let b = 2 * start.x * (end.x - start.x) / major / major
        + 2 * start.y * (end.y - start.y) / minor / minor;
    let c = start.x * start.x / major / major + start.y * start.y / minor / minor - 1;

    let discriminant = b * b - 4 * a * c;
    discriminant >= 0
}

pub fn rect_circle(rect: &Rect, circle: &Circle) -> bool {
    let dist = rect.center().distance(circle.center()) - circle.radius() / 2;
    let max = rect.width().max(rect.height()) / 2;
    dist < max
}

pub fn rect_ellipse(rect: &Rect, ellipse: &Ellipse) -> bool {
    for line in &rect.as_lines() {
        if line.intersects_ellipse(ellipse) {
            return true;
        }
    }
    false
}

pub fn polygon_ellipse(polygon: &Polygon, ellipse: &Ellipse) -> bool {
    for line in &polygon.as_lines() {
        if line.intersects_ellipse(ellipse) {
            return true;
        }
    }
    false
}

pub fn polygon_circle(polygon: &Polygon, circle: &Circle) -> bool {
    for line in polygon.as_lines() {
        if line.intersects_circle(circle) {
            return true;
        }
    }
    false
}

pub fn triangle_ellipse(triangle: &Triangle, ellipse: &Ellipse) -> bool {
    for line in &triangle.as_lines() {
        if line.intersects_ellipse(ellipse) {
            return true;
        }
    }
    false
}

pub fn triangle_circle(triangle: &Triangle, circle: &Circle) -> bool {
    for line in triangle.as_lines() {
        if line.intersects_circle(circle) {
            return true;
        }
    }
    false
}
