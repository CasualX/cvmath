/*!
2D affine transformation matrix.
*/

use super::*;

/// 2D affine transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
///
/// The third row is implied to be `[0, 0, 1]` and is omitted.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Transform2<T> {
	pub a11: T, pub a12: T, pub a13: T,
	pub a21: T, pub a22: T, pub a23: T,
}

// pub struct Transform2<T> {
// 	pub a11: T, pub a21: T,
// 	pub a12: T, pub a22: T,
// 	pub a13: T, pub a23: T,
// }

/// Constructs a new matrix from components.
#[allow(non_snake_case)]
#[inline]
pub const fn Transform2<T>(
	a11: T, a12: T, a13: T,
	a21: T, a22: T, a23: T,
) -> Transform2<T> {
	Transform2 { a11, a12, a13, a21, a22, a23 }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Transform2<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Transform2<T> {
	/// Constructs a new matrix from components.
	#[inline]
	pub const fn new(
		a11: T, a12: T, a13: T,
		a21: T, a22: T, a23: T,
	) -> Transform2<T> {
		Transform2 {
			a11, a12, a13,
			a21, a22, a23,
		}
	}
}
impl<T: Zero> Transform2<T> {
	/// Zero matrix.
	pub const ZERO: Transform2<T> = Transform2 {
		a11: T::ZERO, a12: T::ZERO, a13: T::ZERO,
		a21: T::ZERO, a22: T::ZERO, a23: T::ZERO,
	};
}
impl<T: Zero + One> Transform2<T> {
	/// Identity matrix.
	pub const IDENTITY: Transform2<T> = Transform2 {
		a11: T::ONE,  a12: T::ZERO, a13: T::ZERO,
		a21: T::ZERO, a22: T::ONE,  a23: T::ZERO,
	};
}
impl<T: Scalar> Transform2<T> {
	/// Translation matrix.
	#[inline]
	pub fn translate(trans: impl Into<Vec2<T>>) -> Transform2<T> {
		let trans = trans.into();
		Transform2 {
			a11: T::ONE,  a12: T::ZERO, a13: trans.x,
			a21: T::ZERO, a22: T::ONE,  a23: trans.y,
		}
	}
	/// Scaling matrix.
	///
	/// Scales around the origin.
	#[inline]
	pub fn scale(scale: impl Into<Vec2<T>>) -> Transform2<T> {
		let scale = scale.into();
		Transform2 {
			a11: scale.x, a12: T::ZERO, a13: T::ZERO,
			a21: T::ZERO, a22: scale.y, a23: T::ZERO,
		}
	}
	/// Rotation matrix.
	///
	/// Rotates around the origin.
	#[inline]
	pub fn rotate(angle: impl Angle<T = T>) -> Transform2<T> {
		Mat2::rotate(angle).affine()
	}
	/// Skewing matrix.
	#[inline]
	pub fn skew(skew: impl Into<Vec2<T>>) -> Transform2<T> {
		let skew = skew.into();
		Transform2 {
			a11: T::ONE, a12: skew.x, a13: T::ZERO,
			a21: skew.y, a22: T::ONE, a23: T::ZERO,
		}
	}
	/// Reflection matrix.
	///
	/// Reflects around the given axis.
	/// If axis is the zero vector, returns a point reflection around the origin.
	#[inline]
	pub fn reflect(line: impl Into<Vec2<T>>) -> Transform2<T> {
		Mat2::reflect(line).affine()
	}
	/// Projection matrix.
	///
	/// Projects onto the given axis.
	/// If axis is the zero vector, returns the zero matrix.
	#[inline]
	pub fn project(line: impl Into<Vec2<T>>) -> Transform2<T> {
		Mat2::project(line).affine()
	}
}

//----------------------------------------------------------------
// Conversions

impl<T: Zero + One> Transform2<T> {
	/// Converts to a 3x3 matrix.
	#[inline]
	pub fn mat3(self) -> Mat3<T> {
		self.into()
	}
}

impl<T> Transform2<T> {
	/// Imports the matrix from a row-major layout.
	#[inline]
	pub fn from_row_major(mat: [[T; 3]; 2]) -> Transform2<T> {
		let [[a11, a12, a13], [a21, a22, a23]] = mat;
		Transform2 {
			a11, a12, a13,
			a21, a22, a23,
		}
	}
	/// Imports the matrix from a column-major layout.
	#[inline]
	pub fn from_column_major(mat: [[T; 2]; 3]) -> Transform2<T> {
		let [[a11, a21], [a12, a22], [a13, a23]] = mat;
		Transform2 {
			a11, a12, a13,
			a21, a22, a23,
		}
	}
	/// Exports the matrix as a row-major array.
	#[inline]
	pub fn into_row_major(self) -> [[T; 3]; 2] {
		[
			[self.a11, self.a12, self.a13],
			[self.a21, self.a22, self.a23],
		]
	}
	/// Exports the matrix as a column-major array.
	#[inline]
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

impl<T> Transform2<T> {
	/// Composes the matrix from basis vectors.
	#[inline]
	pub fn compose(x: Vec2<T>, y: Vec2<T>, t: Vec2<T>) -> Transform2<T> {
		Transform2 {
			a11: x.x, a12: y.x, a13: t.x,
			a21: x.y, a22: y.y, a23: t.y,
		}
	}
	/// Gets the transformed X basis vector.
	#[inline]
	pub fn x(self) -> Vec2<T> {
		Vec2 {
			x: self.a11,
			y: self.a21,
		}
	}
	/// Gets the transformed Y basis vector.
	#[inline]
	pub fn y(self) -> Vec2<T> {
		Vec2 {
			x: self.a12,
			y: self.a22,
		}
	}
	/// Gets the translation vector.
	#[inline]
	pub fn t(self) -> Vec2<T> {
		Vec2 {
			x: self.a13,
			y: self.a23,
		}
	}
	/// Gets the rotation matrix.
	#[inline]
	pub fn mat2(self) -> Mat2<T> {
		Mat2 {
			a11: self.a11, a12: self.a12,
			a21: self.a21, a22: self.a22,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Transform2<T> {
	/// Computes the determinant.
	#[inline]
	pub fn determinant(self) -> T {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	/// Computes the inverse matrix.
	#[inline]
	pub fn inverse(self) -> Transform2<T> {
		let det = self.determinant();
		if det != T::ZERO {
			let inv_det = T::ONE / det;
			Transform2 {
				a11: self.a22 * inv_det,
				a12: -self.a12 * inv_det,
				a13: (self.a12 * self.a23 - self.a13 * self.a22) * inv_det,
				a21: -self.a21 * inv_det,
				a22: self.a11 * inv_det,
				a23: (self.a13 * self.a21 - self.a11 * self.a23) * inv_det,
			}
		}
		else {
			self
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Transform2<T> {
	type Output = Vec2<T>;
	#[inline]
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + self.a23,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Transform2<T> {
	type Output = Vec2<T>;
	#[inline]
	fn mul(self, rhs: Vec3<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Transform2<T>> for Transform2<T> {
	type Output = Transform2<T>;
	#[inline]
	fn mul(self, rhs: Transform2<T>) -> Transform2<T> {
		Transform2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat2<T>> for Transform2<T> {
	type Output = Transform2<T>;
	#[inline]
	fn mul(self, rhs: Mat2<T>) -> Transform2<T> {
		Transform2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a13,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Transform2<T>> for Transform2<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Transform2<T>) {
		*self = *self * rhs;
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat2<T>> for Transform2<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Mat2<T>) {
		*self = *self * rhs;
	}
}
