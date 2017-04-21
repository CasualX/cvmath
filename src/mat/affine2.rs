/*!
Affine 2D transformation matrix.
*/

use ::std::{ops};

use ::num::{Zero, One, Scalar, Float};
use ::vec::{Vec2};
use ::angle::{Angle};

use super::Mat2;

/// Affine 2D transformation matrix.
///
/// A 2x3 row-major matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine2<T> {
	pub a11: T, pub a12: T, pub a13: T,
	pub a21: T, pub a22: T, pub a23: T,
}

/// Affine 2D transformation matrix.
///
/// A 2x3 column-major matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine2<T> {
	pub a11: T, pub a21: T,
	pub a12: T, pub a22: T,
	pub a13: T, pub a23: T,
}

//----------------------------------------------------------------
// Constructors

impl<T> Affine2<T> {
	pub fn new(a11: T, a12: T, a13: T,
	           a21: T, a22: T, a23: T) -> Affine2<T> {
		Affine2 {
			a11: a11, a12: a12, a13: a13,
			a21: a21, a22: a22, a23: a23,
		}
	}
	/// Identity matrix.
	pub fn identity() -> Affine2<T> where T: Zero + One {
		Mat2::identity().into()
	}
	/// Null matrix.
	pub fn zero() -> Affine2<T> where T: Zero {
		Mat2::zero().into()
	}
	/// Translation matrix.
	pub fn translate<V: Into<Vec2<T>>>(trans: V) -> Affine2<T> where T: Zero + One {
		Mat2::identity().translate(trans)
	}
	/// Scaling matrix.
	///
	/// Scales around the origin.
	pub fn scale<V: Into<Vec2<T>>>(scale: V) -> Affine2<T> where T: Zero {
		Mat2::scale(scale).into()
	}
	/// Rotation matrix.
	///
	/// Rotates around the origin.
	pub fn rotate<A: Angle<T = T>>(angle: A) -> Affine2<T> where T: Float {
		Mat2::rotate(angle).into()
	}
	/// Skewing matrix.
	pub fn skew<V: Into<Vec2<T>>>(skew: V) -> Affine2<T> where T: Zero + One {
		Mat2::skew(skew).into()
	}
	/// Reflection matrix.
	///
	/// Reflects around the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix will be a point reflection around the origin.
	pub fn reflect<V: Into<Vec2<T>>>(line: V) -> Affine2<T> where T: Float {
		Mat2::reflect(line).into()
	}
	/// Projection matrix.
	///
	/// Projects onto the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix is the null matrix.
	pub fn project<V: Into<Vec2<T>>>(line: V) -> Affine2<T> where T: Float {
		Mat2::project(line).into()
	}
}

//----------------------------------------------------------------
// Conversions

impl<T: Zero> From<Mat2<T>> for Affine2<T> {
	fn from(mat: Mat2<T>) -> Affine2<T> {
		Affine2 {
			a11: mat.a11, a12: mat.a12, a13: T::zero(),
			a21: mat.a21, a22: mat.a22, a23: T::zero(),
		}
	}
}

impl<T> Affine2<T> {
	/// Imports as row major.
	pub fn from_row_major(mat: [[T; 3]; 2]) -> Affine2<T> where T: Copy {
		Affine2 {
			a11: mat[0][0], a12: mat[0][1], a13: mat[0][2],
			a21: mat[1][0], a22: mat[1][1], a23: mat[1][2],
		}
	}
	/// Imports as column major.
	pub fn from_column_major(mat: [[T; 2]; 3]) -> Affine2<T> where T: Copy {
		Affine2 {
			a11: mat[0][0], a12: mat[1][0], a13: mat[2][0],
			a21: mat[0][1], a22: mat[1][1], a23: mat[2][1],
		}
	}
	/// Exports as row major.
	pub fn into_row_major(self) -> [[T; 3]; 2] {
		[
			[self.a11, self.a12, self.a13],
			[self.a21, self.a22, self.a23],
		]
	}
	/// Exports as column major.
	pub fn into_column_major(self) -> [[T; 2]; 3] {
		[
			[self.a11, self.a21],
			[self.a12, self.a22],
			[self.a13, self.a23],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Affine2<T> {
	pub fn compose(x: Vec2<T>, y: Vec2<T>, t: Vec2<T>) -> Affine2<T> {
		Affine2 {
			a11: x.x, a12: y.x, a13: t.x,
			a21: x.y, a22: y.y, a23: t.y,
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
	/// Gets the translation vector.
	pub fn t(self) -> Vec2<T> {
		Vec2 {
			x: self.a13,
			y: self.a23,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T> Affine2<T> {
	/// Calculates the determinant.
	pub fn det(self) -> T where T: Scalar {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	pub fn inverse(self) -> Affine2<T> where T: Scalar + Float {
		let inv_det = T::one() / self.det();
		Affine2 {
			a11: self.a22 * inv_det,
			a12: -self.a12 * inv_det,
			a13: (self.a12 * self.a23 - self.a13 * self.a22) * inv_det,
			a21: -self.a21 * inv_det,
			a22: self.a11 * inv_det,
			a23: (self.a13 * self.a21 - self.a11 * self.a23) * inv_det,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Affine2<T>> for Affine2<T> {
	type Output = Affine2<T>;
	fn mul(self, rhs: Affine2<T>) -> Affine2<T> {
		Affine2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13,
			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Affine2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + self.a23,
		}
	}
}
