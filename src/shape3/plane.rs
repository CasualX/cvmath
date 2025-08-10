use super::*;

/// Plane3 shape.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Plane3<T> {
	/// The normal of the plane.
	pub normal: Vec3<T>,
	/// The distance from the origin.
	pub distance: T,
}

/// Plane3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Plane3<T>(normal: Vec3<T>, distance: T) -> Plane3<T> {
	Plane3 { normal, distance }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Plane3<T> {}

impl<T> Plane3<T> {
	/// Constructs a new plane.
	#[inline]
	pub const fn new(normal: Vec3<T>, distance: T) -> Plane3<T> {
		Plane3 { normal, distance }
	}

	/// Constructs a new plane from a normal and a point.
	#[inline]
	pub fn point(normal: Vec3<T>, pt: Point3<T>) -> Plane3<T> where T: Float {
		let distance = normal.dot(pt);
		Plane3 { normal, distance }
	}

	/// Constructs a new plane from three points.
	///
	/// If the points are collinear, the plane normal is zero.
	#[inline]
	pub fn triangle(pt1: Point3<T>, pt2: Point3<T>, pt3: Point3<T>) -> Plane3<T> where T: Float {
		let normal = (pt2 - pt1).cross(pt3 - pt1).norm();
		let distance = normal.dot(pt1);
		Plane3 { normal, distance }
	}
}

impl<T: ops::Neg> ops::Neg for Plane3<T> {
	type Output = Plane3<T::Output>;

	#[inline]
	fn neg(self) -> Plane3<T::Output> {
		Plane3 {
			normal: -self.normal,
			distance: -self.distance,
		}
	}
}

impl<T: Float> Plane3<T> {
	/// Returns the signed distance from the plane to a point.
	///
	/// Positive means "inside" (in the direction the normal points).
	///
	/// ```
	/// use cvmath::{Plane3, Point3, Vec3};
	///
	/// let plane = Plane3(Vec3(0.0, 0.0, 1.0), 0.0);
	/// let pt = Point3(20.0, 10.0, 4.0);
	/// assert_eq!(plane.distance(pt), 4.0);
	/// ```
	#[inline]
	pub fn distance(&self, pt: Point3<T>) -> T {
		self.normal.dot(pt) + self.distance
	}

	/// Returns the projection of a point onto the plane.
	///
	/// ```
	/// use cvmath::{Plane3, Point3, Vec3};
	///
	/// let plane = Plane3(Vec3(0.0, 0.0, 1.0), 0.0);
	/// let pt = Point3(20.0, 10.0, 4.0);
	/// assert_eq!(plane.project(pt), Point3(20.0, 10.0, 0.0));
	/// ```
	#[inline]
	pub fn project(&self, pt: Point3<T>) -> Point3<T> {
		pt - self.normal * self.distance(pt)
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace3<T> for Plane3<T> {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		self.distance(pt) >= T::ZERO
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let denom = self.normal.dot(ray.direction);
		if denom == T::ZERO {
			return None;
		}

		let distance = -self.distance(ray.origin) / denom;
		if !(distance > T::EPSILON && distance <= ray.distance) {
			return None;
		}

		Some(Hit3 { distance, normal: self.normal, index: 0 })
	}
}
