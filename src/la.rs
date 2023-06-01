#![allow(unused_imports)]

use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ptr::copy;

#[derive(Clone, Debug, Copy)]
pub struct Matrix<T: Sized, const ROW: usize, const COLUMN: usize>([[T; COLUMN]; ROW]);
pub type Vector<T, const COLUMN: usize> = Matrix<T, 1, COLUMN>;
pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

pub trait X {
    type Output;

    fn x(self) -> Self::Output;
}

pub trait Y {
    type Output;

    fn y(self) -> Self::Output;
}

pub trait Z {
    type Output;

    fn z(self) -> Self::Output;
}

pub trait W {
    type Output;

    fn w(self) -> Self::Output;
}

pub trait R {
    type Output;

    fn r(self) -> Self::Output;
}

pub trait G {
    type Output;

    fn g(self) -> Self::Output;
}

pub trait B {
    type Output;

    fn b(self) -> Self::Output;
}

pub trait A {
    type Output;

    fn a(self) -> Self::Output;
}

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

pub trait CrossAssign<Rhs = Self> {
    fn cross_assign(&mut self, rhs: Rhs);
}

pub trait Magnitude {
    type Output;

    fn magnitude(self) -> Self::Output;
}

pub trait Normal {
    type Output;

    fn normal(self) -> Self::Output;
}

pub trait NormalAssign {
    fn normal_aasign(&mut self);
}

pub trait Scale<T> {
    type Output;

    fn scale(self, factor: T) -> Self::Output;
}

pub trait ScaleAssign<T> {
    fn scale_assign(&mut self, factor: T);
}

pub trait Transpose {
    type Output;

    fn transpose(self) -> Self::Output;
}

pub trait TransposeAssign {
    fn transpose_assign(&mut self);
}

macro_rules! impl_vector_xyzw_rgba {
    ($t0:ty, $i0:ident, $t1:ty, $t2:ty, $l0:literal) => {
        impl $t0 for $t1 {
            type Output = $t2;

            fn $i0(self) -> Self::Output {
                self.0[0][$l0]
            }
        }

        impl<'a> $t0 for &'a $t1 {
            type Output = &'a $t2;

            fn $i0(self) -> Self::Output {
                &self.0[0][$l0]
            }
        }

        impl<'a> $t0 for &'a mut $t1 {
            type Output = &'a mut $t2;

            fn $i0(self) -> Self::Output {
                &mut self.0[0][$l0]
            }
        }
    };
}

macro_rules! impl_vector_dot {
    ($($t0:ty),+$(,)?) => {
        $(
            impl<const COLUMN: usize> Dot for &Vector<$t0, COLUMN> {
                type Output = $t0;

                fn dot(self, rhs: Self) -> Self::Output {
                    let mut result = 0 as $t0;

                    for i in 0..COLUMN {
                        result += self.0[0][i] * rhs.0[0][i];
                    }

                    result
                }
            }

            impl<const COLUMN: usize> Dot<Vector<$t0, COLUMN>> for &Vector<$t0, COLUMN> {
                type Output = $t0;

                fn dot(self, rhs: Vector<$t0, COLUMN>) -> Self::Output {
                    self.dot(&rhs)
                }
            }

            impl<const COLUMN: usize> Dot<&Self> for Vector<$t0, COLUMN> {
                type Output = $t0;

                fn dot(self, rhs: &Self) -> Self::Output {
                    (&self).dot(rhs)
                }
            }

            impl<const COLUMN: usize> Dot for Vector<$t0, COLUMN> {
                type Output = $t0;

                fn dot(self, rhs: Self) -> Self::Output {
                    (&self).dot(&rhs)
                }
            }
        )+
    };
}

macro_rules! impl_vector_cross {
    ($($t0:ty),+$(,)?) => {
        $(
            impl Cross for &Vector3<$t0> {
                type Output = Vector3<$t0>;

                fn cross(self, rhs: Self) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; 3]; 1]);

                    result.0[0][0] = self.0[0][1] * rhs.0[0][2] - self.0[0][2] * rhs.0[0][1];
                    result.0[0][1] = self.0[0][2] * rhs.0[0][0] - self.0[0][0] * rhs.0[0][2];
                    result.0[0][2] = self.0[0][0] * rhs.0[0][1] - self.0[0][1] * rhs.0[0][0];

                    result
                }
            }

            impl Cross<Vector3<$t0>> for &Vector3<$t0> {
                type Output = Vector3<$t0>;

                fn cross(self, rhs: Vector3<$t0>) -> Self::Output {
                    self.cross(&rhs)
                }
            }

            impl Cross<&Self> for Vector3<$t0> {
                type Output = Self;

                fn cross(self, rhs: &Self) -> Self::Output {
                    (&self).cross(rhs)
                }
            }

            impl Cross for Vector3<$t0> {
                type Output = Self;

                fn cross(self, rhs: Self) -> Self::Output {
                    (&self).cross(&rhs)
                }
            }

            impl CrossAssign<&Self> for Vector3<$t0> {
                fn cross_assign(&mut self, rhs: &Self) {
                    *self = self.cross(rhs);
                }
            }

            impl CrossAssign for Vector3<$t0> {
                fn cross_assign(&mut self, rhs: Self) {
                    self.cross_assign(&rhs);
                }
            }
        )+
    };
}

macro_rules! impl_vector_magnitude {
    ($($t0:ty),+$(,)?) => {
        $(
            impl<const COLUMN: usize> Magnitude for &Vector<$t0, COLUMN> {
                type Output = $t0;

                fn magnitude(self) -> Self::Output {
                    let mut sum = 0 as $t0;

                    for i in 0..COLUMN {
                        sum += self.0[0][i].powi(2);
                    }

                    sum.sqrt()
                }
            }

            impl<const COLUMN: usize> Magnitude for Vector<$t0, COLUMN> {
                type Output = $t0;

                fn magnitude(self) -> Self::Output {
                    (&self).magnitude()
                }
            }
        )+
    };
}

macro_rules! impl_vector_normal {
    ($($t0:ty),+$(,)?) => {
        $(
            impl<const COLUMN: usize> Normal for &Vector<$t0, COLUMN> {
                type Output = Vector<$t0, COLUMN>;

                fn normal(self) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; COLUMN]; 1]);

                    let mag = self.magnitude();

                    for i in 0..COLUMN {
                        result.0[0][i] = self.0[0][i] / mag;
                    }

                    result
                }
            }

            impl<const COLUMN: usize> Normal for Vector<$t0, COLUMN> {
                type Output = Self;

                fn normal(self) -> Self::Output {
                    (&self).normal()
                }
            }

            impl<const COLUMN: usize> NormalAssign for Vector<$t0, COLUMN> {
                fn normal_aasign(&mut self) {
                    *self = self.normal();
                }
            }
        )+
    };
}

macro_rules! impl_vector_from_vector {
    ($t0:ty, $t1:ty, $t2:ty, $l0:literal, $l1:literal) => {
        impl From<$t0> for $t1 {
            fn from(value: $t0) -> Self {
                let mut result = Self([[0 as $t2; $l1]; 1]);
                let ptr_result = &mut result.0 as *mut [[$t2; $l1]; 1] as *mut $t2;

                let ptr_value = &value.0 as *const [[$t2; $l0]; 1] as *const $t2;

                unsafe {
                    copy(ptr_value, ptr_result, $l1);
                }

                result
            }
        }
    };
}

macro_rules! impl_matrix_eq {
    ($($t0:ty),+$(,)?) => {
        $(
            impl<const ROW: usize, const COLUMN: usize> Eq for Matrix<$t0, ROW, COLUMN> {}
        )+
    };
}

macro_rules! impl_matrix {
    ($($t0:ty),+$(,)?) => {
        $(
            // impl From
            impl<const ROW: usize, const COLUMN: usize> From<$t0> for Matrix<$t0, ROW, COLUMN> {
                fn from(value: $t0) -> Self {
                    Self([[value; COLUMN]; ROW])
                }
            }

            impl<const ROW: usize, const COLUMN: usize> From<[$t0; COLUMN]> for Matrix<$t0, ROW, COLUMN> {
                fn from(value: [$t0; COLUMN]) -> Self {
                    Self([value; ROW])
                }
            }

            impl<const ROW: usize, const COLUMN: usize> From<[[$t0; COLUMN]; ROW]>
                for Matrix<$t0, ROW, COLUMN>
            {
                fn from(value: [[$t0; COLUMN]; ROW]) -> Self {
                    Self(value)
                }
            }

            // impl Add, AddAssign
            impl<const ROW: usize, const COLUMN: usize> Add for &Matrix<$t0, ROW, COLUMN> {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn add(self, rhs: Self) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; COLUMN]; ROW]);

                    for r in 0..ROW {
                        for c in 0..COLUMN {
                            result.0[r][c] = self.0[r][c] + rhs.0[r][c];
                        }
                    }

                    result
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Add<Matrix<$t0, ROW, COLUMN>>
                for &Matrix<$t0, ROW, COLUMN>
            {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn add(self, rhs: Matrix<$t0, ROW, COLUMN>) -> Self::Output {
                    self + &rhs
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Add<&Self> for Matrix<$t0, ROW, COLUMN> {
                type Output = Self;

                fn add(self, rhs: &Self) -> Self::Output {
                    &self + rhs
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Add for Matrix<$t0, ROW, COLUMN> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    &self + &rhs
                }
            }

            impl<const ROW: usize, const COLUMN: usize> AddAssign<&Self> for Matrix<$t0, ROW, COLUMN> {
                fn add_assign(&mut self, rhs: &Self) {
                    *self = &*self + rhs;
                }
            }

            impl<const ROW: usize, const COLUMN: usize> AddAssign<Self> for Matrix<$t0, ROW, COLUMN> {
                fn add_assign(&mut self, rhs: Self) {
                    *self += &rhs;
                }
            }

            // impl Sub, SubAssign
            impl<const ROW: usize, const COLUMN: usize> Sub for &Matrix<$t0, ROW, COLUMN> {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn sub(self, rhs: Self) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; COLUMN]; ROW]);

                    for r in 0..ROW {
                        for c in 0..COLUMN {
                            result.0[r][c] = self.0[r][c] - rhs.0[r][c];
                        }
                    }

                    result
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Sub<Matrix<$t0, ROW, COLUMN>>
                for &Matrix<$t0, ROW, COLUMN>
            {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn sub(self, rhs: Matrix<$t0, ROW, COLUMN>) -> Self::Output {
                    self - &rhs
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Sub<&Self> for Matrix<$t0, ROW, COLUMN> {
                type Output = Self;

                fn sub(self, rhs: &Self) -> Self::Output {
                    &self - rhs
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Sub for Matrix<$t0, ROW, COLUMN> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    &self - &rhs
                }
            }

            impl<const ROW: usize, const COLUMN: usize> SubAssign<&Self> for Matrix<$t0, ROW, COLUMN> {
                fn sub_assign(&mut self, rhs: &Self) {
                    *self = &*self - rhs;
                }
            }

            impl<const ROW: usize, const COLUMN: usize> SubAssign<Self> for Matrix<$t0, ROW, COLUMN> {
                fn sub_assign(&mut self, rhs: Self) {
                    *self -= &rhs;
                }
            }

            // impl Mul, MulAssign
            impl<const ROW: usize, const MIDDLE: usize, const COLUMN: usize> Mul<&Matrix<$t0, MIDDLE, COLUMN>>
                for &Matrix<$t0, ROW, MIDDLE>
            {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn mul(self, rhs: &Matrix<$t0, MIDDLE, COLUMN>) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; COLUMN]; ROW]);

                    for r in 0..ROW {
                        for c in 0..COLUMN {
                            for m in 0..MIDDLE {
                                result.0[r][c] += self.0[r][m] * rhs.0[m][c];
                            }
                        }
                    }

                    result
                }
            }

            impl<const ROW: usize, const MIDDLE: usize, const COLUMN: usize> Mul<Matrix<$t0, MIDDLE, COLUMN>>
                for &Matrix<$t0, ROW, MIDDLE>
            {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn mul(self, rhs: Matrix<$t0, MIDDLE, COLUMN>) -> Self::Output {
                    self * &rhs
                }
            }

            impl<const ROW: usize, const MIDDLE: usize, const COLUMN: usize> Mul<&Matrix<$t0, MIDDLE, COLUMN>>
                for Matrix<$t0, ROW, MIDDLE>
            {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn mul(self, rhs: &Matrix<$t0, MIDDLE, COLUMN>) -> Self::Output {
                    &self * rhs
                }
            }

            impl<const ROW: usize, const MIDDLE: usize, const COLUMN: usize> Mul<Matrix<$t0, MIDDLE, COLUMN>>
                for Matrix<$t0, ROW, MIDDLE>
            {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn mul(self, rhs: Matrix<$t0, MIDDLE, COLUMN>) -> Self::Output {
                    &self * &rhs
                }
            }

            impl<const N: usize> MulAssign<&Self> for Matrix<$t0, N, N> {
                fn mul_assign(&mut self, rhs: &Self) {
                    *self = &*self * rhs;
                }
            }

            impl<const N: usize> MulAssign<Self> for Matrix<$t0, N, N> {
                fn mul_assign(&mut self, rhs: Self) {
                    *self *= &rhs;
                }
            }

            // impl Scale, ScaleAssign
            impl<const ROW: usize, const COLUMN: usize> Scale<$t0> for &Matrix<$t0, ROW, COLUMN> {
                type Output = Matrix<$t0, ROW, COLUMN>;

                fn scale(self, factor: $t0) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; COLUMN]; ROW]);

                    for r in 0..ROW {
                        for c in 0..COLUMN {
                            result.0[r][c] = self.0[r][c] * factor;
                        }
                    }

                    result
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Scale<$t0> for Matrix<$t0, ROW, COLUMN> {
                type Output = Self;

                fn scale(self, factor: $t0) -> Self::Output {
                    (&self).scale(factor)
                }
            }

            impl<const ROW: usize, const COLUMN: usize> ScaleAssign<$t0> for Matrix<$t0, ROW, COLUMN> {
                fn scale_assign(&mut self, factor: $t0) {
                    *self = self.scale(factor);
                }
            }

            // impl Transpose, TransposeAssign
            impl<const ROW: usize, const COLUMN: usize> Transpose for &Matrix<$t0, ROW, COLUMN> {
                type Output = Matrix<$t0, COLUMN, ROW>;

                fn transpose(self) -> Self::Output {
                    let mut result = Matrix([[0 as $t0; ROW]; COLUMN]);

                    for r in 0..ROW {
                        for c in 0..COLUMN {
                            result.0[c][r] = self.0[r][c]
                        }
                    }

                    result
                }
            }

            impl<const ROW: usize, const COLUMN: usize> Transpose for Matrix<$t0, ROW, COLUMN> {
                type Output = Matrix<$t0, COLUMN, ROW>;

                fn transpose(self) -> Self::Output {
                    (&self).transpose()
                }
            }

            impl<const N: usize> TransposeAssign for Matrix<$t0, N, N> {
                fn transpose_assign(&mut self) {
                    *self = self.transpose()
                }
            }

            // impl Deref, DerefMut
            impl<const ROW: usize, const COLUMN: usize> Deref for Matrix<$t0, ROW, COLUMN> {
                type Target = [[$t0; COLUMN]; ROW];

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<const ROW: usize, const COLUMN: usize> DerefMut for Matrix<$t0, ROW, COLUMN> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            // impl PartialEq
            impl<const ROW: usize, const COLUMN: usize> PartialEq for Matrix<$t0, ROW, COLUMN> {
                fn eq(&self, other: &Self) -> bool {
                    for r in 0..ROW {
                        for c in 0..COLUMN {
                            if self.0[r][c] != other.0[r][c] {
                                return false;
                            }
                        }
                    }
                    return true;
                }
            }

            //
            impl_vector_xyzw_rgba!(X, x, Vector2<$t0>, $t0, 0);
            impl_vector_xyzw_rgba!(Y, y, Vector2<$t0>, $t0, 1);

            impl_vector_xyzw_rgba!(X, x, Vector3<$t0>, $t0, 0);
            impl_vector_xyzw_rgba!(Y, y, Vector3<$t0>, $t0, 1);
            impl_vector_xyzw_rgba!(Z, z, Vector3<$t0>, $t0, 2);

            impl_vector_xyzw_rgba!(X, x, Vector4<$t0>, $t0, 0);
            impl_vector_xyzw_rgba!(Y, y, Vector4<$t0>, $t0, 1);
            impl_vector_xyzw_rgba!(Z, z, Vector4<$t0>, $t0, 2);
            impl_vector_xyzw_rgba!(W, w, Vector4<$t0>, $t0, 3);

            impl_vector_xyzw_rgba!(R, r, Vector3<$t0>, $t0, 0);
            impl_vector_xyzw_rgba!(G, g, Vector3<$t0>, $t0, 1);
            impl_vector_xyzw_rgba!(B, b, Vector3<$t0>, $t0, 2);

            impl_vector_xyzw_rgba!(R, r, Vector4<$t0>, $t0, 0);
            impl_vector_xyzw_rgba!(G, g, Vector4<$t0>, $t0, 1);
            impl_vector_xyzw_rgba!(B, b, Vector4<$t0>, $t0, 2);
            impl_vector_xyzw_rgba!(A, a, Vector4<$t0>, $t0, 3);

            impl_vector_from_vector!(Vector4<$t0>, Vector3<$t0>, $t0, 4, 3);
            impl_vector_from_vector!(Vector4<$t0>, Vector2<$t0>, $t0, 4, 2);

            impl_vector_from_vector!(Vector3<$t0>, Vector2<$t0>, $t0, 3, 2);
        )+
    };
}
impl_matrix!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,);
impl_matrix_eq!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_vector_dot!(f32, f64,);
impl_vector_cross!(f32, f64,);
impl_vector_magnitude!(f32, f64,);
impl_vector_normal!(f32, f64,);
