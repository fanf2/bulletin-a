use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Euclid(i32);

impl From<i32> for Euclid {
    fn from(i: i32) -> Euclid {
        Euclid(i)
    }
}

impl From<Euclid> for i32 {
    fn from(e: Euclid) -> i32 {
        e.0
    }
}

impl Neg for Euclid {
    type Output = Euclid;
    fn neg(self) -> Euclid {
        Euclid(-self.0)
    }
}

impl Add<i32> for Euclid {
    type Output = Euclid;
    fn add(self, other: i32) -> Euclid {
        Euclid(self.0 + other)
    }
}

impl Sub<i32> for Euclid {
    type Output = Euclid;
    fn sub(self, other: i32) -> Euclid {
        Euclid(self.0 - other)
    }
}

impl Mul<i32> for Euclid {
    type Output = Euclid;
    fn mul(self, other: i32) -> Euclid {
        Euclid(self.0 * other)
    }
}

impl Div<i32> for Euclid {
    type Output = Euclid;
    fn div(self, other: i32) -> Euclid {
        Euclid(self.0.div_euclid(other))
    }
}

impl Rem<i32> for Euclid {
    type Output = Euclid;
    fn rem(self, other: i32) -> Euclid {
        Euclid(self.0.rem_euclid(other))
    }
}

impl Add<Euclid> for Euclid {
    type Output = Euclid;
    fn add(self, other: Euclid) -> Euclid {
        self + other.0
    }
}

impl Sub<Euclid> for Euclid {
    type Output = Euclid;
    fn sub(self, other: Euclid) -> Euclid {
        self - other.0
    }
}

impl Mul<Euclid> for Euclid {
    type Output = Euclid;
    fn mul(self, other: Euclid) -> Euclid {
        self * other.0
    }
}

impl Div<Euclid> for Euclid {
    type Output = Euclid;
    fn div(self, other: Euclid) -> Euclid {
        self / other.0
    }
}

impl Rem<Euclid> for Euclid {
    type Output = Euclid;
    fn rem(self, other: Euclid) -> Euclid {
        self % other.0
    }
}

impl<T> AddAssign<T> for Euclid
where
    Euclid: Add<T, Output = Euclid>,
{
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}

impl<T> SubAssign<T> for Euclid
where
    Euclid: Sub<T, Output = Euclid>,
{
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

impl<T> MulAssign<T> for Euclid
where
    Euclid: Mul<T, Output = Euclid>,
{
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> DivAssign<T> for Euclid
where
    Euclid: Div<T, Output = Euclid>,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other;
    }
}

impl<T> RemAssign<T> for Euclid
where
    Euclid: Rem<T, Output = Euclid>,
{
    fn rem_assign(&mut self, other: T) {
        *self = *self % other;
    }
}
