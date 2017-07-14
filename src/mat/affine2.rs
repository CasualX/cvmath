/*!
Affine 2D transformation matrix.
*/

use ::std::{ops};

use ::num::{Scalar, Float};
use ::vec::{Vec2, Vec3};
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
			a11, a12, a13,
			a21, a22, a23,
		}
	}
}
impl<T: Scalar> Affine2<T> {
	/// Identity matrix.
	pub fn identity() -> Affine2<T> {
		Affine2 {
			a11: T::one(), a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: T::one(), a23: T::zero(),
		}
	}
	/// Null matrix.
	pub fn null() -> Affine2<T> {
		Affine2 {
			a11: T::zero(), a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: T::zero(), a23: T::zero(),
		}
	}
	/// Translation matrix.
	pub fn translate<V>(trans: V) -> Affine2<T> where V: Into<Vec2<T>> {
		let trans = trans.into();
		Affine2 {
			a11: T::one(), a12: T::zero(), a13: trans.x,
			a21: T::zero(), a22: T::one(), a23: trans.y,
		}
	}
	/// Scaling matrix.
	///
	/// Scales around the origin.
	pub fn scale<V>(scale: V) -> Affine2<T> where V: Into<Vec2<T>> {
		let scale = scale.into();
		Affine2 {
			a11: scale.x,   a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: scale.y,   a23: T::zero(),
		}
	}
	/// Rotation matrix.
	///
	/// Rotates around the origin.
	pub fn rotate<A>(angle: A) -> Affine2<T> where T: Float, A: Angle<T = T> {
		Affine2::from_mat(Mat2::rotate(angle), Vec2::default())
	}
	/// Skewing matrix.
	pub fn skew<V>(skew: V) -> Affine2<T> where V: Into<Vec2<T>> {
		let skew = skew.into();
		Affine2 {
			a11: T::one(), a12: skew.x,   a13: T::zero(),
			a21: skew.y,   a22: T::one(), a23: T::zero(),
		}
	}
	/// Reflection matrix.
	///
	/// Reflects around the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix will be a point reflection around the origin.
	pub fn reflect<V>(line: V) -> Affine2<T> where T: Float, V: Into<Vec2<T>> {
		Affine2::from_mat(Mat2::reflect(line), Vec2::default())
	}
	/// Projection matrix.
	///
	/// Projects onto the line defined by the line going through the origin and `line`.
	///
	/// If `line` is the zero vector, the matrix is the null matrix.
	pub fn project<V>(line: V) -> Affine2<T> where T: Float, V: Into<Vec2<T>> {
		Affine2::from_mat(Mat2::project(line), Vec2::default())
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Affine2<T> {
	pub fn into_mat(self) -> (Mat2<T>, Vec2<T>) {
		(
			Mat2 {
				a11: self.a11, a12: self.a12,
				a21: self.a21, a22: self.a22,
			},
			Vec2 {
				x: self.a13,
				y: self.a23,
			}
		)
	}
	pub fn from_mat(mat: Mat2<T>, vec: Vec2<T>) -> Affine2<T> {
		Affine2 {
			a11: mat.a11, a12: mat.a12, a13: vec.x,
			a21: mat.a21, a22: mat.a22, a23: vec.y,
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
	/// Compose the matrix from unit vectors.
	///
	/// Any vector can be [decomposed](https://en.wikipedia.org/wiki/Vector_decomposition) by multiplying each unit vector with its respective component and then summing the result.
	///
	/// In 2D affine space this means `Vec2::unit_x() * x + Vec2::unit_y() * y + translate`.
	///
	/// A linear transformation then can be defined by these unit vectors. The result is a transformation which remaps the unit vectors to their new location.
	///
	/// These unit vectors are simply the columns of the transformation matrix and as such can be trivially decomposed.
	pub fn compose<V>(x: V, y: V, t: V) -> Affine2<T> where V: Into<Vec2<T>> {
		let (x, y, t) = (x.into(), y.into(), t.into());
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

impl<T: Scalar> Affine2<T> {
	/// Calculates the determinant.
	pub fn det(self) -> T {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	/// Calculates the inverse matrix.
	///
	/// Note that even though this matrix isn't strictly square, if an inverse exists, it will also be an affine transform.
	///
	/// This can be intuitively realized when you consider matrices as geometric linear transforms.
	/// Any affine transform, if its determinant isn't zero, has an inverse affine transform.
	///
	/// To prove this we only need to prove that translation is invertible with another translation (duh).
	/// The resulting inverse transform is then the inverse translation followed by the inverse of the matrix without the translation.
	pub fn inv(self) -> Affine2<T> where T: Float {
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

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Affine2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + self.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Affine2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Vec3<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23,
		}
	}
}
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
