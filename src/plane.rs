use super::*;

/// Plane shape.
#[derive(Copy, Clone, Debug, PartialEq)]
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
	pub fn from_point(normal: Vec3<T>, pt: Point3<T>) -> Plane<T> where T: Float {
		let distance = -normal.dot(pt);
		Plane { normal, distance }
	}

	/// Constructs a new plane from three points.
	///
	/// If the points are collinear, the plane normal is zero.
	#[inline]
	pub fn from_points(pt1: Point3<T>, pt2: Point3<T>, pt3: Point3<T>) -> Plane<T> where T: Float {
		let normal = (pt2 - pt1).cross(pt3 - pt1).norm();
		let distance = -normal.dot(pt1);
		Plane { normal, distance }
	}

	#[inline]
	pub fn triangle(p: Point3<T>, u: Point3<T>, v: Point3<T>) -> Plane<T> where T: Float {
		let normal = u.cross(v).norm();
		let distance = -normal.dot(p);
		Plane { normal, distance }
	}
}

impl<T: ops::Neg> ops::Neg for Plane<T> {
	type Output = Plane<T::Output>;

	#[inline]
	fn neg(self) -> Plane<T::Output> {
		Plane {
			normal: -self.normal,
			distance: -self.distance,
		}
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
	/// assert_eq!(plane.project_point(pt), Point3(20.0, 10.0, 0.0));
	/// ```
	#[inline]
	pub fn project_point(&self, pt: Point3<T>) -> Point3<T> {
		pt - self.normal * self.distance_to_point(pt)
	}

	/// Returns the signed distance from the plane to a point.
	///
	/// ```
	/// use cvmath::{Plane, Point3, Vec3};
	///
	/// let plane = Plane(Vec3(0.0, 0.0, 1.0), 0.0);
	/// let pt = Point3(20.0, 10.0, 4.0);
	/// assert_eq!(plane.distance_to_point(pt), 4.0);
	/// ```
	#[inline]
	pub fn distance_to_point(&self, pt: Point3<T>) -> T {
		self.normal.dot(pt) + self.distance
	}
}

//----------------------------------------------------------------

impl<T: Float> TraceRay<T> for Plane<T> {
	fn inside(&self, ray: &Ray<T>) -> bool {
		self.distance_to_point(ray.origin) <= T::ZERO
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

		if let Some(hit) = hits.get_mut(0) {
			*hit = TraceHit { distance, normal: self.normal, index: 0 };
		}

		return 1;
	}
}
