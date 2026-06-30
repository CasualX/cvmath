use crate::*;

/// Types which can be cross producted.
pub trait Cross<Rhs = Self>: Copy {
	type Output;

	fn cross(self, rhs: Rhs) -> Self::Output;
}

/// Computes the cross product of `a` and `b`.
///
/// ```
/// assert_eq!(-10, cvmath::cross(cvmath::Vec2(-3, -4), cvmath::Vec2(-1, 2)));
/// assert_eq!(cvmath::Vec3(-12, 1, 39), cvmath::cross(cvmath::Vec3(3, -3, 1), cvmath::Vec3(4, 9, 1)));
/// ```
#[inline]
pub fn cross<T, U>(a: T, b: U) -> T::Output where T: Cross<U> {
	a.cross(b)
}

impl<T: Scalar> Cross for Vec2<T> {
	type Output = T;

	#[inline]
	fn cross(self, rhs: Vec2<T>) -> T {
		self.cross(rhs)
	}
}

impl<T: Scalar> Cross for Vec3<T> {
	type Output = Vec3<T>;

	#[inline]
	fn cross(self, rhs: Vec3<T>) -> Vec3<T> {
		self.cross(rhs)
	}
}
