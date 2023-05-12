use crate::intersection::IntersectsShape;
use crate::prelude::{Circle, Line, Polygon, Rect, Shape, Triangle};

pub fn line_circle(line: &Line, circle: &Circle) -> bool {
    let mut ax = line.start().x as f64;
    let mut ay = line.start().y as f64;
    let mut bx = line.end().x as f64;
    let mut by = line.end().y as f64;
    let cx = circle.center().x as f64;
    let cy = circle.center().y as f64;
    let r = circle.radius() as f64;

    ax -= cx;
    ay -= cy;
    bx -= cx;
    by -= cy;
    let a = ((bx - ax).powi(2)) + ((by - ay).powi(2));
    let b = 2.0 * (ax * (bx - ax) + ay * (by - ay));
    let c = ax.powi(2) + ay.powi(2) - r.powi(2);
    let disc = b.powi(2) - 4.0 * a * c;
    if disc <= 0.0 {
        return false;
    }
    let sqrtdisc = disc.sqrt();
    let t1 = (-b + sqrtdisc) / (2.0 * a);
    let t2 = (-b - sqrtdisc) / (2.0 * a);
    (0.0 < t1 && t1 < 1.0) || (0.0 < t2 && t2 < 1.0)
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

// pub fn line_ellipse(line: &Line, ellipse: &Ellipse) -> bool {
//     let mut rect = ellipse.as_rect();
//     if rect.width() == 1 || rect.height() == 1 {
//         return line.contains(rect.top_left());
//     }
//     if line.line_type() == LineType::Point {
//         return ellipse.contains(line.start());
//     }
//     let dist = rect.center();
//     rect = rect.move_center_to(coord!(0, 0));
//     let line = line.translate_by(-dist);
//
//     let major = (rect.width() / 2) as isize;
//     let minor = (rect.height() / 2) as isize;
//
//     let start = line.start();
//     let end = line.end();
//     let a = (end.x - start.x) * (end.x - start.x) / major / major
//         + (end.y - start.y) * (end.y - start.y) / minor / minor;
//     let b = 2 * start.x * (end.x - start.x) / major / major
//         + 2 * start.y * (end.y - start.y) / minor / minor;
//     let c = start.x * start.x / major / major + start.y * start.y / minor / minor - 1;
//
//     let discriminant = b * b - 4 * a * c;
//     discriminant >= 0
// }

pub fn rect_circle(rect: &Rect, circle: &Circle) -> bool {
    for line in &rect.as_lines() {
        if line.intersects_circle(circle) {
            return true;
        }
    }
    false
}

// pub fn rect_ellipse(rect: &Rect, ellipse: &Ellipse) -> bool {
//     for line in &rect.as_lines() {
//         if line.intersects_ellipse(ellipse) {
//             return true;
//         }
//     }
//     false
// }
//
// pub fn polygon_ellipse(polygon: &Polygon, ellipse: &Ellipse) -> bool {
//     for line in &polygon.as_lines() {
//         if line.intersects_ellipse(ellipse) {
//             return true;
//         }
//     }
//     false
// }

pub fn polygon_circle(polygon: &Polygon, circle: &Circle) -> bool {
    println!("Checking against {circle:?}");
    for line in polygon.as_lines() {
        println!("Comparing {line:?}");
        if line.intersects_circle(circle) {
            return true;
        }
    }
    false
}

// pub fn triangle_ellipse(triangle: &Triangle, ellipse: &Ellipse) -> bool {
//     for line in &triangle.as_lines() {
//         if line.intersects_ellipse(ellipse) {
//             return true;
//         }
//     }
//     false
// }

pub fn triangle_circle(triangle: &Triangle, circle: &Circle) -> bool {
    for line in triangle.as_lines() {
        if line.intersects_circle(circle) {
            return true;
        }
    }
    false
}
