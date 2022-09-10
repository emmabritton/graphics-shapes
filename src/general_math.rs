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
