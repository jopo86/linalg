use num::Float;
use std::ops::Index;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default)]
pub struct Mat2x3<T: Float>(pub [[T; 3]; 2]);

impl<T: Float> Mat2x3<T> {
    pub fn new(m00: T, m01: T, m02: T, m10: T, m11: T, m12: T) -> Self {
        Self([[m00, m01, m02], [m10, m11, m12]])
    }

    pub fn from(arr: &[[T; 3]; 2]) -> Self {
        Self(*arr)
    }

    pub fn zero() -> Self {
        Self([[T::zero(), T::zero(), T::zero()], [T::zero(), T::zero(), T::zero()]])
    }

    pub fn fill(n: T) -> Self {
        Self([[n, n, n], [n, n, n]])
    }
}

impl<T: Float> Index<usize> for Mat2x3<T> {
    type Output = [T; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mat = Mat2x3::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(mat[0][0], 1.0);
        assert_eq!(mat[0][1], 2.0);
        assert_eq!(mat[1][0], 3.0);
        assert_eq!(mat[1][1], 4.0);
    }

    #[test]
    fn from() {
        let mat = Mat2x3::from(&[[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(mat, Mat2x3::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn zero() {
        let mat = Mat2x3::zero();
        assert_eq!(mat, Mat2x3::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn fill() {
        let mat = Mat2x3::fill(1.0);
        assert_eq!(mat, Mat2x3::new(1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn diagonal() {
        let mat = Mat2x3::diagonal(2.0);
        assert_eq!(mat, Mat2x3::new(2.0, 0.0, 0.0, 2.0));
    }

    #[test]
    fn identity() {
        let mat = Mat2x3::identity();
        assert_eq!(mat, Mat2x3::new(1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn index() {
        let mat = Mat2x3::new(1.0, 2.0, 3.0, 4.0);
        
        assert_eq!(mat.0[0], mat[0]);
        assert_eq!(mat.0[1], mat[1]);

        assert_eq!(mat.0[0][0], mat[0][0]);
        assert_eq!(mat.0[0][1], mat[0][1]);
        assert_eq!(mat.0[1][0], mat[1][0]);
        assert_eq!(mat.0[1][1], mat[1][1]);
    }
}
