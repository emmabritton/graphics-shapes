use crate::circle::Circle;
use crate::ellipse::Ellipse;
use crate::line::LineType;
use crate::polygon::Polygon;
use crate::prelude::{ContainsShape, Line, Rect, Triangle};

impl ContainsShape for Line {
    fn contains_rect(&self, rect: &Rect) -> bool {
        match self.line_type() {
            LineType::Point => {
                self.start() == rect.top_left() && self.start() == rect.bottom_right()
            }
            LineType::Vertical | LineType::Horizontal => {
                (self.start() == rect.top_left() && self.end() == rect.bottom_right())
                    || (self.end() == rect.top_left() && self.start() == rect.bottom_right())
            }
            LineType::Angled => false,
        }
    }

    fn contains_circle(&self, _: &Circle) -> bool {
        false
    }

    fn contains_line(&self, line: &Line) -> bool {
        (self.start() == line.start() || self.start() == line.end())
            && (self.end() == line.start() || self.end() == line.end())
    }

    fn contains_triangle(&self, _: &Triangle) -> bool {
        false
    }

    fn contains_ellipse(&self, _: &Ellipse) -> bool {
        false
    }

    fn contains_polygon(&self, _: &Polygon) -> bool {
        false
    }
}
