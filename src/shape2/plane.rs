use super::*;

/// Plane2 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Plane2<T> {
	/// The normal of the plane.
	pub normal: Vec2<T>,
	/// The distance from the origin.
	pub distance: T,
}

/// Plane2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Plane2<T>(normal: Vec2<T>, distance: T) -> Plane2<T> {
	Plane2 { normal, distance }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Plane2<T> {}

impl<T> Plane2<T> {
	/// Constructs a new plane.
	#[inline]
	pub const fn new(normal: Vec2<T>, distance: T) -> Plane2<T> {
		Plane2 { normal, distance }
	}

	/// Constructs a new plane from a normal and a point.
	#[inline]
	pub fn point(normal: Vec2<T>, pt: Point2<T>) -> Plane2<T> where T: Float {
		let distance = normal.dot(pt);
		Plane2 { normal, distance }
	}

	/// Constructs a new plane from two points.
	#[inline]
	pub fn line(pt1: Point2<T>, pt2: Point2<T>) -> Plane2<T> where T: Float {
		let normal = (pt2 - pt1).cw().norm();
		let distance = normal.dot(pt1);
		Plane2 { normal, distance }
	}
}

impl<T: ops::Neg> ops::Neg for Plane2<T> {
	type Output = Plane2<T::Output>;

	#[inline]
	fn neg(self) -> Plane2<T::Output> {
		Plane2 {
			normal: -self.normal,
			distance: -self.distance,
		}
	}
}

impl<T: Float> Plane2<T> {
	/// Signed distance to the point.
	#[inline]
	pub fn distance(self, pt: Point2<T>) -> T {
		self.normal.dot(pt) + self.distance
	}

	/// Projects the point onto the line.
	#[inline]
	pub fn project(self, pt: Point2<T>) -> Point2<T> {
		pt - self.normal * self.distance(pt)
	}

	/// Computes the y coordinate where the plane intercepts the Y axis.
	///
	/// Returns none if the plane is parallel with the Y axis.
	#[inline]
	pub fn y_intercept(self) -> Option<T> {
		if self.normal.y == T::ZERO {
			return None;
		}
		Some(-self.distance / self.normal.y)
	}

	/// Computes the x coordinate where the plane intercepts the X axis.
	///
	/// Returns none if the plane is parallel with the X axis.
	#[inline]
	pub fn x_intercept(self) -> Option<T> {
		if self.normal.x == T::ZERO {
			return None;
		}
		Some(-self.distance / self.normal.x)
	}

	/// Computes the intersection point of two planes.
	///
	/// Returns none if the planes are parallel.
	#[inline]
	pub fn intersect(self, rhs: Plane2<T>) -> Option<Point2<T>> {
		let det = self.normal.cross(rhs.normal);
		if det == T::ZERO {
			return None;
		}

		let a1 = self.normal.x;
		let b1 = self.normal.y;
		let c1 = -self.distance;

		let a2 = rhs.normal.x;
		let b2 = rhs.normal.y;
		let c2 = -rhs.distance;

		let x = (b2 * c1 - b1 * c2) / det;
		let y = (a1 * c2 - a2 * c1) / det;

		Some(Point2(x, y))
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Plane2<T> {
	#[inline]
	fn inside(&self, pt: Point2<T>) -> bool {
		self.distance(pt) >= T::ZERO
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let denom = self.normal.dot(ray.direction);

		// If denom is zero, the ray is parallel to the plane
		if denom == T::ZERO {
			return None;
		}

		// Compute the intersection distance along the ray
		let distance = -self.distance(ray.origin) / denom;
		if !(distance > ray.distance.min && distance <= ray.distance.max) {
			return None;
		}

		let point = ray.at(distance);
		let (normal, side) = if denom < T::ZERO {
			(self.normal, HitSide::Entry)
		}
		else {
			(-self.normal, HitSide::Exit)
		};

		Some(Hit2 { point, distance, normal, index: 0, side })
	}
}
