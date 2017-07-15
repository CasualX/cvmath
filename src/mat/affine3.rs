/*!
Affine 3D transformation matrix.
*/

use ::std::{ops};

use ::num::{Scalar};
use ::vec::{Vec3, Vec4};

use super::{Mat3, Transform3};

/// Affine 3D transformation matrix.
///
/// A 3x4 row-major matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine3<T> {
	pub a11: T, pub a12: T, pub a13: T, pub a14: T,
	pub a21: T, pub a22: T, pub a23: T, pub a24: T,
	pub a31: T, pub a32: T, pub a33: T, pub a34: T,
}

/// Affine 3D transformation matrix.
///
/// A 3x4 column-major matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine3<T> {
	pub a11: T, pub a21: T, pub a31: T,
	pub a12: T, pub a22: T, pub a32: T,
	pub a13: T, pub a23: T, pub a33: T,
	pub a14: T, pub a24: T, pub a34: T,
}

//----------------------------------------------------------------
// Constructors

impl<T> Affine3<T> {
	pub fn new(a11: T, a12: T, a13: T, a14: T,
	           a21: T, a22: T, a23: T, a24: T,
	           a31: T, a32: T, a33: T, a34: T) -> Affine3<T> {
		Affine3 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Affine3<T> {
	pub fn from_row_major(mat: [[T; 4]; 3]) -> Affine3<T> where T: Copy {
		Affine3 {
			a11: mat[0][0], a12: mat[0][1], a13: mat[0][2], a14: mat[0][3],
			a21: mat[1][0], a22: mat[1][1], a23: mat[1][2], a24: mat[1][3],
			a31: mat[2][0], a32: mat[2][1], a33: mat[2][2], a34: mat[2][3],
		}
	}
	pub fn from_column_major(mat: [[T; 3]; 4]) -> Affine3<T> where T: Copy {
		Affine3 {
			a11: mat[0][0], a12: mat[1][0], a13: mat[2][0], a14: mat[3][0],
			a21: mat[0][1], a22: mat[1][1], a23: mat[2][1], a24: mat[3][1],
			a31: mat[0][2], a32: mat[1][2], a33: mat[2][2], a34: mat[3][2],
		}
	}
	pub fn into_row_major(self) -> [[T; 4]; 3] {
		[
			[self.a11, self.a12, self.a13, self.a14],
			[self.a21, self.a22, self.a23, self.a24],
			[self.a31, self.a32, self.a33, self.a34],
		]
	}
	pub fn into_column_major(self) -> [[T; 3]; 4] {
		[
			[self.a11, self.a21, self.a31],
			[self.a12, self.a22, self.a32],
			[self.a13, self.a23, self.a33],
			[self.a14, self.a24, self.a34],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Affine3<T> {
	pub fn compose<V>(x: V, y: V, z: V, t: V) -> Affine3<T> where V: Into<Vec3<T>> {
		let (x, y, z, t) = (x.into(), y.into(), z.into(), t.into());
		Affine3 {
			a11: x.x, a12: y.x, a13: z.x, a14: t.x,
			a21: x.y, a22: y.y, a23: z.y, a24: t.y,
			a31: x.z, a32: y.z, a33: z.z, a34: t.z,
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
	/// Gets the translation vector.
	pub fn t(self) -> Vec3<T> {
		Vec3 {
			x: self.a14,
			y: self.a24,
			z: self.a34,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Affine3<T> {
	pub fn det(self) -> T {
		self.a11 * (self.a22 * self.a33 - self.a23 * self.a32) +
		self.a12 * (self.a23 * self.a31 - self.a21 * self.a33) +
		self.a13 * (self.a21 * self.a32 - self.a22 * self.a31)
	}
	pub fn inv(self) -> Affine3<T> {
		let inv_det = T::one() / self.det();
		Affine3 {
			a11: (self.a22 * self.a33 - self.a23 * self.a32) * inv_det,
			a12: (self.a13 * self.a32 - self.a12 * self.a33) * inv_det,
			a13: (self.a12 * self.a23 - self.a13 * self.a22) * inv_det,
			a14: (self.a12 * (self.a24 * self.a33 - self.a23 * self.a34) +
			      self.a13 * (self.a22 * self.a34 - self.a24 * self.a32) +
			      self.a14 * (self.a23 * self.a32 - self.a22 * self.a33)) * inv_det,
			a21: (self.a23 * self.a31 - self.a21 * self.a33) * inv_det,
			a22: (self.a11 * self.a33 - self.a13 * self.a31) * inv_det,
			a23: (self.a13 * self.a21 - self.a11 * self.a23) * inv_det,
			a24: (self.a11 * (self.a23 * self.a34 - self.a24 * self.a33) +
			      self.a13 * (self.a24 * self.a31 - self.a21 * self.a34) +
			      self.a14 * (self.a21 * self.a33 - self.a23 * self.a31)) * inv_det,
			a31: (self.a21 * self.a32 - self.a22 * self.a31) * inv_det,
			a32: (self.a12 * self.a31 - self.a11 * self.a32) * inv_det,
			a33: (self.a11 * self.a22 - self.a12 * self.a21) * inv_det,
			a34: (self.a11 * (self.a24 * self.a32 - self.a22 * self.a34) +
			      self.a12 * (self.a21 * self.a34 - self.a24 * self.a31) +
			      self.a14 * (self.a22 * self.a31 - self.a21 * self.a32)) * inv_det,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Affine3<T> {
	type Output = Vec3<T>;
	fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
		Vec3 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13 + self.a14,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23 + self.a24,
			z: rhs.x * self.a31 + rhs.y * self.a32 + rhs.z * self.a33 + self.a34,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec4<T>> for Affine3<T> {
	type Output = Vec3<T>;
	fn mul(self, rhs: Vec4<T>) -> Vec3<T> {
		Vec3 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13 + rhs.w * self.a14,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23 + rhs.w * self.a24,
			z: rhs.x * self.a31 + rhs.y * self.a32 + rhs.z * self.a33 + rhs.w * self.a34,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Affine3<T>> for Affine3<T> {
	type Output = Affine3<T>;
	fn mul(self, rhs: Affine3<T>) -> Affine3<T> {
		Affine3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33,
			a14: self.a11 * rhs.a14 + self.a12 * rhs.a24 + self.a13 * rhs.a34 + self.a14,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33,
			a24: self.a21 * rhs.a14 + self.a22 * rhs.a24 + self.a23 * rhs.a34 + self.a24,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33,
			a34: self.a31 * rhs.a14 + self.a32 * rhs.a24 + self.a33 * rhs.a34 + self.a34,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat3<T>> for Affine3<T> {
	type Output = Affine3<T>;
	fn mul(self, rhs: Mat3<T>) -> Affine3<T> {
		Affine3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33,
			a14: self.a14,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33,
			a24: self.a24,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33,
			a34: self.a34,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> Transform3<T> for Affine3<T> {}
