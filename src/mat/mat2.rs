/*!
*/

use ::std::{ops};

use ::num::{Zero, One, Scalar, Float};
use ::vec::{Vec2};
use ::angle::{Angle};

/// 2D row-major transformation matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	a11: T,
	a12: T,
	a21: T,
	a22: T,
}

/// 2D column-major transformation matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	a11: T,
	a21: T,
	a12: T,
	a22: T,
}

//----------------------------------------------------------------
// Constructors

impl<T> Mat2<T> {
	/// Constructs from scalars.
	pub fn new(a11: T, a12: T,
	           a21: T, a22: T) -> Mat2<T> {
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
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Mat2<T> {
	type Output = Vec2<T>;
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12,
			y: rhs.x * self.a21 + rhs.y * self.a22,
		}
	}
}
