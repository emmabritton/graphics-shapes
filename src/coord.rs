#[cfg(feature = "mint")]
use mint::Point2;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Neg, Sub};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    #[inline]
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_angle<P: Into<Coord>>(center: P, distance: usize, degrees: isize) -> Self {
        let center = center.into();
        let distance = distance as f32;
        let rads = (degrees as f32).to_radians();
        let x = (distance * rads.cos()).round() as isize;
        let y = (distance * rads.sin()).round() as isize;
        Coord::new(center.x + x, center.y + y)
    }
}

impl Coord {
    /// Distance between self and rhs
    pub fn distance<P: Into<Coord>>(self, rhs: P) -> usize {
        let rhs = rhs.into();
        let x = (rhs.x - self.x) as f32;
        let y = (rhs.y - self.y) as f32;
        x.hypot(y).round().abs() as usize
    }

    /// Point midway in between self and rhs
    /// Use lerp for other positions
    pub fn mid_point<P: Into<Coord>>(self, rhs: P) -> Coord {
        let rhs = rhs.into();
        let x = (self.x + rhs.x) / 2;
        let y = (self.y + rhs.y) / 2;
        Coord::new(x, y)
    }

    /// Angle in degrees from self to rhs
    pub fn angle_to<P: Into<Coord>>(self, rhs: P) -> isize {
        let rhs = rhs.into();
        let x = (rhs.x - self.x) as f32;
        let y = (rhs.y - self.y) as f32;
        y.atan2(x).to_degrees().round() as isize
    }

    pub fn cross_product<P: Into<Coord>>(self, rhs: P) -> isize {
        let rhs = rhs.into();
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn dot_product<P: Into<Coord>>(self, rhs: P) -> isize {
        let rhs = rhs.into();
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn perpendicular(self) -> Coord {
        Coord::new(self.y, -self.x)
    }

    #[inline]
    pub fn abs(self) -> Coord {
        Coord {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<P: Into<Coord>> Add<P> for Coord {
    type Output = Coord;

    #[inline]
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
    fn neg(self) -> Self::Output {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<P: Into<Coord>> Sub<P> for Coord {
    type Output = Coord;

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
    fn from(point: Point2<isize>) -> Self {
        Coord {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(feature = "mint")]
impl From<Coord> for Point2<isize> {
    fn from(coord: Coord) -> Self {
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
            fn from(nums: ($num_type, $num_type)) -> Coord {
                Coord {
                    x: nums.0 as isize,
                    y: nums.1 as isize,
                }
            }
        }

        impl Add<$num_type> for Coord {
            type Output = Coord;

            #[inline]
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
            fn sub(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x - rhs as isize,
                    y: self.y - rhs as isize,
                }
            }
        }

        impl Mul<$num_type> for Coord {
            type Output = Coord;

            #[inline]
            fn mul(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x * rhs as isize,
                    y: self.y * rhs as isize,
                }
            }
        }
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

#[cfg(test)]
mod test {
    mod point_on_circle {
        use crate::Coord;

        #[test]
        fn zero_dist() {
            let center = Coord::new(100, 100);

            for i in 0..400 {
                assert_eq!(Coord::from_angle(center, 0, i), center, "rot: {i}");
            }
        }

        #[test]
        fn twenty_dist_positive_degrees() {
            let center = Coord::new(100, 100);

            let zero_degree = Coord::from_angle(center, 20, 0);
            let ninety_degree = Coord::from_angle(center, 20, 90);
            let oneeighty_degree = Coord::from_angle(center, 20, 180);
            let twoseventy_degree = Coord::from_angle(center, 20, 270);
            let seventwenty_degree = Coord::from_angle(center, 20, 720);

            assert_eq!(zero_degree, (120, 100).into());
            assert_eq!(ninety_degree, (100, 120).into());
            assert_eq!(oneeighty_degree, (80, 100).into());
            assert_eq!(twoseventy_degree, (100, 80).into());
            assert_eq!(seventwenty_degree, (120, 100).into());
        }

        #[test]
        fn twenty_dist_negative_degrees() {
            let center = Coord::new(100, 100);

            let zero_degree = Coord::from_angle(center, 20, -0);
            let ninety_degree = Coord::from_angle(center, 20, -90);
            let oneeighty_degree = Coord::from_angle(center, 20, -180);
            let twoseventy_degree = Coord::from_angle(center, 20, -270);
            let seventwenty_degree = Coord::from_angle(center, 20, -720);

            assert_eq!(zero_degree, (120, 100).into());
            assert_eq!(ninety_degree, (100, 80).into());
            assert_eq!(oneeighty_degree, (80, 100).into());
            assert_eq!(twoseventy_degree, (100, 120).into());
            assert_eq!(seventwenty_degree, (120, 100).into());
        }

        #[test]
        fn eighths() {
            let center = Coord::new(100, 100);

            let degree_45 = Coord::from_angle(center, 20, 45);
            let degree_135 = Coord::from_angle(center, 20, 135);
            let degree_225 = Coord::from_angle(center, 20, 225);
            let degree_315 = Coord::from_angle(center, 20, 315);

            assert_eq!(degree_45, (114, 114).into());
            assert_eq!(degree_135, (86, 114).into());
            assert_eq!(degree_225, (86, 86).into());
            assert_eq!(degree_315, (114, 86).into());
        }
    }

    mod methods {
        use crate::Coord;

        #[test]
        fn dist() {
            let start = Coord::new(10, 10);

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
            let center = Coord::new(20,20);

            assert_eq!(center.angle_to((30,20)), 0);
            assert_eq!(center.angle_to((20,30)), 90);
            assert_eq!(center.angle_to((10,20)), 180);
            assert_eq!(center.angle_to((20,10)), -90);
        }

        #[test]
        fn mid_points() {
            let start = Coord::new(10, 10);

            assert_eq!(start.mid_point((20, 10)), (15, 10).into());
            assert_eq!(start.mid_point((0, 10)), (5, 10).into());
            assert_eq!(start.mid_point((10, 0)), (10, 5).into());
        }
    }

    mod ops {
        use std::ops::{Add, Mul, Neg, Sub};
        use crate::Coord;

        #[test]
        fn simple() {
            assert_eq!(Coord::new(1, 1).add((1, 1)), (2, 2).into());
            assert_eq!(Coord::new(1, 1).sub((1, 1)), (0, 0).into());
            assert_eq!(Coord::new(1, 1).mul((1, 1)), (1, 1).into());
            assert_eq!(Coord::new(1, 1).abs(), (1, 1).into());
            assert_eq!(Coord::new(1, 1).neg(), (-1, -1).into());

            assert_eq!(Coord::new(2, 8).add((12, 63)), (14, 71).into());
            assert_eq!(Coord::new(3, 7).sub((13, 24)), (-10, -17).into());
            assert_eq!(Coord::new(4, 6).mul((11, 21)), (44, 126).into());
            assert_eq!(Coord::new(5, -5).abs(), (5, 5).into());
            assert_eq!(Coord::new(6, -4).neg(), (-6, 4).into());
        }
    }
}