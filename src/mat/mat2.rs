/*!
2D transformation matrix.
*/

use ::std::{ops};

use ::num::{Zero, One, Scalar, Float};
use ::vec::{Vec2};
use ::angle::{Angle};

use super::Affine2;

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
			a11: a11, a12: a12,
			a21: a21, a22: a22,
		}
	}
	/// Identity matrix.
	pub fn identity() -> Mat2<T> where T: Zero + One {
		Mat2 {
			a11: T::one(), a12: T::zero(),
			a21: T::zero(), a22: T::one(),
		}
	}
	pub fn translate<V: Into<Vec2<T>>>(self, translate: V) -> Affine2<T> {
		let translate = translate.into();
		Affine2 {
			a11: self.a11, a12: self.a12, a13: translate.x,
			a21: self.a21, a22: self.a22, a23: translate.y,
		}
	}
	/// Scaling matrix.
	///
	/// Scales around the origin.
	pub fn scale<V: Into<Vec2<T>>>(scale: V) -> Mat2<T> where T: Zero {
		let scale = scale.into();
		Mat2 {
			a11: scale.x,   a12: T::zero(),
			a21: T::zero(), a22: scale.y,
		}
	}
	/// Rotation matrix.
	///
	/// Rotates around the origin.
	pub fn rotate<A: Angle<T = T>>(angle: A) -> Mat2<T> where T: Float {
		let (cy, cx) = angle.sin_cos();
		Mat2 {
			a11: cx, a12: -cy,
			a21: cy, a22:  cx,
		}
	}
	/// Skewing matrix.
	pub fn skew<V: Into<Vec2<T>>>(skew: V) -> Mat2<T> where T: One {
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
	pub fn reflect<V: Into<Vec2<T>>>(axis: V) -> Mat2<T> where T: Float {
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
			Mat2::scale((-T::one(), -T::one()))
		}
	}
	/// Projection matrix.
	///
	/// Projects onto the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix is the null matrix.
	pub fn project<V: Into<Vec2<T>>>(axis: V) -> Mat2<T> where T: Float {
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
			Mat2::default()
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> From<Affine2<T>> for Mat2<T> {
	fn from(affine: Affine2<T>) -> Mat2<T> {
		Mat2 {
			a11: affine.a11, a12: affine.a12,
			a21: affine.a21, a22: affine.a22,
		}
	}
}

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
//
// Any vector can be decomposed to `unit_x * vec.x + unit_y * vec.y` where `unit_x` is the unit vector in the X direction and `unit_y` is the unit vector in the Y direction.
//
// A linear transformation then changes the unit vectors. The transformed location can then be calculated.
//
// This calculation can be represented by a transformation matrix where the first column is the new unit vector for the X direction and the second column is the new unit vector for the Y direction.
//
// Decomposing a matrix is then simply accessing the transformed unit vectors.

impl<T> Mat2<T> {
	/// Compose the matrix from unit vectors.
	pub fn compose(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
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

impl<T> Mat2<T> {
	/// Calculates the determinant.
	pub fn det(self) -> T where T: Scalar {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	pub fn inverse(self) -> Mat2<T> where T: Float {
		let inv_det = T::one() / self.det();
		Mat2 {
			a11:  self.a22 * inv_det, a12: -self.a12 * inv_det,
			a21: -self.a21 * inv_det, a22:  self.a11 * inv_det,
		}
	}
	pub fn transpose(self) -> Mat2<T> {
		Mat2 {
			a11: self.a11, a12: self.a21,
			a21: self.a12, a22: self.a22,
		}
	}
}

//----------------------------------------------------------------
// Operators

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
impl<T: Copy + Zero + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Affine2<T>> for Mat2<T> {
	type Output = Affine2<T>;
	fn mul(self, rhs: Affine2<T>) -> Affine2<T> {
		Affine2::from(self) * rhs
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
