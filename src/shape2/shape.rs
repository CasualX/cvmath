use super::*;

/// Heterogeneous Shape2.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape2<T> {
	Point(Point2<T>),
	Bounds(Bounds2<T>),
	Plane(Plane2<T>),
	Circle(Circle<T>),
	Line(Line2<T>),
	Triangle(Triangle2<T>),
}

impl<T: Scalar> Shape2<T> {
	/// Returns the bounds of this shape, if it has one.
	///
	/// Planes return `None`.
	#[inline]
	pub fn bounds(&self) -> Option<Bounds2<T>> {
		match self {
			Shape2::Point(point) => Some(Bounds2(*point, *point)),
			Shape2::Bounds(bounds) => Some(*bounds),
			Shape2::Plane(_) => None,
			Shape2::Circle(circle) => Some(circle.bounds()),
			Shape2::Line(line) => Some(line.bounds()),
			Shape2::Triangle(triangle) => Some(triangle.bounds()),
		}
	}
}

impl<T: Float> Trace2<T> for Shape2<T> {
	fn inside(&self, pt: Point2<T>) -> bool {
		match self {
			Shape2::Point(point) => point.inside(pt),
			Shape2::Bounds(bounds) => bounds.inside(pt),
			Shape2::Plane(plane) => plane.inside(pt),
			Shape2::Circle(circle) => circle.inside(pt),
			Shape2::Line(line) => line.inside(pt),
			Shape2::Triangle(triangle) => triangle.inside(pt),
		}
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		match self {
			Shape2::Point(point) => point.trace(ray),
			Shape2::Bounds(bounds) => bounds.trace(ray),
			Shape2::Plane(plane) => plane.trace(ray),
			Shape2::Circle(circle) => circle.trace(ray),
			Shape2::Line(line) => line.trace(ray),
			Shape2::Triangle(triangle) => triangle.trace(ray),
		}
	}
}
