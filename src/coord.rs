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
        let rads = degrees as f32 * 3.1459 / 180.0;
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

    pub fn perpendicular(self) -> Coord {
        Coord::new(self.y, -self.x)
    }

    pub fn abs(self) -> Coord {
        Coord {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<P: Into<Coord>> Add<P> for Coord {
    type Output = Coord;

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
            fn from(nums: ($num_type, $num_type)) -> Coord {
                Coord {
                    x: nums.0 as isize,
                    y: nums.1 as isize,
                }
            }
        }

        impl Add<$num_type> for Coord {
            type Output = Coord;

            fn add(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x + rhs as isize,
                    y: self.y + rhs as isize,
                }
            }
        }

        impl Sub<$num_type> for Coord {
            type Output = Coord;

            fn sub(self, rhs: $num_type) -> Self::Output {
                Coord {
                    x: self.x - rhs as isize,
                    y: self.y - rhs as isize,
                }
            }
        }

        impl Mul<$num_type> for Coord {
            type Output = Coord;

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
