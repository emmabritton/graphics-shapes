use crate::Coord;

pub trait Lerp {
    /// calculate the point at `percent` between `self` and `end`
    ///
    /// # Usage
    /// ```rust
    ///# use graphics_shapes::lerp::Lerp;
    /// assert_eq!(10.lerp(20, 0.5), 15);
    /// assert_eq!(10.lerp(20, 0.1), 11);
    /// ```
    ///
    /// internally the values are cast as f32 and rounded before being returned
    #[must_use]
    fn lerp(self, end: Self, percent: f32) -> Self;

    /// calculate the percent for `point` between `self` and `end`
    ///
    /// # Usage
    /// ```rust
    ///# use graphics_shapes::lerp::Lerp;
    /// assert_eq!(10.inv_lerp(20, 15), 0.5);
    /// assert_eq!(10.inv_lerp(20, 11), 0.1);
    /// ```
    ///
    /// internally the values are cast as f32 and rounded before being returned
    #[must_use]
    fn inv_lerp(self, end: Self, point: Self) -> f32;
}

/// This method has to be separate and named differently because
/// f32::lerp already exists but is unstable
///
/// see [f32::lerp]
///
/// calculate the point at `percent` between `self` and `end`
///
/// # Usage
/// ```rust
///# use graphics_shapes::lerp::flerp;
/// assert_eq!(flerp(10.0, 20.0, 0.5), 15.0);
/// assert_eq!(flerp(10.0, 20.0, 0.1), 11.0);
/// ```
#[inline]
#[must_use]
pub fn flerp(start: f32, end: f32, percent: f32) -> f32 {
    start + ((end - start) * percent)
}

/// calculate the percent for `point` between `self` and `end`
///
/// # Usage
/// ```rust
///# use graphics_shapes::lerp::inv_flerp;
/// assert_eq!(inv_flerp(10.0, 20.0, 15.0), 0.5);
/// assert_eq!(inv_flerp(10.0, 20.0, 11.0), 0.1);
/// ```
#[inline]
#[must_use]
pub fn inv_flerp(start: f32, end: f32, point: f32) -> f32 {
    if point == start {
        return 0.0;
    }
    if point == end {
        return 1.0;
    }
    (point - start) / (end - start)
}

macro_rules! impl_lerp {
    ($num_type: ty) => {
        impl Lerp for $num_type {
            #[inline]
            fn lerp(self, end: $num_type, percent: f32) -> $num_type {
                let start = self as f32;
                let end = end as f32;
                flerp(start, end, percent).round() as $num_type
            }

            #[inline]
            fn inv_lerp(self, end: $num_type, point: $num_type) -> f32 {
                let start = self as f32;
                let end = end as f32;
                let point = point as f32;
                inv_flerp(start, end, point)
            }
        }
    };
}

impl_lerp!(u8);
impl_lerp!(i8);
impl_lerp!(u16);
impl_lerp!(i16);
impl_lerp!(u32);
impl_lerp!(i32);
impl_lerp!(u64);
impl_lerp!(i64);
impl_lerp!(u128);
impl_lerp!(i128);
impl_lerp!(usize);
impl_lerp!(isize);

impl Lerp for Coord {
    #[inline]
    fn lerp(self, end: Coord, percent: f32) -> Coord {
        Coord {
            x: self.x.lerp(end.x, percent),
            y: self.y.lerp(end.y, percent),
        }
    }

    #[inline]
    fn inv_lerp(self, end: Coord, point: Coord) -> f32 {
        (self.x.inv_lerp(end.x, point.x) + self.y.inv_lerp(end.y, point.y)) / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn isize_simple() {
        assert_eq!(0_isize.lerp(10, 0.), 0);
        assert_eq!(0_isize.lerp(10, 0.5), 5);
        assert_eq!(0_isize.lerp(10, 1.), 10);
        assert_eq!(0_isize.lerp(10, 0.2), 2);

        assert_eq!(5_isize.lerp(10, 0.), 5);
        assert_eq!(5_isize.lerp(10, 1.), 10);

        assert_eq!(785_isize.lerp(787, 0.), 785);
        assert_eq!(785_isize.lerp(787, 0.5), 786);
        assert_eq!(785_isize.lerp(787, 1.), 787);

        assert_eq!(21_isize.lerp(21, 0.), 21);
        assert_eq!(21_isize.lerp(21, 0.5), 21);
        assert_eq!(21_isize.lerp(21, 1.), 21);

        assert_eq!(10_isize.lerp(1, 1.), 1);
        assert_eq!(10_isize.lerp(1, 0.5), 6);
        assert_eq!(10_isize.lerp(1, 0.), 10);

        assert_eq!((-5_isize).lerp(5, 1.), 5);
        assert_eq!((-5_isize).lerp(5, 0.5), 0);
        assert_eq!((-5_isize).lerp(5, 0.), -5);

        assert_eq!(5_isize.lerp(-5, 1.), -5);
        assert_eq!(5_isize.lerp(-5, 0.5), 0);
        assert_eq!(5_isize.lerp(-5, 0.), 5);

        assert_eq!(0_isize.inv_lerp(10, 0), 0.);
        assert_eq!(0_isize.inv_lerp(10, 5), 0.5);
        assert_eq!(0_isize.inv_lerp(10, 10), 1.);
        assert_eq!(0_isize.inv_lerp(10, 2), 0.2);

        assert_eq!(5_isize.inv_lerp(10, 5), 0.);
        assert_eq!(5_isize.inv_lerp(10, 10), 1.);

        assert_eq!(785_isize.inv_lerp(787, 785), 0.);
        assert_eq!(785_isize.inv_lerp(787, 786), 0.5);
        assert_eq!(785_isize.inv_lerp(787, 787), 1.);

        assert_eq!(21_isize.inv_lerp(21, 21), 0.);

        assert_eq!(10_isize.inv_lerp(1, 1), 1.);
        assert_eq!(10_isize.inv_lerp(1, 6), 0.44444445);
        assert_eq!(10_isize.inv_lerp(1, 10), 0.);

        assert_eq!((-5_isize).inv_lerp(5, 5), 1.);
        assert_eq!((-5_isize).inv_lerp(5, 0), 0.5);
        assert_eq!((-5_isize).inv_lerp(5, -5), 0.);

        assert_eq!(5_isize.inv_lerp(-5, -5), 1.);
        assert_eq!(5_isize.inv_lerp(-5, 0), 0.5);
        assert_eq!(5_isize.inv_lerp(-5, 5), 0.);
    }

    #[test]
    fn point_simple() {
        let start1 = Coord { x: 0, y: 0 };
        let end1 = Coord { x: 10, y: 10 };

        let start2 = Coord { x: -1, y: -1 };
        let end2 = Coord { x: 1, y: 1 };

        let start3 = Coord { x: 1, y: -1 };
        let end3 = Coord { x: -1, y: 1 };

        assert_eq!(start1.lerp(end1, 0.), Coord { x: 0, y: 0 });
        assert_eq!(start1.lerp(end1, 0.5), Coord { x: 5, y: 5 });
        assert_eq!(start1.lerp(end1, 1.), Coord { x: 10, y: 10 });

        assert_eq!(end1.lerp(start1, 0.), Coord { x: 10, y: 10 });
        assert_eq!(end1.lerp(start1, 0.5), Coord { x: 5, y: 5 });
        assert_eq!(end1.lerp(start1, 1.), Coord { x: 0, y: 0 });

        assert_eq!(start2.lerp(end2, 0.), Coord { x: -1, y: -1 });
        assert_eq!(start2.lerp(end2, 0.5), Coord { x: 0, y: 0 });
        assert_eq!(start2.lerp(end2, 1.), Coord { x: 1, y: 1 });

        assert_eq!(end2.lerp(start2, 0.), Coord { x: 1, y: 1 });
        assert_eq!(end2.lerp(start2, 0.5), Coord { x: 0, y: 0 });
        assert_eq!(end2.lerp(start2, 1.), Coord { x: -1, y: -1 });

        assert_eq!(start3.lerp(end3, 0.), Coord { x: 1, y: -1 });
        assert_eq!(start3.lerp(end3, 0.5), Coord { x: 0, y: 0 });
        assert_eq!(start3.lerp(end3, 1.), Coord { x: -1, y: 1 });

        assert_eq!(end3.lerp(start3, 0.), Coord { x: -1, y: 1 });
        assert_eq!(end3.lerp(start3, 0.5), Coord { x: 0, y: 0 });
        assert_eq!(end3.lerp(start3, 1.), Coord { x: 1, y: -1 });

        assert_eq!(start1.lerp(end1, 2.), Coord { x: 20, y: 20 });
        assert_eq!(start1.lerp(end1, -1.), Coord { x: -10, y: -10 });

        assert_eq!(start1.inv_lerp(end1, Coord { x: 0, y: 0 }), 0.);
        assert_eq!(start1.inv_lerp(end1, Coord { x: 5, y: 5 }), 0.5);
        assert_eq!(start1.inv_lerp(end1, Coord { x: 10, y: 10 }), 1.);

        assert_eq!(end1.inv_lerp(start1, Coord { x: 10, y: 10 }), 0.);
        assert_eq!(end1.inv_lerp(start1, Coord { x: 5, y: 5 }), 0.5);
        assert_eq!(end1.inv_lerp(start1, Coord { x: 0, y: 0 }), 1.);

        assert_eq!(start2.inv_lerp(end2, Coord { x: -1, y: -1 }), 0.);
        assert_eq!(start2.inv_lerp(end2, Coord { x: 0, y: 0 }), 0.5);
        assert_eq!(start2.inv_lerp(end2, Coord { x: 1, y: 1 }), 1.);

        assert_eq!(end2.inv_lerp(start2, Coord { x: 1, y: 1 }), 0.);
        assert_eq!(end2.inv_lerp(start2, Coord { x: 0, y: 0 }), 0.5);
        assert_eq!(end2.inv_lerp(start2, Coord { x: -1, y: -1 }), 1.);

        assert_eq!(start3.inv_lerp(end3, Coord { x: 1, y: -1 }), 0.);
        assert_eq!(start3.inv_lerp(end3, Coord { x: 0, y: 0 }), 0.5);
        assert_eq!(start3.inv_lerp(end3, Coord { x: -1, y: 1 }), 1.);

        assert_eq!(end3.inv_lerp(start3, Coord { x: -1, y: 1 }), 0.);
        assert_eq!(end3.inv_lerp(start3, Coord { x: 0, y: 0 }), 0.5);
        assert_eq!(end3.inv_lerp(start3, Coord { x: 1, y: -1 }), 1.);

        assert_eq!(start1.inv_lerp(end1, Coord { x: 20, y: 20 }), 2.);
        assert_eq!(start1.inv_lerp(end1, Coord { x: -10, y: -10 }), -1.);
    }
}
