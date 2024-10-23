use num::Float;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default)]
pub struct Vec2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn fill(n: T) -> Self {
        Self { x: n, y: n }
    }

    pub fn mag(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Float> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Float> Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Float> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Float> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Float> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vec = Vec2::new(1.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 3.0);
    }

    #[test]
    fn zero() {
        let vec = Vec2::zero();
        assert_eq!(vec, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn fill() {
        let vec = Vec2::fill(1.0);
        assert_eq!(vec, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn add() {
        let vec_a = Vec2::new(5.0, 5.0);
        let vec_b = Vec2::new(1.0, 3.0);
        assert_eq!(vec_a + vec_b, Vec2::new(6.0, 8.0));
    }

    #[test]
    fn sub() {
        let vec_a = Vec2::new(5.0, 5.0);
        let vec_b = Vec2::new(1.0, 3.0);
        assert_eq!(vec_a - vec_b, Vec2::new(4.0, 2.0));
    }

    #[test]
    fn dot() {
        let vec_a = Vec2::new(5.0, 5.0);
        let vec_b = Vec2::new(1.0, 3.0);
        assert_eq!(vec_a.dot(vec_b), 20.0);
    }
}
