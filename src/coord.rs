use crate::coord;
#[cfg(feature = "mint")]
use mint::Point2;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represents a 2D point
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    #[inline]
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// 0 is the top of the circle
    #[must_use]
    pub fn from_angle<P: Into<Coord>>(center: P, distance: usize, degrees: isize) -> Self {
        let center = center.into();
        let distance = distance as f32;
        let rads = (degrees as f32 - 90.0).to_radians();
        let x = (distance * rads.cos()).round() as isize;
        let y = (distance * rads.sin()).round() as isize;
        coord!(center.x + x, center.y + y)
    }
}

impl Coord {
    /// Distance between self and rhs
    #[must_use]
    pub fn distance<P: Into<Coord>>(self, rhs: P) -> usize {
        let rhs = rhs.into();
        let x = (rhs.x - self.x) as f64;
        let y = (rhs.y - self.y) as f64;
        x.hypot(y).round().abs() as usize
    }

    #[must_use]
    pub fn are_collinear<P1: Into<Coord>, P2: Into<Coord>>(self, b: P1, c: P2) -> bool {
        let b = b.into();
        let c = c.into();
        (b.x - self.x) * (c.y - self.y) == (c.x - self.x) * (b.y - self.y)
    }

    #[must_use]
    pub fn is_between<P1: Into<Coord>, P2: Into<Coord>>(self, a: P1, b: P2) -> bool {
        let a = a.into();
        let b = b.into();
        ((a.x <= self.x && self.x <= b.x) && (a.y <= self.y && self.y <= b.y))
            || ((b.x <= self.x && self.x <= a.x) && (b.y <= self.y && self.y <= a.y))
    }

    /// Point midway in between self and rhs
    /// Use lerp for other positions
    #[must_use]
    pub fn mid_point<P: Into<Coord>>(self, rhs: P) -> Coord {
        let rhs = rhs.into();
        let x = (self.x + rhs.x) / 2;
        let y = (self.y + rhs.y) / 2;
        coord!(x, y)
    }

    /// Angle in degrees from self to rhs
    /// 0 is the top of the circle
    #[must_use]
    pub fn angle_to<P: Into<Coord>>(self, rhs: P) -> isize {
        let rhs = rhs.into();
        let x = (rhs.x - self.x) as f32;
        let y = (rhs.y - self.y) as f32;
        y.atan2(x).to_degrees().round() as isize + 90
    }

    #[must_use]
    pub fn cross_product<P: Into<Coord>>(self, rhs: P) -> isize {
        let rhs = rhs.into();
        self.x * rhs.y - self.y * rhs.x
    }

    #[must_use]
    pub fn dot_product<P: Into<Coord>>(self, rhs: P) -> isize {
        let rhs = rhs.into();
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    #[must_use]
    pub const fn perpendicular(self) -> Coord {
        Coord::new(self.y, -self.x)
    }

    #[inline]
    #[must_use]
    pub const fn abs(self) -> Coord {
        Coord {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<P: Into<Coord>> Add<P> for Coord {
    type Output = Coord;

    #[inline]
    #[must_use]
    fn add(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Coord {
    type Output = Coord;

    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<P: Into<Coord>> Sub<P> for Coord {
    type Output = Coord;

    #[inline]
    #[must_use]
    fn sub(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<P: Into<Coord>> Mul<P> for Coord {
    type Output = Coord;

    #[inline]
    #[must_use]
    fn mul(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Coord {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[cfg(feature = "mint")]
impl From<Point2<isize>> for Coord {
    #[inline]
    #[must_use]
    fn from(point: Point2<isize>) -> Self {
        Coord {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(feature = "mint")]
impl From<&Point2<isize>> for Coord {
    #[inline]
    #[must_use]
    fn from(point: &Point2<isize>) -> Self {
        Coord {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(feature = "mint")]
impl From<Coord> for Point2<isize> {
    #[inline]
    #[must_use]
    fn from(coord: Coord) -> Self {
        Point2 {
            x: coord.x,
            y: coord.y,
        }
    }
}

#[cfg(feature = "mint")]
impl From<&Coord> for Point2<isize> {
    #[inline]
    #[must_use]
    fn from(coord: &Coord) -> Self {
        Point2 {
            x: coord.x,
            y: coord.y,
        }
    }
}

macro_rules! impl_from_num {
    ($num_type:ty) => {
        impl From<($num_type, $num_type)> for Coord {
            #[inline]
            #[must_use]
            fn from(nums: ($num_type, $num_type)) -> Coord {
                Coord {
                    x: nums.0 as isize,
                    y: nums.1 as isize,
                }
            }
        }

        impl From<&($num_type, $num_type)> for Coord {
            #[inline]
            #[must_use]
            fn from(nums: &($num_type, $num_type)) -> Coord {
                Coord {
                    x: nums.0 as isize,
                    y: nums.1 as isize,
                }
            }
        }

        impl Add<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            #[must_use]
            fn add(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x + rhs as isize,
                    y: self.y + rhs as isize,
                }
            }
        }

        impl Sub<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            #[must_use]
            fn sub(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x - rhs as isize,
                    y: self.y - rhs as isize,
                }
            }
        }
    };
}

macro_rules! int_mul {
    ($num_type:ty) => {
        impl Mul<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            #[must_use]
            fn mul(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x * rhs as isize,
                    y: self.y * rhs as isize,
                }
            }
        }

        impl Div<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            #[must_use]
            fn div(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x / rhs as isize,
                    y: self.y / rhs as isize,
                }
            }
        }
    };
}

macro_rules! float_mul {
    ($num_type:ty) => {
        impl Mul<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            #[must_use]
            fn mul(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: ((self.x as $num_type) * rhs).ceil() as isize,
                    y: ((self.y as $num_type) * rhs).ceil() as isize,
                }
            }
        }

        impl Div<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            #[must_use]
            fn div(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: ((self.x as $num_type) / rhs).ceil() as isize,
                    y: ((self.y as $num_type) / rhs).ceil() as isize,
                }
            }
        }
    };
}

impl From<&Coord> for Coord {
    fn from(value: &Coord) -> Self {
        *value
    }
}

#[macro_export]
macro_rules! coord {
    ($lhs: expr, $rhs: expr,) => {
        $crate::coord::Coord::from(($lhs, $rhs))
    };
    ($lhs: expr, $rhs: expr) => {
        $crate::coord::Coord::from(($lhs, $rhs))
    };
    ($pair: expr) => {
        $crate::coord::Coord::from($pair)
    };
}

impl_from_num!(u8);
impl_from_num!(i8);
impl_from_num!(u16);
impl_from_num!(i16);
impl_from_num!(u32);
impl_from_num!(i32);
impl_from_num!(u64);
impl_from_num!(i64);
impl_from_num!(u128);
impl_from_num!(i128);
impl_from_num!(usize);
impl_from_num!(isize);
impl_from_num!(f32);
impl_from_num!(f64);
int_mul!(u8);
int_mul!(u16);
int_mul!(u32);
int_mul!(u64);
int_mul!(u128);
int_mul!(i8);
int_mul!(i16);
int_mul!(i32);
int_mul!(i64);
int_mul!(i128);
int_mul!(usize);
int_mul!(isize);
float_mul!(f32);
float_mul!(f64);

/// Create a list of [Coord]s
///
/// # Example
/// ```rust
///# use graphics_shapes::coord::Coord;
///# use graphics_shapes::{coord, coord_vec};
/// let list = coord_vec![(5.0,6.0), (1_usize,2), coord!(-4,1)];
/// assert_eq!(list, vec![coord!(5,6), coord!(1,2), coord!(-4,1)]);
/// ```
#[macro_export]
macro_rules! coord_vec {
    () => (
        Vec::<$crate::coord::Coord>::new()
    );
    ($first:expr) => (
        vec![$crate::coord::Coord::from($first)]
    );
    ($first:expr, $($vararg:expr),+) => (
        vec![$crate::coord::Coord::from($first), $($crate::coord::Coord::from($vararg)),*]
    );
}

#[cfg(test)]
mod test {
    mod list {
        #[test]
        fn empty() {
            let list = coord_vec![];
            assert_eq!(list, vec![]);
        }

        #[test]
        fn one() {
            let list = coord_vec![coord!(1, 1)];
            assert_eq!(list, vec![coord!(1, 1)]);

            let list = coord_vec![(4.0, 2.0)];
            assert_eq!(list, vec![coord!(4, 2)]);
        }

        #[test]
        fn many() {
            let list = coord_vec![(-1_isize, 1), (9_usize, 4)];
            assert_eq!(list, vec![coord!(-1, 1), coord!(9, 4)]);

            let list = coord_vec![(-1, 1), (9_usize, 4), (5, 6), (9, 8)];
            assert_eq!(
                list,
                vec![coord!(-1, 1), coord!(9, 4), coord!(5, 6), coord!(9, 8)]
            );
        }
    }

    mod point_on_circle {
        use crate::Coord;

        #[test]
        fn zero_dist() {
            let center = coord!(100, 100);

            for i in 0..400 {
                assert_eq!(Coord::from_angle(center, 0, i), center, "rot: {i}");
            }
        }

        #[test]
        fn twenty_dist_positive_degrees() {
            let center = coord!(100, 100);

            let zero_degree = Coord::from_angle(center, 20, 0);
            let ninety_degree = Coord::from_angle(center, 20, 90);
            let oneeighty_degree = Coord::from_angle(center, 20, 180);
            let twoseventy_degree = Coord::from_angle(center, 20, 270);
            let seventwenty_degree = Coord::from_angle(center, 20, 720);

            assert_eq!(zero_degree, (100, 80).into());
            assert_eq!(ninety_degree, (120, 100).into());
            assert_eq!(oneeighty_degree, (100, 120).into());
            assert_eq!(twoseventy_degree, (80, 100).into());
            assert_eq!(seventwenty_degree, (100, 80).into());
        }

        #[test]
        fn twenty_dist_negative_degrees() {
            let center = coord!(100, 100);

            let zero_degree = Coord::from_angle(center, 20, -0);
            let ninety_degree = Coord::from_angle(center, 20, -90);
            let oneeighty_degree = Coord::from_angle(center, 20, -180);
            let twoseventy_degree = Coord::from_angle(center, 20, -270);
            let seventwenty_degree = Coord::from_angle(center, 20, -720);

            assert_eq!(zero_degree, (100, 80).into());
            assert_eq!(ninety_degree, (80, 100).into());
            assert_eq!(oneeighty_degree, (100, 120).into());
            assert_eq!(twoseventy_degree, (120, 100).into());
            assert_eq!(seventwenty_degree, (100, 80).into());
        }

        #[test]
        fn eighths() {
            let center = coord!(100, 100);

            let degree_45 = Coord::from_angle(center, 20, 45);
            let degree_135 = Coord::from_angle(center, 20, 135);
            let degree_225 = Coord::from_angle(center, 20, 225);
            let degree_315 = Coord::from_angle(center, 20, 315);

            assert_eq!(degree_45, (114, 86).into());
            assert_eq!(degree_135, (114, 114).into());
            assert_eq!(degree_225, (86, 114).into());
            assert_eq!(degree_315, (86, 86).into());
        }
    }

    mod methods {
        #[test]
        fn dist() {
            let start = coord!(10, 10);

            assert_eq!(start.distance((20, 10)), 10);
            assert_eq!(start.distance((0, 10)), 10);
            assert_eq!(start.distance((10, 0)), 10);
            assert_eq!(start.distance((10, 20)), 10);
            assert_eq!(start.distance((20, 20)), 14);
            assert_eq!(start.distance((0, 0)), 14);
            assert_eq!(start.distance((20, 0)), 14);
            assert_eq!(start.distance((0, 20)), 14);
        }

        #[test]
        fn angle() {
            let center = coord!(20, 20);

            assert_eq!(center.angle_to((30, 20)), 90);
            assert_eq!(center.angle_to((20, 30)), 180);
            assert_eq!(center.angle_to((10, 20)), 270);
            assert_eq!(center.angle_to((20, 10)), 0);
        }

        #[test]
        fn mid_points() {
            let start = coord!(10, 10);

            assert_eq!(start.mid_point((20, 10)), (15, 10).into());
            assert_eq!(start.mid_point((0, 10)), (5, 10).into());
            assert_eq!(start.mid_point((10, 0)), (10, 5).into());
        }
    }

    mod ops {
        use crate::Coord;
        use std::ops::{Add, Mul, Neg, Sub};

        #[test]
        fn simple() {
            assert_eq!(coord!(1, 1).add((1, 1)), (2, 2).into());
            assert_eq!(coord!(1, 1).sub((1, 1)), (0, 0).into());
            assert_eq!(coord!(1, 1).mul((1, 1)), (1, 1).into());
            assert_eq!(coord!(1, 1).abs(), (1, 1).into());
            assert_eq!(coord!(1, 1).neg(), (-1, -1).into());

            assert_eq!(coord!(2, 8).add((12, 63)), (14, 71).into());
            assert_eq!(coord!(3, 7).sub((13, 24)), (-10, -17).into());
            assert_eq!(coord!(4, 6).mul((11, 21)), (44, 126).into());
            assert_eq!(coord!(5, -5).abs(), (5, 5).into());
            assert_eq!(coord!(6, -4).neg(), (-6, 4).into());

            assert_eq!(coord!(4, 8).mul(0.5), (2, 4).into());
            assert_eq!(coord!(4, 8).mul(Coord::from((0.5, 0.5))), (0, 0).into());
            assert_eq!(coord!(4, 8).mul(Coord::from((0.4, 0.4))), (0, 0).into());
        }
    }
}
