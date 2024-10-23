use num::Float;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default)]
pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn fill(n: T) -> Self {
        Self { x: n, y: n, z: n }
    }

    pub fn mag(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: other.x * self.z - self.x * other.z,
            z: self.x * other.y - other.x * self.y,
        }
    }
}

impl<T: Float> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T: Float> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn zero() {
        let vec = Vec3::zero();
        assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn fill() {
        let vec = Vec3::fill(1.0);
        assert_eq!(vec, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn add() {
        let vec_a = Vec3::new(5.0, 5.0, 5.0);
        let vec_b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec_a + vec_b, Vec3::new(6.0, 7.0, 8.0));
    }

    #[test]
    fn sub() {
        let vec_a = Vec3::new(5.0, 5.0, 5.0);
        let vec_b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec_a - vec_b, Vec3::new(4.0, 3.0, 2.0));
    }

    #[test]
    fn dot() {
        let vec_a = Vec3::new(5.0, 5.0, 5.0);
        let vec_b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec_a.dot(vec_b), 30.0);
    }

    #[test]
    fn cross_general_1() {
        let vec_a = Vec3::new(1.0, 2.0, 3.0);
        let vec_b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(vec_a.cross(vec_b), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn cross_general_2() {
        let vec_a = Vec3::new(3.0, -3.0, 1.0);
        let vec_b = Vec3::new(4.0, 9.0, 2.0);
        assert_eq!(vec_a.cross(vec_b), Vec3::new(-15.0, -2.0, 39.0));
    }

    #[test]
    fn cross_orthogonal() {
        let vec_a = Vec3::new(1.0, 0.0, 0.0);
        let vec_b = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(vec_a.cross(vec_b), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn cross_parallel() {
        let vec_a = Vec3::new(2.0, 2.0, 2.0);
        let vec_b = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(vec_a.cross(vec_b), Vec3::zero());
    }
}
