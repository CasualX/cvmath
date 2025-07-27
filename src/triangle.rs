use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Triangle<T> {
	/// Base point of the triangle.
	pub p: T,
	/// First edge vector of the triangle.
	pub u: T,
	/// Second edge vector of the triangle.
	pub v: T,
}

/// Triangle constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Triangle<T>(p: T, u: T, v: T) -> Triangle<T> {
	Triangle { p, u, v }
}

impl<T: Copy> Triangle<T> {
	/// Constructs a new triangle.
	#[inline]
	pub const fn new(p: T, u: T, v: T) -> Triangle<T> {
		Triangle { p, u, v }
	}

	pub fn points(p1: T, p2: T, p3: T) -> Triangle<T> where T: ops::Sub<Output = T>{
		let u = p2 - p1;
		let v = p3 - p1;
		Triangle { p: p1, u, v }
	}

	#[inline]
	pub fn p1(&self) -> T {
		self.p
	}
	#[inline]
	pub fn p2(&self) -> T where T: ops::Add<T, Output = T> {
		self.p + self.u
	}
	#[inline]
	pub fn p3(&self) -> T where T: ops::Add<T, Output = T> {
		self.p + self.v
	}
}

pub type Triangle2<T> = Triangle<Point2<T>>;

pub type Triangle3<T> = Triangle<Point3<T>>;

impl<T: Float> Triangle3<T> {
	#[inline]
	pub fn plane(&self) -> Plane<T> {
		Plane::triangle(self.p, self.u, self.v)
	}
}

impl<T: Float> TraceRay<T> for Triangle3<T> {
	#[inline]
	fn inside(&self, ray: &Ray<T>) -> bool {
		Plane::triangle(self.p, self.u, self.v).inside(ray)
	}

	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize {
		let h = ray.direction.cross(self.v);
		let a = self.u.dot(h);

		if a.abs() < T::EPSILON {
			// Ray is parallel to the triangle
			return 0;
		}

		let f = T::ONE / a;
		let s = ray.origin - self.p;
		let u = f * s.dot(h);

		if u < T::ZERO || u > T::ONE {
			return 0;
		}

		let q = s.cross(self.u);
		let v = f * ray.direction.dot(q);

		if v < T::ZERO || u + v > T::ONE {
			return 0;
		}

		let distance = f * self.v.dot(q);

		if distance < T::EPSILON {
			return 0;
		}

		if let Some(hit) = hits.get_mut(0) {
			let normal = Plane::triangle(self.p, self.u, self.v).normal;
			*hit = TraceHit {
				distance,
				normal,
				index: 0,
			};
		}

		return 1;
	}
}
