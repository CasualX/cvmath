use super::*;

/// Ray structure.
///
/// Rays are typically used to trace shapes for intersection tests.
/// See [`Ray3::trace`] for more information.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Ray3<T> {
	/// The origin point where the ray starts.
	pub origin: Point3<T>,

	/// The direction in which the ray extends from its origin.
	///
	/// This vector should be normalized and non-zero; otherwise, results may be incorrect.
	pub direction: Vec3<T>,

	/// Maximum distance to trace the ray.
	pub distance: T,
}

/// Ray constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Ray3<T>(origin: Point3<T>, direction: Vec3<T>, distance: T) -> Ray3<T> {
	Ray3 { origin, direction, distance }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Ray3<T> {}

impl<T> Ray3<T> {
	/// Constructs a new ray with normalized direction.
	///
	/// The direction is normalized. Zero directions may result in unexpected behavior.
	#[inline]
	pub fn new(origin: Point3<T>, direction: Vec3<T>, distance: T) -> Ray3<T> where T: Float {
		let direction = direction.norm();
		Ray3 { origin, direction, distance }
	}
}

impl<T: Float> Ray3<T> {
	/// Returns the point at a given distance along the ray's direction.
	#[inline]
	pub fn at(&self, distance: T) -> Point3<T> {
		self.origin.mul_add(self.direction, distance)
	}

	/// Returns a new ray that is a step along the ray's direction by the specified distance.
	#[inline]
	pub fn step(&self, distance: T) -> Ray3<T> {
		Ray3 {
			origin: self.origin.mul_add(self.direction, distance),
			direction: self.direction,
			distance: self.distance - distance,
		}
	}
}

impl<T: Float> ops::Mul<Ray3<T>> for Transform3<T> {
	type Output = Ray3<T>;

	#[inline]
	fn mul(self, ray: Ray3<T>) -> Ray3<T> {
		let origin = self * ray.origin;

		let (direction, distance) = if ray.distance.is_finite() {
			let end = self * ray.at(ray.distance);
			(end - origin).norm_len()
		}
		else {
			let dir = self.mat3() * ray.direction;
			(dir.norm(), ray.distance)
		};

		Ray3 { origin, direction, distance }
	}
}

/// Trace hit structure.
///
/// Represents an intersection point between a ray and a shape.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Hit3<T> {
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
pub trait Trace3<T> {
	/// Returns if the ray starts inside the shape.
	fn inside(&self, pt: Point3<T>) -> bool;

	/// Trace the ray against a shape.
	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>>;
}

impl<T: Float, Shape: Trace3<T> + ?Sized> Trace3<T> for &Shape {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		(*self).inside(pt)
	}

	#[inline]
	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		(*self).trace(ray)
	}
}

impl<T: Float> Ray3<T> {
	/// Returns if the ray starts inside the shape.
	#[inline]
	pub fn inside<U: Trace3<T> + ?Sized>(&self, shape: &U) -> bool {
		shape.inside(self.origin)
	}

	/// Returns if the ray starts inside any shape in the collection.
	#[inline]
	pub fn inside_collection<Shape: Trace3<T>, I: IntoIterator<Item = Shape>>(&self, shapes: I) -> bool {
		shapes.into_iter().any(|shape| shape.inside(self.origin))
	}

	/// Trace the ray against a shape.
	#[inline]
	pub fn trace<U: Trace3<T> + ?Sized>(&self, shape: &U) -> Option<Hit3<T>> {
		shape.trace(self)
	}

	/// Trace the ray against a collection of shapes.
	#[inline]
	pub fn trace_collection<Shape: Trace3<T>, I: IntoIterator<Item = Shape>>(&self, shapes: I) -> Option<Hit3<T>> {
		shapes.into_iter().enumerate()
			.filter_map(|(index, shape)| shape.trace(self).map(|hit| Hit3 { index, ..hit }))
			.min_by(|a, b| T::total_cmp(&a.distance, &b.distance))
	}
}
