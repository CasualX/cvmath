use crate::*;

/// Types which can measure distance to another value.
pub trait Distance<Rhs = Self>: Copy {
	type T;

	fn distance(self, rhs: Rhs) -> Self::T;
}

/// Computes the distance between `a` and `b`.
///
/// ```
/// assert_eq!(5.0, cvmath::distance(cvmath::Vec2(0.0, 0.0), cvmath::Vec2(3.0, 4.0)));
/// ```
#[inline]
pub fn distance<T, U>(a: T, b: U) -> T::T where T: Distance<U> {
	a.distance(b)
}

macro_rules! impl_vec_distance {
	($vec:ident) => {
		impl<T: Float> Distance for $vec<T> {
			type T = T;

			#[inline]
			fn distance(self, rhs: $vec<T>) -> T {
				self.distance(rhs)
			}
		}
	};
}

impl_vec_distance!(Vec2);
impl_vec_distance!(Vec3);
impl_vec_distance!(Vec4);

impl<T: Float> Distance<Point2<T>> for Line2<T> {
	type T = T;

	#[inline]
	fn distance(self, pt: Point2<T>) -> T {
		self.distance(pt)
	}
}

impl<T: Float> Distance<Point2<T>> for Plane2<T> {
	type T = T;

	#[inline]
	fn distance(self, pt: Point2<T>) -> T {
		self.distance(pt)
	}
}

impl<T: Float> Distance<Point3<T>> for Line3<T> {
	type T = T;

	#[inline]
	fn distance(self, pt: Point3<T>) -> T {
		self.distance(pt)
	}
}

impl<T: Float> Distance<Point3<T>> for Plane3<T> {
	type T = T;

	#[inline]
	fn distance(self, pt: Point3<T>) -> T {
		Plane3::distance(&self, pt)
	}
}
