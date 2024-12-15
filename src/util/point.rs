use std::ops::{ Add, Mul, Neg, Sub };

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point(pub isize, pub isize);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl From<Point> for (isize, isize) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}
