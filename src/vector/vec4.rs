use super::super::matrix::Mat;
use num::Float;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default)]
pub struct Vec4<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Float> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    pub fn fill(n: T) -> Self {
        Self {
            x: n,
            y: n,
            z: n,
            w: n,
        }
    }

    pub fn mag(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl<T: Float> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T: Float> Sub<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T: Float> Neg for Vec4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: Float> AddAssign for Vec4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl<T: Float> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

impl<T: Float, const C: usize> Mul<Mat<T, 1, C>> for Vec4<T> {
    type Output = Mat<T, 4, C>;

    fn mul(self, rhs: Mat<T, 1, C>) -> Self::Output {
        Mat::from(self) * rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vec = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
        assert_eq!(vec.w, 4.0);
    }

    #[test]
    fn zero() {
        let vec = Vec4::zero();
        assert_eq!(vec, Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn fill() {
        let vec = Vec4::fill(1.0);
        assert_eq!(vec, Vec4::new(1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn add() {
        let vec_a = Vec4::new(5.0, 5.0, 5.0, 5.0);
        let vec_b = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vec_a + vec_b, Vec4::new(6.0, 7.0, 8.0, 9.0));
    }

    #[test]
    fn sub() {
        let vec_a = Vec4::new(5.0, 5.0, 5.0, 5.0);
        let vec_b = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vec_a - vec_b, Vec4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn dot() {
        let vec_a = Vec4::new(5.0, 5.0, 5.0, 5.0);
        let vec_b = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vec_a.dot(vec_b), 50.0);
    }

    #[test]
    fn mul_mat() {
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0) * Mat::new(&[[4.0, 3.0, 2.0, 1.0]]),
            Mat::new(&[
                [4.0, 3.0, 2.0, 1.0],
                [8.0, 6.0, 4.0, 2.0],
                [12.0, 9.0, 6.0, 3.0],
                [16.0, 12.0, 8.0, 4.0],
            ])
        )
    }
}
