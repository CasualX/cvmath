use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape3<T> {
	Plane(Plane<T>),
	Sphere(Sphere<T>),
	Box(Bounds3<T>),
	Triangle(Triangle3<T>),
}

impl<T: Float> TraceRay<T> for Shape3<T> {
	#[inline]
	fn inside(&self, ray: &Ray<T>) -> bool {
		match self {
			Shape3::Plane(plane) => plane.inside(ray),
			Shape3::Sphere(sphere) => sphere.inside(ray),
			Shape3::Box(bounds) => bounds.inside(ray),
			Shape3::Triangle(triangle) => triangle.inside(ray),
		}
	}

	#[inline]
	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize {
		match self {
			Shape3::Plane(plane) => plane.trace(ray, hits),
			Shape3::Sphere(sphere) => sphere.trace(ray, hits),
			Shape3::Box(bounds) => bounds.trace(ray, hits),
			Shape3::Triangle(triangle) => triangle.trace(ray, hits),
		}
	}
}
