use super::*;

/// Plane structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Plane<T> {
	/// The normal of the plane.
	pub normal: Vec3<T>,
	/// The distance from the origin.
	pub distance: T,
}

/// Plane constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Plane<T>(normal: Vec3<T>, distance: T) -> Plane<T> {
	Plane { normal, distance }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Plane<T> {}

impl<T> Plane<T> {
	/// Constructs a new plane.
	#[inline]
	pub const fn new(normal: Vec3<T>, distance: T) -> Plane<T> {
		Plane { normal, distance }
	}

	/// Constructs a new plane from a normal and a point.
	#[inline]
	pub fn new_alt(normal: Vec3<T>, pt: Point3<T>) -> Plane<T> where T: Float {
		let distance = -normal.dot(pt);
		Plane { normal, distance }
	}

	/// Constructs a new plane from three points.
	#[inline]
	pub fn from_pts(pt1: Point3<T>, pt2: Point3<T>, pt3: Point3<T>) -> Plane<T> where T: Float {
		let normal = (pt2 - pt1).cross(pt3 - pt1).normalize();
		let distance = -normal.dot(pt1);
		Plane { normal, distance }
	}
}

impl<T: Float> Plane<T> {
	/// Returns the projection of a point onto the plane.
	///
	/// ```
	/// use cvmath::{Plane, Point3, Vec3};
	///
	/// let plane = Plane(Vec3(0.0, 0.0, 1.0), 0.0);
	/// let pt = Point3(20.0, 10.0, 4.0);
	/// assert_eq!(plane.project_pt(pt), Point3(20.0, 10.0, 0.0));
	/// ```
	#[inline]
	pub fn project_pt(&self, pt: Point3<T>) -> Point3<T> {
		pt - self.normal * (self.normal.dot(pt) + self.distance)
	}

	/// Returns the distance from the plane to a point.
	///
	/// ```
	/// use cvmath::{Plane, Point3, Vec3};
	///
	/// let plane = Plane(Vec3(0.0, 0.0, 1.0), 0.0);
	/// let pt = Point3(20.0, 10.0, 4.0);
	/// assert_eq!(plane.dist_pt(pt), 4.0);
	/// ```
	#[inline]
	pub fn dist_pt(&self, pt: Point3<T>) -> T {
		self.normal.dot(pt) + self.distance
	}
}

//----------------------------------------------------------------

impl<T: Float> TraceRay<T> for Plane<T> {
	fn inside(&self, ray: &Ray<T>) -> bool {
		self.normal.dot(ray.origin) + self.distance < T::ZERO
	}

	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize {
		let denom = self.normal.dot(ray.direction);
		if denom.abs() < T::EPSILON {
			return 0;
		}

		let distance = -self.normal.dot(ray.origin) / denom;
		if distance < T::ZERO {
			return 0;
		}

		let mut count = 0;
		if hits.len() > count {
			hits[count] = TraceHit { distance, normal: self.normal };
			count += 1;
		}

		return count;
	}
}
