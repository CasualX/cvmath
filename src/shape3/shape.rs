use super::*;

/// Heterogeneous Shape3.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape3<T> {
	Point(Point3<T>),
	Bounds(Bounds3<T>),
	Plane(Plane3<T>),
	Sphere(Sphere<T>),
	Capsule(Capsule3<T>),
	Line(Line3<T>),
	Triangle(Triangle3<T>),
}

impl<T: Scalar> Shape3<T> {
	/// Returns the bounds of this shape, if it has one.
	///
	/// Planes return `None`.
	#[inline]
	pub fn bounds(&self) -> Option<Bounds3<T>> {
		match self {
			Shape3::Point(point) => Some(Bounds3(*point, *point)),
			Shape3::Bounds(bounds) => Some(*bounds),
			Shape3::Plane(_) => None,
			Shape3::Sphere(sphere) => Some(sphere.bounds()),
			Shape3::Capsule(capsule) => Some(capsule.bounds()),
			Shape3::Line(line) => Some(line.bounds()),
			Shape3::Triangle(triangle) => Some(triangle.bounds()),
		}
	}
}

impl<T: Float> Trace3<T> for Shape3<T> {
	fn inside(&self, pt: Point3<T>) -> bool {
		match self {
			Shape3::Point(point) => point.inside(pt),
			Shape3::Bounds(bounds) => bounds.inside(pt),
			Shape3::Plane(plane) => plane.inside(pt),
			Shape3::Sphere(sphere) => sphere.inside(pt),
			Shape3::Capsule(capsule) => capsule.inside(pt),
			Shape3::Line(line) => line.inside(pt),
			Shape3::Triangle(triangle) => triangle.inside(pt),
		}
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		match self {
			Shape3::Point(point) => point.trace(ray),
			Shape3::Bounds(bounds) => bounds.trace(ray),
			Shape3::Plane(plane) => plane.trace(ray),
			Shape3::Sphere(sphere) => sphere.trace(ray),
			Shape3::Capsule(capsule) => capsule.trace(ray),
			Shape3::Line(line) => line.trace(ray),
			Shape3::Triangle(triangle) => triangle.trace(ray),
		}
	}
}
