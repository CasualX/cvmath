/*!
Affine 2x3 transformation matrices.
*/

use ::std::{ops};

use ::num::{Zero, One, Scalar, Float};
use ::vec::{Vec2};
use ::angle::{Angle};

/// Affine 2x3 row-major transformation matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine2<T> {
	pub a11: T,
	pub a12: T,
	pub a13: T,
	pub a21: T,
	pub a22: T,
	pub a23: T,
}

/// Affine 2x3 column-major transformation matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine2<T> {
	pub a11: T,
	pub a21: T,
	pub a12: T,
	pub a22: T,
	pub a13: T,
	pub a23: T,
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
		Affine2 {
			a11: T::one(),  a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: T::one(),  a23: T::zero(),
		}
	}
	/// Translation matrix.
	pub fn translate<V: Into<Vec2<T>>>(trans: V) -> Affine2<T> where T: Zero + One {
		let trans = trans.into();
		Affine2 {
			a11: T::one(),  a12: T::zero(), a13: trans.x,
			a21: T::zero(), a22: T::one(),  a23: trans.y,
		}
	}
	/// Scaling matrix.
	pub fn scale<V: Into<Vec2<T>>>(scale: V) -> Affine2<T> where T: Zero + One {
		let scale = scale.into();
		Affine2 {
			a11: scale.x,   a12: T::zero(), a13: T::zero(),
			a21: T::zero(), a22: scale.y,   a23: T::zero(),
		}
	}
	/// Rotation matrix.
	pub fn rotate<A: Angle<T = T>>(angle: A) -> Affine2<T> where T: Float {
		let (cy, cx) = angle.sin_cos();
		Affine2 {
			a11: cx, a12: -cy, a13: T::zero(),
			a21: cy, a22: cx,  a23: T::zero(),
		}
	}
	/// Skewing matrix.
	pub fn skew<V: Into<Vec2<T>>>(skew: V) -> Affine2<T> where T: Zero + One {
		let skew = skew.into();
		Affine2 {
			a11: T::one(), a12: skew.x,   a13: T::zero(),
			a21: skew.y,   a22: T::one(), a23: T::zero(),
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
