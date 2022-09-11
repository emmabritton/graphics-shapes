use crate::Coord;

/// Scale `points` (move them towards or away) around the `center` by `factor`
///
/// The resulting points distance will be points[x].distance(center) * factor but at the same angle
pub fn scale_points(center: Coord, points: &[Coord], factor: f32) -> Vec<Coord> {
    let mut output = vec![];
    for point in points {
        let angle = center.angle_to(*point);
        let dist = center.distance(*point) as f32 * factor;
        output.push(Coord::from_angle(
            center,
            dist.round().max(0.0) as usize,
            angle,
        ));
    }
    output
}

/// Rotate `points` around the `center` by `degrees`
///
/// The resulting points at the same distance but at +degrees angle
pub fn rotate_points(center: Coord, points: &[Coord], degrees: isize) -> Vec<Coord> {
    let mut output = vec![];
    for point in points {
        let starting_angle = center.angle_to(*point);
        let dist = center.distance(*point);
        output.push(Coord::from_angle(center, dist, starting_angle + degrees));
    }
    output
}

#[cfg(test)]
mod test {
    use crate::{Coord, rotate_points};

    #[test]
    fn one_point_rotation() {
        let center = Coord::new(20,20);
        let initial = Coord::new(30,20);
        let no_rotation = rotate_points(center, &[initial], 0);
        let quarter_rotation = rotate_points(center, &[initial], 90);
        let half_rotation = rotate_points(center, &[initial], 180);
        let three_quarter_rotation = rotate_points(center, &[initial], 270);
        let full_rotation = rotate_points(center, &[initial], 360);
        assert_eq!(no_rotation, vec![Coord::new(30,20)]);
        assert_eq!(quarter_rotation, vec![Coord::new(20,30)]);
        assert_eq!(half_rotation, vec![Coord::new(10,20)]);
        assert_eq!(three_quarter_rotation, vec![Coord::new(20,10)]);
        assert_eq!(full_rotation, vec![Coord::new(30,20)]);
        let one_degree = rotate_points(center, &[initial], 1);
        assert_eq!(one_degree, vec![Coord::new(30,20)]);
        let eighth_degree = rotate_points(center, &[initial], 45);
        assert_eq!(eighth_degree, vec![Coord::new(27,27)]);
    }
}