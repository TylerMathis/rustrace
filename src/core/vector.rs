use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::float::Float;
use num_traits::Num;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// A geometric three-dimensional vector
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3<T> = Vec3<T>;

pub type Vec3f = Vec3<f64>;
pub type Vec3i = Vec3<i32>;

pub type Point3f = Vec3f;
pub type Point3i = Vec3i;

pub type Color3f = Vec3f;
pub type Color3i = Vec3i;

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn into<U>(self) -> Vec3<U>
    where
        T: Into<U>,
    {
        Vec3::new(self.x.into(), self.y.into(), self.z.into())
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Hadamard product
impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Inverse Hadamard product
impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length_sq(&self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec3<T> {
    pub fn length(&self) -> T {
        self.length_sq().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vec3<T> {
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vec = Vec3::new(1, 2, 3);
        assert_eq!(vec, Vec3 { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn add() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 + vec2, Vec3::new(4, 4, 4));
    }

    #[test]
    fn sub() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 - vec2, Vec3::new(-2, 0, 2));
    }

    #[test]
    fn hadamard() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 * vec2, Vec3::new(3, 4, 3));
    }

    #[test]
    fn inverse_hadamard() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 / vec2, Vec3::new(0, 1, 3));
    }

    #[test]
    fn dot() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1.dot(&vec2), 10);
    }

    #[test]
    fn cross() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1.cross(&vec2), Vec3::new(-4, 8, -4));
    }

    #[test]
    fn lengths() {
        let vec_i = Vec3::new(1, 2, 3);
        assert_eq!(vec_i.length_sq(), 14);

        let vec_f = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec_f.length(), vec_f.length_sq().sqrt());
    }
}

///////////////
// END TESTS //
///////////////
