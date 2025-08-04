use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape3<T> {
	Point(Point3<T>),
	Bounds(Bounds3<T>),
	Plane(Plane3<T>),
	Sphere(Sphere<T>),
	Line(Line3<T>),
	Triangle(Triangle3<T>),
}

impl<T: Float> Shape3<T> {
	#[inline]
	pub fn inside(&self, pt: Point3<T>) -> bool {
		match self {
			Shape3::Point(point) => point.inside(pt),
			Shape3::Bounds(bounds) => bounds.inside(pt),
			Shape3::Plane(plane) => plane.inside(pt),
			Shape3::Sphere(sphere) => sphere.inside(pt),
			Shape3::Line(line) => line.inside(pt),
			Shape3::Triangle(triangle) => triangle.inside(pt),
		}
	}
}

impl<T: Float> Trace3<T> for Shape3<T> {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		self.inside(pt)
	}

	#[inline]
	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		match self {
			Shape3::Point(point) => point.trace(ray),
			Shape3::Bounds(bounds) => bounds.trace(ray),
			Shape3::Plane(plane) => plane.trace(ray),
			Shape3::Sphere(sphere) => sphere.trace(ray),
			Shape3::Line(line) => line.trace(ray),
			Shape3::Triangle(triangle) => triangle.trace(ray),
		}
	}
}
