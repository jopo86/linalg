use super::vector::{vec2::Vec2, vec3::Vec3, vec4::Vec4};
use num::Float;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

trait DimEqual<const A: usize, const B: usize> {}

impl<const A: usize> DimEqual<A, A> for () {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Mat<T: Float, const R: usize, const C: usize>(pub [[T; C]; R]);

impl<T: Float, const R: usize, const C: usize> Mat<T, R, C> {
    pub fn new(arr: &[[T; C]; R]) -> Self {
        Self(*arr)
    }

    pub fn fill(n: T) -> Self {
        Self([[n; C]; R])
    }

    pub fn zero() -> Self {
        Self::fill(T::zero())
    }
}

impl<T: Float, const N: usize> Mat<T, N, N> {
    pub fn diagonal(n: T) -> Self {
        let mut arr = [[T::zero(); N]; N];
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    arr[i][j] = n;
                }
            }
        }
        Self(arr)
    }

    pub fn identity() -> Self {
        Self::diagonal(T::one())
    }
}

impl<T: Float, const R: usize, const C: usize> Index<usize> for Mat<T, R, C> {
    type Output = [T; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Float, const R: usize, const C: usize> IndexMut<usize> for Mat<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Float, const R: usize, const C: usize> Neg for Mat<T, R, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut arr = [[T::zero(); C]; R];
        for i in 0..R {
            for j in 0..C {
                arr[i][j] = -self[i][j];
            }
        }
        Self(arr)
    }
}

impl<T: Float, const R: usize, const C: usize> Add<Mat<T, R, C>> for Mat<T, R, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut arr = [[T::zero(); C]; R];
        for i in 0..R {
            for j in 0..C {
                arr[i][j] = self.0[i][j] + rhs.0[i][j];
            }
        }
        Self(arr)
    }
}

impl<T: Float, const R: usize, const C: usize> AddAssign<Mat<T, R, C>> for Mat<T, R, C> {
    fn add_assign(&mut self, rhs: Mat<T, R, C>) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = self[i][j] + rhs[i][j];
            }
        }
    }
}

impl<T: Float, const R: usize, const C: usize> Sub<Mat<T, R, C>> for Mat<T, R, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut arr = [[T::zero(); C]; R];
        for i in 0..R {
            for j in 0..C {
                arr[i][j] = self.0[i][j] - rhs.0[i][j];
            }
        }
        Self(arr)
    }
}

impl<T: Float, const R: usize, const C: usize> SubAssign<Mat<T, R, C>> for Mat<T, R, C> {
    fn sub_assign(&mut self, rhs: Mat<T, R, C>) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = self[i][j] - rhs[i][j];
            }
        }
    }
}

impl<T: Float, const LR: usize, const LC: usize, const RR: usize, const RC: usize>
    Mul<Mat<T, RR, RC>> for Mat<T, LR, LC>
where
    (): DimEqual<LC, RR>,
{
    type Output = Mat<T, LR, RC>;

    fn mul(self, rhs: Mat<T, RR, RC>) -> Self::Output {
        let mut arr = [[T::zero(); RC]; LR];
        for lrow in 0..LR {
            for rcol in 0..RC {
                let mut sum = T::zero();
                for i in 0..LC {
                    sum = sum + self[lrow][i] * rhs[i][rcol];
                }
                arr[lrow][rcol] = sum;
            }
        }

        Mat::new(&arr)
    }
}

impl<T: Float, const N: usize> MulAssign for Mat<T, N, N> {
    fn mul_assign(&mut self, rhs: Self) {
        for lrow in 0..N {
            for rcol in 0..N {
                let mut sum = T::zero();
                for i in 0..N {
                    sum = sum + self[lrow][i] * rhs[i][rcol];
                }
                self[lrow][rcol] = sum;
            }
        }
    }
}

impl<T: Float> From<Vec2<T>> for Mat<T, 2, 1> {
    fn from(vec: Vec2<T>) -> Self {
        Self([[vec.x], [vec.y]])
    }
}

impl<T: Float> From<Vec3<T>> for Mat<T, 3, 1> {
    fn from(vec: Vec3<T>) -> Self {
        Self([[vec.x], [vec.y], [vec.z]])
    }
}

impl<T: Float> From<Vec4<T>> for Mat<T, 4, 1> {
    fn from(vec: Vec4<T>) -> Self {
        Self([[vec.x], [vec.y], [vec.z], [vec.w]])
    }
}

impl<T: Float, const R: usize> Mul<Vec2<T>> for Mat<T, R, 2> {
    type Output = Mat<T, R, 1>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self * Mat::from(rhs)
    }
}

impl<T: Float, const R: usize> Mul<Vec3<T>> for Mat<T, R, 3> {
    type Output = Mat<T, R, 1>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self * Mat::from(rhs)
    }
}

impl<T: Float, const R: usize> Mul<Vec4<T>> for Mat<T, R, 4> {
    type Output = Mat<T, R, 1>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        self * Mat::from(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_1() {
        let mat = Mat::new(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        assert_eq!(mat[0][0], 1.0);
        assert_eq!(mat[0][1], 2.0);
        assert_eq!(mat[0][2], 3.0);
        assert_eq!(mat[1][0], 4.0);
        assert_eq!(mat[1][1], 5.0);
        assert_eq!(mat[1][2], 6.0);
    }

    #[test]
    fn new_2() {
        let mat = Mat::new(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);

        assert_eq!(mat[0][0], 1.0);
        assert_eq!(mat[0][1], 2.0);
        assert_eq!(mat[1][0], 3.0);
        assert_eq!(mat[1][1], 4.0);
        assert_eq!(mat[2][0], 5.0);
        assert_eq!(mat[2][1], 6.0);
    }

    #[test]
    fn fill_1() {
        assert_eq!(
            Mat::<f32, 2, 3>::fill(1.0),
            Mat::new(&[[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]])
        );
    }

    #[test]
    fn fill_2() {
        assert_eq!(
            Mat::<f32, 3, 2>::fill(1.0),
            Mat::new(&[[1.0, 1.0], [1.0, 1.0], [1.0, 1.0]])
        );
    }

    #[test]
    fn zero_1() {
        assert_eq!(
            Mat::<f32, 2, 3>::zero(),
            Mat::new(&[[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        );
    }

    #[test]
    fn zero_2() {
        assert_eq!(
            Mat::<f32, 3, 2>::zero(),
            Mat::new(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]])
        );
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Mat::new(&[[1.0, 2.0], [3.0, 4.0]]),
            Mat::new(&[[-1.0, -2.0], [-3.0, -4.0]])
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Mat::new(&[[1.0, 2.0], [3.0, 4.0]]) + Mat::<f32, 2, 2>::fill(1.0),
            Mat::new(&[[2.0, 3.0], [4.0, 5.0]])
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Mat::new(&[[1.0, 2.0], [3.0, 4.0]]) - Mat::<f32, 2, 2>::fill(1.0),
            Mat::new(&[[0.0, 1.0], [2.0, 3.0]])
        );
    }

    #[test]
    fn mul_identity() {
        assert_eq!(
            Mat::new(&[[2.0, 3.0], [4.0, 5.0]]) * Mat::<f32, 2, 2>::identity(),
            Mat::new(&[[2.0, 3.0], [4.0, 5.0]])
        );
    }

    #[test]
    fn mul_square_2x2() {
        assert_eq!(
            Mat::new(&[[1.0, 2.0], [3.0, 4.0]]) * Mat::new(&[[5.0, 6.0], [7.0, 8.0]]),
            Mat::new(&[[19.0, 22.0], [43.0, 50.0]])
        );
    }

    #[test]
    fn mul_square_3x3() {
        assert_eq!(
            Mat::new(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0],])
                * Mat::new(&[[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0],]),
            Mat::new(&[[30.0, 24.0, 18.0], [84.0, 69.0, 54.0], [138.0, 114.0, 90.0],])
        );
    }

    #[test]
    fn mul_2x3_3x2() {
        assert_eq!(
            Mat::new(&[[1.0, 4.0, 2.0], [3.0, 0.0, 5.0]])
                * Mat::new(&[[2.0, 3.0], [1.0, 5.0], [4.0, 0.0]]),
            Mat::new(&[[14.0, 23.0], [26.0, 9.0]])
        );
    }

    #[test]
    fn mul_2x3_3x4() {
        assert_eq!(
            Mat::new(&[[2.0, 3.0, 4.0], [1.0, 0.0, 2.0],])
                * Mat::new(&[
                    [0.0, 1.0, 2.0, 3.0],
                    [1.0, 2.0, 3.0, 4.0],
                    [4.0, 5.0, 6.0, 7.0],
                ]),
            Mat::new(&[[19.0, 28.0, 37.0, 46.0], [8.0, 11.0, 14.0, 17.0],])
        );
    }

    #[test]
    fn mul_vec2() {
        assert_eq!(
            Mat::new(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]]) * Vec2::new(2.0, 1.0),
            Mat::new(&[[4.0], [10.0], [16.0], [22.0]])
        );
    }

    #[test]
    fn mul_vec3() {
        assert_eq!(
            Mat::new(&[
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
                [10.0, 11.0, 12.0]
            ]) * Vec3::new(3.0, 2.0, 1.0),
            Mat::new(&[[10.0], [28.0], [46.0], [64.0]])
        );
    }

    #[test]
    fn mul_vec4() {
        assert_eq!(
            Mat::new(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]) * Vec4::new(4.0, 3.0, 2.0, 1.0),
            Mat::new(&[[20.0], [60.0], [100.0], [140.0]])
        );
    }
}
