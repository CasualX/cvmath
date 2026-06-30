use crate::*;

/// Types which can be dot producted.
pub trait Dot: Copy {
	type T;

	fn dot(self, other: Self) -> Self::T;
}

/// Computes the dot product of `a` and `b`.
///
/// ```
/// assert_eq!(12, cvmath::dot(cvmath::Vec3(1, 2, 3), cvmath::Vec3(4, -5, 6)));
/// ```
#[inline]
pub fn dot<T: Dot>(a: T, b: T) -> T::T {
	a.dot(b)
}

impl<T: Scalar> Dot for T {
	type T = T;

	#[inline]
	fn dot(self, other: Self) -> Self::T {
		self * other
	}
}

macro_rules! impl_vec_dot {
	($vec:ident) => {
		impl<T: Scalar> Dot for $vec<T> {
			type T = T;

			#[inline]
			fn dot(self, rhs: $vec<T>) -> T {
				self.dot(rhs)
			}
		}
	};
}

impl_vec_dot!(Vec2);
impl_vec_dot!(Vec3);
impl_vec_dot!(Vec4);

impl<T: Scalar> Dot for Quat<T> {
	type T = T;

	#[inline]
	fn dot(self, other: Quat<T>) -> T {
		self.dot(other)
	}
}
