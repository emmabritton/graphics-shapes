use crate::line::LineType;
use crate::prelude::*;

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

pub fn line_ellipse(line: &Line, ellipse: &Ellipse) -> bool {
    let temp_line = line.rotate_around(-ellipse.angle(), ellipse.center());

    if ellipse.width() == 1 || ellipse.height() == 1 {
        return temp_line.contains(ellipse.top_left());
    }
    if temp_line.line_type() == LineType::Point {
        return ellipse.contains(temp_line.start());
    }

    let line = line
        .translate_by(-ellipse.center())
        .rotate_around(-ellipse.angle(), Coord::default());
    let x1 = line.start().x as f64;
    let y1 = line.start().y as f64;
    let x2 = line.end().x as f64;
    let y2 = line.end().y as f64;

    let w = ellipse.width() as f64 / 2.0;
    let h = ellipse.height() as f64 / 2.0;

    let a = (x2 - x1).powi(2) / w / w + (y2 - y1).powi(2) / h / h;
    let b = 2.0 * x1 * (x2 - x1) / w / w + 2.0 * y1 * (y2 - y1) / h / h;
    let c = x1 * x1 / w / w + y1 * y1 / h / h - 1.0;
    let d = b * b - 4.0 * a * c;

    if d == 0.0 {
        let t = -b / 2.0 / a;
        (0.0..=1.0).contains(&t)
    } else if d > 0.0 {
        let sqrt = d.sqrt();
        let t1 = (-b + sqrt) / 2.0 / a;
        let t2 = (-b - sqrt) / 2.0 / a;
        (0.0..=1.0).contains(&t1) || (0.0..=1.0).contains(&t2)
    } else {
        false
    }
}

pub fn rect_circle(rect: &Rect, circle: &Circle) -> bool {
    for line in &rect.as_lines() {
        if line.intersects_circle(circle) {
            return true;
        }
    }
    false
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

pub fn iterate(
    x: f64,
    y: f64,
    mut c0x: f64,
    mut c0y: f64,
    mut c2x: f64,
    mut c2y: f64,
    rr: f64,
) -> bool {
    let mut inner_polygon_coef = [0.0; 11];
    let mut outer_polygon_coef = [0.0; 11];
    let zero_acos = 0.0_f64.acos() * 2.0;
    for t in 0..=10 {
        let num_nodes = (4 << t) as f64;
        inner_polygon_coef[t] = 0.5 / (2.0 * zero_acos / num_nodes).cos();
        outer_polygon_coef[t] =
            0.5 / ((zero_acos / num_nodes).cos() * (zero_acos) / num_nodes).cos();
    }
    for t in 1..=10 {
        let c1x = (c0x + c2x) * inner_polygon_coef[t];
        let c1y = (c0y + c2y) * inner_polygon_coef[t];
        let tx = x - c1x;
        let ty = y - c1y;
        if tx * tx + ty * ty <= rr {
            return true;
        }
        let t2x = c2x - c1x;
        let t2y = c2y - c1y;
        if tx * t2x + ty * t2y >= 0.0
            && tx * t2x + ty * t2y <= t2x * t2x + t2y * t2y
            && (ty * t2x - tx * t2y >= 0.0
                || rr * (t2x * t2x + t2y * t2y) >= (ty * t2x - tx * t2y) * (ty * t2x - tx * t2y))
        {
            return true;
        }
        let t0x = c0x - c1x;
        let t0y = c0y - c1y;
        if tx * t0x + ty * t0y >= 0.0
            && tx * t0x + ty * t0y <= t0x * t0x + t0y * t0y
            && (ty * t0x - tx * t0y <= 0.0
                || rr * (t0x * t0x + t0y * t0y) >= (ty * t0x - tx * t0y) * (ty * t0x - tx * t0y))
        {
            return true;
        }
        let c3x = (c0x + c1x) * outer_polygon_coef[t];
        let c3y = (c0y + c1y) * outer_polygon_coef[t];
        if (c3x - x) * (c3x - x) + (c3y - y) * (c3y - y) < rr {
            c2x = c1x;
            c2y = c1y;
            continue;
        }
        let c4x = c1x - c3x + c1x;
        let c4y = c1y - c3y + c1y;
        if (c4x - x) * (c4x - x) + (c4y - y) * (c4y - y) < rr {
            c0x = c1x;
            c0y = c1y;
            continue;
        }
        let t3x = c3x - c1x;
        let t3y = c3y - c1y;
        if ty * t3x - tx * t3y <= 0.0
            || rr * (t3x * t3x + t3y * t3y) > (ty * t3x - tx * t3y) * (ty * t3x - tx * t3y)
        {
            if tx * t3x + ty * t3y > 0.0 {
                if (tx * t3x + ty * t3y).abs() <= t3x * t3x + t3y * t3y
                    || (x - c3x) * (c0x - c3x) + (y - c3y) * (c0y - c3y) >= 0.0
                {
                    c2x = c1x;
                    c2y = c1y;
                    continue;
                }
            } else if -(tx * t3x + ty * t3y) <= t3x * t3x + t3y * t3y
                || (x - c4x) * (c2x - c4x) + (y - c4y) * (c2y - c4y) >= 0.0
            {
                c0x = c1x;
                c0y = c1y;
                continue;
            }
        }
        return false;
    }
    false
}

pub fn ellipse_circle(ellipse: &Ellipse, circle: &Circle) -> bool {
    let x0 = ellipse.center().x as f64;
    let y0 = ellipse.center().y as f64;
    let w = ellipse.width() as f64;
    let h = ellipse.width() as f64;
    let x1 = circle.center().x as f64;
    let y1 = circle.center().y as f64;
    let r = circle.radius() as f64;

    let x = (x1 - x0).abs();
    let y = (y1 - y0).abs();

    if x * x + (h - y) * (h - y) <= r * r
        || (w - x) * (w - x) + y * y <= r * r
        || x * h + y * w <= w * h
        || ((x * h + y * w - w * h) * (x * h + y * w - w * h) <= r * r * (w * w + h * h)
            && x * w - y * h >= -h * h
            && x * w - y * h <= w * w)
    {
        true
    } else if (x - w) * (x - w) + (y - h) * (y - h) <= r * r
        || (x <= w && y - r <= h)
        || (y <= h && x - r <= w)
    {
        iterate(x, y, w, 0.0, 0.0, h, r * r)
    } else {
        false
    }
}
