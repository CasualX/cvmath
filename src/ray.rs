use super::*;

/// Ray structure.
///
/// Rays are used to trace shapes in 3D space, see [`trace`](Ray::trace).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Ray<T> {
	pub origin: Point3<T>,
	pub direction: Vec3<T>,
}

/// Ray constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Ray<T>(origin: Point3<T>, direction: Vec3<T>) -> Ray<T> {
	Ray { origin, direction }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Ray<T> {}

impl<T> Ray<T> {
	/// Constructs a new ray.
	#[inline]
	pub const fn new(origin: Point3<T>, direction: Vec3<T>) -> Ray<T> {
		Ray { origin, direction }
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Ray<T>> for Transform3<T> {
	type Output = Ray<T>;
	#[inline]
	fn mul(self, ray: Ray<T>) -> Ray<T> {
		Ray {
			origin: self * ray.origin,
			direction: self * ray.direction,
		}
	}
}

/// Trace hit structure.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TraceHit<T> {
	/// Distance along the ray to the intersection point.
	pub distance: T,
	/// Normal of the shape at the intersection point.
	pub normal: Vec3<T>,
}

/// Shapes that can be traced by a ray.
pub trait TraceRay<T> {
	/// Returns if the ray starts inside the shape.
	fn inside(&self, ray: &Ray<T>) -> bool;

	/// Trace the ray against a shape.
	///
	/// See [`Ray::trace`](Ray::trace) for more information.
	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize;
}

impl<T: Float> Ray<T> {
	/// Returns the point at a distance along the ray.
	#[inline]
	pub fn at(&self, distance: T) -> Point3<T> {
		self.origin + self.direction * distance
	}

	/// Returns if the ray starts inside the shape.
	#[inline]
	pub fn inside<U: TraceRay<T>>(&self, shape: &U) -> bool {
		shape.inside(self)
	}

	/// Trace the ray against a shape.
	///
	/// Returns the number of hits along the ray to the intersection points.
	///
	/// The hits are not sorted in any particular order.
	/// Hits can be both entering and exiting the shape, check the hit normal to determine the direction.
	#[inline]
	pub fn trace<U: TraceRay<T>>(&self, shape: &U, hits: &mut [TraceHit<T>]) -> usize {
		shape.trace(self, hits)
	}
}
