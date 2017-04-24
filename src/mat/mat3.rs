/*!
3D transformation matrix.
*/

use ::std::{ops};

use ::num::{Zero, One, Scalar};
use ::vec::{Vec3};

/// 3D transformation matrix.
///
/// A 3x3 row-major matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat3<T> {
	pub a11: T, pub a12: T, pub a13: T,
	pub a21: T, pub a22: T, pub a23: T,
	pub a31: T, pub a32: T, pub a33: T,
}

/// 3D transformation matrix.
///
/// A 3x3 column-major matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	pub a11: T, pub a21: T, pub a31: T,
	pub a12: T, pub a22: T, pub a32: T,
	pub a13: T, pub a23: T, pub a33: T,
}

//----------------------------------------------------------------
// Constructors

impl<T> Mat3<T> {
	pub fn new(
		a11: T, a12: T, a13: T,
		a21: T, a22: T, a23: T,
		a31: T, a32: T, a33: T,
	) -> Mat3<T> {
		Mat3 {
			a11: a11, a12: a12, a13: a13,
			a21: a21, a22: a22, a23: a23,
			a31: a31, a32: a32, a33: a33,
		}
	}
	/// Identity matrix.
	pub fn identity() -> Mat3<T> where T: Zero + One {
		Mat3 {
			a11: T::one(),  a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: T::one(),  a23: T::zero(),
			a31: T::zero(), a32: T::zero(), a33: T::one(),
		}
	}
	/// Scaling matrix.
	pub fn scale<V: Into<Vec3<T>>>(scale: V) -> Mat3<T> where T: Zero {
		let scale = scale.into();
		Mat3 {
			a11: scale.x,   a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: scale.y,   a23: T::zero(),
			a31: T::zero(), a32: T::zero(), a33: scale.z,
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Mat3<T> {
	/// Imports as row major.
	pub fn from_row_major(mat: [[T; 3]; 3]) -> Mat3<T> where T: Copy {
		Mat3 {
			a11: mat[0][0], a12: mat[0][1], a13: mat[0][2],
			a21: mat[1][0], a22: mat[1][1], a23: mat[1][2],
			a31: mat[2][0], a32: mat[2][1], a33: mat[2][2],
		}
	}
	/// Imports as column major.
	pub fn from_column_major(mat: [[T; 3]; 3]) -> Mat3<T> where T: Copy {
		Mat3 {
			a11: mat[0][0], a12: mat[1][0], a13: mat[2][0],
			a21: mat[0][1], a22: mat[1][1], a23: mat[2][1],
			a31: mat[0][2], a32: mat[1][2], a33: mat[2][2],
		}
	}
	/// Exports as row major.
	pub fn into_row_major(self) -> [[T; 3]; 3] {
		[
			[self.a11, self.a12, self.a13],
			[self.a21, self.a22, self.a23],
			[self.a31, self.a32, self.a33],
		]
	}
	/// Exports as column major.
	pub fn into_column_major(self) -> [[T; 3]; 3] {
		[
			[self.a11, self.a21, self.a31],
			[self.a12, self.a22, self.a32],
			[self.a13, self.a23, self.a33],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Mat3<T> {
	pub fn compose(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
		Mat3 {
			a11: x.x, a12: y.x, a13: z.x,
			a21: x.y, a22: y.y, a23: z.y,
			a31: x.z, a32: y.z, a33: z.z,
		}
	}
	/// Gets the transformed X unit vector.
	pub fn x(self) -> Vec3<T> {
		Vec3 {
			x: self.a11,
			y: self.a21,
			z: self.a31,
		}
	}
	/// Gets the transformed Y unit vector.
	pub fn y(self) -> Vec3<T> {
		Vec3 {
			x: self.a12,
			y: self.a22,
			z: self.a32,
		}
	}
	/// Gets the transformed Z unit vector.
	pub fn z(self) -> Vec3<T> {
		Vec3 {
			x: self.a13,
			y: self.a23,
			z: self.a33,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T> Mat3<T> {
	pub fn det(self) -> T where T: Scalar {
		self.a11 * self.a22 * self.a33 + self.a12 * self.a23 * self.a31 + self.a13 * self.a21 * self.a32
			- self.a13 * self.a22 * self.a31 - self.a12 * self.a21 * self.a33 - self.a11 * self.a23 * self.a32
	}
	pub fn inverse(self) -> Mat3<T> where T: Scalar {
		unimplemented!()
	}
	pub fn transpose(self) -> Mat3<T> {
		Mat3 {
			a11: self.a11, a12: self.a21, a13: self.a31,
			a21: self.a12, a22: self.a22, a23: self.a32,
			a31: self.a13, a32: self.a23, a33: self.a33,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat3<T>> for Mat3<T> {
	type Output = Mat3<T>;
	fn mul(self, rhs: Mat3<T>) -> Mat3<T> {
		Mat3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Mat3<T> {
	type Output = Vec3<T>;
	fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
		Vec3 {
			x: self.a11 * rhs.x + self.a12 * rhs.y + self.a13 * rhs.z,
			y: self.a21 * rhs.x + self.a22 * rhs.y + self.a23 * rhs.z,
			z: self.a31 * rhs.x + self.a32 * rhs.y + self.a33 * rhs.z,
		}
	}
}
