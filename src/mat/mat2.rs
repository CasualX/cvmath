/*!
*/

use ::std::{ops};

use ::num::{Zero, One, Scalar, Float};
use ::vec::{Vec2};
use ::angle::{Angle};

use super::Affine2;

/// 2D row-major transformation matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	pub a11: T,
	pub a12: T,
	pub a21: T,
	pub a22: T,
}

/// 2D column-major transformation matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	pub a11: T,
	pub a21: T,
	pub a12: T,
	pub a22: T,
}

//----------------------------------------------------------------
// Constructors

impl<T> Mat2<T> {
	pub fn new(a11: T, a12: T,
	           a21: T, a22: T) -> Mat2<T> {
		Mat2 {
			a11: a11, a12: a12,
			a21: a21, a22: a22,
		}
	}
	pub fn compose(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
		Mat2 {
			a11: x.x, a12: y.x,
			a21: x.y, a22: y.y,
		}
	}
	/// Identity matrix.
	pub fn identity() -> Mat2<T> where T: Zero + One {
		Mat2 {
			a11: T::one(), a12: T::zero(),
			a21: T::zero(), a22: T::one(),
		}
	}
	/// Scaling matrix.
	pub fn scale<V: Into<Vec2<T>>>(scale: V) -> Mat2<T> where T: Zero + One {
		let scale = scale.into();
		Mat2 {
			a11: scale.x,   a12: T::zero(),
			a21: T::zero(), a22: scale.y,
		}
	}
	/// Rotation matrix.
	pub fn rotate<A: Angle<T = T>>(angle: A) -> Mat2<T> where T: Float {
		let (cy, cx) = angle.sin_cos();
		Mat2 {
			a11: cx, a12: -cy,
			a21: cy, a22: cx,
		}
	}
	/// Skewing matrix.
	pub fn skew<V: Into<Vec2<T>>>(skew: V) -> Mat2<T> where T: Zero + One {
		let skew = skew.into();
		Mat2 {
			a11: T::one(), a12: skew.x,
			a21: skew.y,   a22: T::one(),
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
	pub fn inverse(self) -> Mat2<T> where T: Scalar + Float {
		let inv_det = T::one() / self.det();
		Mat2 {
			a11: self.a22 * inv_det,
			a12: -self.a12 * inv_det,
			a21: -self.a21 * inv_det,
			a22: self.a11 * inv_det,
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

macro_rules! transform {
	($mat:expr, $vec:expr) => {
		Vec2 {
			x: $vec.x * $mat.a11 + $vec.y * $mat.a12,
			y: $vec.x * $mat.a21 + $vec.y * $mat.a22,
		}
	};
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Mat2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		transform!(self, rhs)
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat2<T>> for Vec2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Mat2<T>) -> Vec2<T> {
		transform!(rhs, self)
	}
}
