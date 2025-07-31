use super::*;

/// Ray structure.
///
/// Rays are typically used to trace shapes for intersection tests.
/// See [`Ray::trace`] for more information.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Ray<T> {
	/// The origin point where the ray starts.
	pub origin: Point3<T>,

	/// The direction in which the ray extends from its origin.
	///
	/// This vector should be normalized and non-zero; otherwise, results may be incorrect.
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
	/// Constructs a new ray with normalized direction.
	///
	/// The direction is normalized. Zero directions may result in unexpected behavior.
	#[inline]
	pub fn new(origin: Point3<T>, direction: Vec3<T>) -> Ray<T> where T: Float {
		let direction = direction.norm();
		Ray { origin, direction }
	}
}

/// Transforms the ray's origin and direction.
///
/// This allows transforming rays through space using standard linear transforms.
/// Assumes the transform preserves ray semantics (e.g., no non-uniform scaling for normals).
impl<T: Float> ops::Mul<Ray<T>> for Transform3<T> {
	type Output = Ray<T>;

	#[inline]
	fn mul(self, ray: Ray<T>) -> Ray<T> {
		Ray {
			origin: self * ray.origin,
			direction: (self.mat3() * ray.direction).norm(),
		}
	}
}

/// Trace hit structure.
///
/// Represents an intersection point between a ray and a shape.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TraceHit<T> {
	/// The distance from the ray's origin to the intersection point.
	pub distance: T,

	/// The surface normal at the intersection point.
	///
	/// This vector can be assumed to be normalized.
	pub normal: Vec3<T>,

	/// Index of the shape that was hit, if applicable.
	pub index: usize,
}

/// Shapes that support ray intersection tests.
///
/// Types implementing this trait can be intersected by rays, returning hit information such as distance and surface normals.
pub trait TraceRay<T> {
	/// Returns if the ray starts inside the shape.
	fn inside(&self, ray: &Ray<T>) -> bool;

	/// Trace the ray against a shape.
	///
	/// Implementors may write up to `hits.len()` intersection points to the `hits` buffer,
	/// but must return the total number of intersection points found.
	///
	/// This allows the caller to query how many intersection points exist without storing them.
	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize;
}

impl<T: Float> Ray<T> {
	/// Returns the point at a given distance along the ray's direction.
	#[inline]
	pub fn at(&self, distance: T) -> Point3<T> {
		self.origin.mul_add(self.direction, distance)
	}

	/// Returns if the ray starts inside the shape.
	///
	/// This method delegates to the [`TraceRay::inside`] implementation of the shape.
	#[inline]
	pub fn inside<U: TraceRay<T> + ?Sized>(&self, shape: &U) -> bool {
		shape.inside(self)
	}

	/// Trace the ray against a shape.
	///
	/// This method delegates to the [`TraceRay::trace`] implementation of the shape.
	///
	/// Returns the **total number of intersection points** along the ray, regardless of how many
	/// were stored in `hits`. This allows passing an empty slice to query the hit count only.
	///
	/// Note: Hits are not sorted. Use the surface normal to determine entry/exit.
	#[inline]
	pub fn trace<U: TraceRay<T> + ?Sized>(&self, shape: &U, hits: &mut [TraceHit<T>]) -> usize {
		shape.trace(self, hits)
	}
}
