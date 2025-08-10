use super::*;

/// Ray2 structure.
///
/// Rays are typically used to trace shapes for intersection tests.
/// See [`Ray2::trace`] for more information.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Ray2<T> {
	/// The origin point where the ray starts.
	pub origin: Point2<T>,

	/// The direction in which the ray extends from its origin.
	///
	/// This vector should be normalized and non-zero; otherwise, results may be incorrect.
	pub direction: Vec2<T>,

	/// Maximum distance to trace the ray.
	pub distance: T,
}

/// Ray2 constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Ray2<T>(origin: Point2<T>, direction: Vec2<T>, distance: T) -> Ray2<T> {
	Ray2 { origin, direction, distance }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Ray2<T> {}

impl<T> Ray2<T> {
	/// Constructs a new ray with normalized direction.
	///
	/// The direction is normalized. Zero directions may result in unexpected behavior.
	#[inline]
	pub fn new(origin: Point2<T>, direction: Vec2<T>, distance: T) -> Ray2<T> where T: Float {
		let direction = direction.norm();
		Ray2 { origin, direction, distance }
	}
}

impl<T: Float> Ray2<T> {
	/// Returns the point at a given distance along the ray's direction.
	#[inline]
	pub fn at(&self, distance: T) -> Point2<T> {
		self.origin.mul_add(self.direction, distance)
	}

	/// Returns a new ray that is a step along the ray's direction by the specified distance.
	#[inline]
	pub fn step(&self, distance: T) -> Ray2<T> {
		Ray2 {
			origin: self.origin.mul_add(self.direction, distance),
			direction: self.direction,
			distance: self.distance - distance,
		}
	}

	/// Computes the y coordinate where the ray intercepts the Y axis.
	///
	/// Returns none if the ray is parallel with the Y axis.
	#[inline]
	pub fn y_intercept(self) -> Option<T> {
		if self.direction.x == T::ZERO {
			return None;
		}
		let t = -self.origin.x / self.direction.x;
		if !(t >= T::ZERO && t <= self.distance) {
			return None;
		}
		let y = self.origin.y + self.direction.y * t;
		Some(y)
	}

	/// Computes the x coordinate where the ray intercepts the X axis.
	///
	/// Returns none if the ray is parallel with the X axis.
	#[inline]
	pub fn x_intercept(self) -> Option<T> {
		if self.direction.y == T::ZERO {
			return None;
		}
		let t = -self.origin.y / self.direction.y;
		if !(t >= T::ZERO && t <= self.distance) {
			return None;
		}
		let x = self.origin.x + self.direction.x * t;
		Some(x)
	}
}

impl<T: Float> ops::Mul<Ray2<T>> for Transform2<T> {
	type Output = Ray2<T>;

	#[inline]
	fn mul(self, ray: Ray2<T>) -> Ray2<T> {
		let origin = self * ray.origin;

		let (direction, distance) = if ray.distance.is_finite() {
			let end = self * ray.at(ray.distance);
			(end - origin).norm_len()
		}
		else {
			let dir = self.mat2() * ray.direction;
			(dir, ray.distance)
		};

		Ray2 { origin, direction, distance }
	}
}

/// Hit2 structure.
///
/// Represents an intersection point between a ray and a shape.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Hit2<T> {
	/// The distance from the ray's origin to the intersection point.
	pub distance: T,

	/// The surface normal at the intersection point.
	///
	/// This vector can be assumed to be normalized.
	pub normal: Vec2<T>,

	/// Index of the shape that was hit, if applicable.
	pub index: usize,
}

/// Shapes that support Ray2 intersection tests.
///
/// Types implementing this trait can be intersected by rays, returning hit information such as distance and surface normals.
pub trait Trace2<T> {
	/// Returns if the point starts inside the shape.
	fn inside(&self, pt: Point2<T>) -> bool;

	/// Trace the ray against a shape.
	///
	/// Returns the nearest intersection point along the ray, if any.
	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>>;
}

impl<T: Float, Shape: Trace2<T> + ?Sized> Trace2<T> for &Shape {
	#[inline]
	fn inside(&self, ray: Point2<T>) -> bool {
		(*self).inside(ray)
	}

	#[inline]
	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		(*self).trace(ray)
	}
}

impl<T: Float> Ray2<T> {
	/// Returns if the ray starts inside the shape.
	#[inline]
	pub fn inside<U: Trace2<T> + ?Sized>(&self, shape: &U) -> bool {
		shape.inside(self.origin)
	}

	/// Returns if the ray starts inside any shape in the collection.
	#[inline]
	pub fn inside_collection<Shape: Trace2<T>, I: IntoIterator<Item = Shape>>(&self, shapes: I) -> bool {
		shapes.into_iter().any(|shape| shape.inside(self.origin))
	}

	/// Trace the ray against a shape.
	#[inline]
	pub fn trace<U: Trace2<T> + ?Sized>(&self, shape: &U) -> Option<Hit2<T>> {
		shape.trace(self)
	}

	/// Trace the ray against a collection of shapes.
	#[inline]
	pub fn trace_collection<Shape: Trace2<T>, I: IntoIterator<Item = Shape>>(&self, shapes: I) -> Option<Hit2<T>> {
		shapes.into_iter().enumerate()
			.filter_map(|(index, shape)| shape.trace(self).map(|hit| Hit2 { index, ..hit }))
			.min_by(|a, b| T::total_cmp(&a.distance, &b.distance))
	}
}
