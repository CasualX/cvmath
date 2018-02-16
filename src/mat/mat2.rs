/*!
2D transformation matrix.
*/

use std::ops;

use num::{Scalar, Float};
use vec::Vec2;
use angle::Angle;

use super::{Affine2, Transform2};

/// 2D transformation matrix.
///
/// A 2x2 row-major matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	pub a11: T, pub a12: T,
	pub a21: T, pub a22: T,
}

/// 2D transformation matrix.
///
/// A 2x2 column-major matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	pub a11: T, pub a21: T,
	pub a12: T, pub a22: T,
}

//----------------------------------------------------------------
// Constructors

impl<T> Mat2<T> {
	pub fn new(
		a11: T, a12: T,
		a21: T, a22: T,
	) -> Mat2<T> {
		Mat2 {
			a11, a12,
			a21, a22,
		}
	}
}
impl<T: Scalar> Mat2<T> {
	/// Identity matrix.
	pub fn identity() -> Mat2<T> {
		Mat2 {
			a11: T::one(), a12: T::zero(),
			a21: T::zero(), a22: T::one(),
		}
	}
	/// Null matrix.
	pub fn null() -> Mat2<T> {
		Mat2 {
			a11: T::zero(), a12: T::zero(),
			a21: T::zero(), a22: T::zero(),
		}
	}
	/// Scaling matrix.
	///
	/// Scales around the origin.
	pub fn scale<V>(scale: V) -> Mat2<T> where V: Into<Vec2<T>> {
		let scale = scale.into();
		Mat2 {
			a11: scale.x,   a12: T::zero(),
			a21: T::zero(), a22: scale.y,
		}
	}
	/// Rotation matrix.
	///
	/// Rotates around the origin.
	pub fn rotate<A>(angle: A) -> Mat2<T> where T: Float, A: Angle<T = T> {
		let (cy, cx) = angle.sin_cos();
		Mat2 {
			a11: cx, a12: -cy,
			a21: cy, a22:  cx,
		}
	}
	/// Skewing matrix.
	pub fn skew<V>(skew: V) -> Mat2<T> where V: Into<Vec2<T>> {
		let skew = skew.into();
		Mat2 {
			a11: T::one(), a12: skew.x,
			a21: skew.y,   a22: T::one(),
		}
	}
	/// Reflection matrix.
	///
	/// Reflects around the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix will be a point reflection around the origin.
	pub fn reflect<V>(axis: V) -> Mat2<T> where T: Float, V: Into<Vec2<T>> {
		let l = axis.into();
		let ls = l.dot(l);
		if ls > T::zero() {
			let (lx, ly) = l.into();
			let ls = T::one() / ls;
			Mat2 {
				a11: ls * (lx * lx - ly * ly), a12: ls * (lx * ly + lx * ly),
				a21: ls * (lx * ly + lx * ly), a22: ls * (ly * ly - lx * lx),
			}
		}
		else {
			// Do something like point reflection instead of NaN
			Mat2::scale(-T::one())
		}
	}
	/// Projection matrix.
	///
	/// Projects onto the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix is the null matrix.
	pub fn project<V>(axis: V) -> Mat2<T> where T: Float, V: Into<Vec2<T>> {
		let u = axis.into();
		let us = u.dot(u);
		if us > T::zero() {
			let (ux, uy) = u.into();
			let us = T::one() / us;
			Mat2 {
				a11: us * ux * ux, a12: us * ux * uy,
				a21: us * ux * uy, a22: us * uy * uy,
			}
		}
		else {
			// Do something like absorb all
			Mat2::null()
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Mat2<T> {
	/// Imports as row major.
	pub fn from_row_major(mat: [[T; 2]; 2]) -> Mat2<T> where T: Copy {
		Mat2 {
			a11: mat[0][0], a12: mat[0][1],
			a21: mat[1][0], a22: mat[1][1],
		}
	}
	/// Imports as column major.
	pub fn from_column_major(mat: [[T; 2]; 2]) -> Mat2<T> where T: Copy {
		Mat2 {
			a11: mat[0][0], a12: mat[1][0],
			a21: mat[0][1], a22: mat[1][1],
		}
	}
	/// Exports as row major.
	pub fn into_row_major(self) -> [[T; 2]; 2] {
		[
			[self.a11, self.a12],
			[self.a21, self.a22],
		]
	}
	/// Exports as column major.
	pub fn into_column_major(self) -> [[T; 2]; 2] {
		[
			[self.a11, self.a21],
			[self.a12, self.a22],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Mat2<T> {
	/// Compose the matrix from unit vectors.
	///
	/// Any vector can be [decomposed](https://en.wikipedia.org/wiki/Vector_decomposition) by multiplying each unit vector with its respective component and then summing the result.
	///
	/// In 2D space this means `Vec2::unit_x() * x + Vec2::unit_y() * y`.
	///
	/// A linear transformation then can be defined by these unit vectors. The result is a transformation which remaps the unit vectors to their new location.
	///
	/// These unit vectors are simply the columns of the transformation matrix and as such can be trivially decomposed.
	pub fn compose<V>(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
		Mat2 {
			a11: x.x, a12: y.x,
			a21: x.y, a22: y.y,
		}
	}
	/// Gets the transformed X unit vector.
	pub fn x(self) -> Vec2<T> {
		Vec2 {
			x: self.a11,
			y: self.a21,
		}
	}
	/// Gets the transformed Y unit vector.
	pub fn y(self) -> Vec2<T> {
		Vec2 {
			x: self.a12,
			y: self.a22,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Mat2<T> {
	/// Calculates the determinant.
	pub fn det(&self) -> T {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	/// Calculates the inverse matrix.
	pub fn inverse(&self) -> Mat2<T> where T: Float {
		let det = self.det();
		if det != T::zero() {
			self.adjugate() * (T::one() / det)
		}
		else { *self }
	}
	/// Calculates the transposed matrix.
	pub fn transpose(&self) -> Mat2<T> {
		Mat2 {
			a11: self.a22, a12: self.a21,
			a21: self.a12, a22: self.a11,
		}
	}
	/// Calculates the adjugate matrix.
	pub fn adjugate(&self) -> Mat2<T> {
		Mat2 {
			a11:  self.a22, a12: -self.a12,
			a21: -self.a21, a22:  self.a11,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for Mat2<T> {
	type Output = Mat2<T>;
	fn mul(self, rhs: T) -> Mat2<T> {
		Mat2 {
			a11: self.a11 * rhs,
			a12: self.a12 * rhs,
			a21: self.a21 * rhs,
			a22: self.a22 * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Mat2<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.a11 *= rhs;
		self.a12 *= rhs;
		self.a21 *= rhs;
		self.a22 *= rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Mat2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12,
			y: rhs.x * self.a21 + rhs.y * self.a22,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat2<T>> for Mat2<T> {
	type Output = Mat2<T>;
	fn mul(self, rhs: Mat2<T>) -> Mat2<T> {
		Mat2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Affine2<T>> for Mat2<T> {
	type Output = Affine2<T>;
	fn mul(self, rhs: Affine2<T>) -> Affine2<T> {
		Affine2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat2<T>> for Mat2<T> {
	fn mul_assign(&mut self, rhs: Mat2<T>) {
		*self = *self * rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> Transform2<T> for Mat2<T> {}
