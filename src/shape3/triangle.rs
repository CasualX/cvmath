use super::*;

/// Triangle3 shape.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Triangle3<T> {
	/// Base point of the triangle.
	pub p: Point3<T>,
	/// First edge vector of the triangle.
	pub u: Vec3<T>,
	/// Second edge vector of the triangle.
	pub v: Vec3<T>,
}

/// Triangle3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Triangle3<T>(p: Point3<T>, u: Vec3<T>, v: Vec3<T>) -> Triangle3<T> {
	Triangle3 { p, u, v }
}

impl<T> Triangle3<T> {
	/// Constructs a new triangle.
	#[inline]
	pub const fn new(p: Point3<T>, u: Vec3<T>, v: Vec3<T>) -> Triangle3<T> {
		Triangle3 { p, u, v }
	}
}

impl<T: Copy> Triangle3<T> {
	/// Constructs a triangle with positive area from three points.
	pub fn points(p: Point3<T>, p2: Point3<T>, p3: Point3<T>) -> Triangle3<T> where T: Scalar {
		let u = p2 - p;
		let v = p3 - p;
		Triangle3 { p, u, v }.norm()
	}

	/// Normalizes the triangle to have positive area.
	#[inline]
	pub fn norm(self) -> Triangle3<T> where T: Scalar {
		if self.u.cross(self.v).len_sqr() >= T::ZERO { self } else { -self }
	}

	/// Returns the first point of the triangle.
	#[inline]
	pub fn p1(&self) -> Point3<T> {
		self.p
	}
	/// Returns the second point: `p + u`.
	#[inline]
	pub fn p2(&self) -> Point3<T> where T: ops::Add<T, Output = T> {
		self.p + self.u
	}
	/// Returns the third point: `p + v`.
	#[inline]
	pub fn p3(&self) -> Point3<T> where T: ops::Add<T, Output = T> {
		self.p + self.v
	}

	/// Returns the centroid of the triangle.
	#[inline]
	pub fn centroid(&self) -> Point3<T> where T: Scalar {
		let p1 = self.p;
		let p2 = self.p + self.u;
		let p3 = self.p + self.v;
		let three = T::ONE + T::ONE + T::ONE;

		(p1 + p2 + p3) / three
	}

	/// Returns the signed area of the triangle.
	#[inline]
	pub fn area(&self) -> T where T: Float {
		self.u.cross(self.v).len() / (T::ONE + T::ONE)
	}
}

impl<T> ops::Neg for Triangle3<T> {
	type Output = Triangle3<T>;

	#[inline]
	fn neg(self) -> Triangle3<T> {
		Triangle3 { p: self.p, u: self.v, v: self.u }
	}
}

impl<T: Float> Triangle3<T> {
	/// Returns the plane defined by the triangle.
	#[inline]
	pub fn plane(&self) -> Plane3<T> {
		let normal = -self.normal();
		let distance = -normal.dot(self.p);
		Plane3 { normal, distance }
	}

	/// Returns the normal vector of the triangle.
	///
	/// The normal is computed as normalized `u × v`.
	///
	/// It points outward, following the right-hand rule:
	/// index finger along `u`, middle finger along `v`, thumb points in the direction of the normal.
	#[inline]
	pub fn normal(&self) -> Vec3<T> {
		self.u.cross(self.v).norm()
	}

	/// Decomposes a point relative to the triangle's local basis.
	///
	/// Returns `(x, y, z)` such that: `q = p + x·u + y·v + z·n`,
	/// where `n` is the normalized normal vector of the triangle.
	///
	/// The `z` component represents the signed distance from `q` to the triangle's plane.
	#[inline]
	pub fn decompose(&self, q: Vec3<T>) -> Vec3<T> {
		let d = q - self.p;
		let n = self.u.cross(self.v).norm();
		let a = Mat3::compose(self.u, self.v, n);
		a.inverse() * d
	}

	/// Computes the barycentric coordinates of a point relative to the triangle.
	///
	/// `α`, `β`, and `γ` are the weights for vertices `p1`, `p2`, and `p3`, respectively, such that `project(q) = α·p1 + β·p2 + γ·p3`.
	#[inline]
	pub fn barycentric(&self, q: Vec3<T>) -> Vec3<T> {
		let d = q - self.p;
		let n = self.u.cross(self.v);
		let a = Mat3::compose(self.u, self.v, n);
		let Vec3 { x, y, .. } = a.inverse() * d;
		Vec3(T::ONE - x - y, x, y)
	}
}

impl<T: Float> ops::Mul<Triangle3<T>> for Transform3<T> {
	type Output = Triangle3<T>;

	#[inline]
	fn mul(self, triangle: Triangle3<T>) -> Triangle3<T> {
		let p = self * triangle.p;
		let u = self.mat3() * triangle.u;
		let v = self.mat3() * triangle.v;
		Triangle3 { p, u, v }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace3<T> for Triangle3<T> {
	#[inline]
	fn inside(&self, _pt: Point3<T>) -> bool {
		false
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let h = ray.direction.cross(self.v);
		let a = self.u.dot(h);

		// Ray is parallel to the triangle
		if a == T::ZERO {
			return None;
		}

		let f = T::ONE / a;
		let s = ray.origin - self.p;
		let u = f * s.dot(h);

		if !(u >= T::ZERO && u <= T::ONE) {
			return None;
		}

		let q = s.cross(self.u);
		let v = f * ray.direction.dot(q);

		if !(v >= T::ZERO && u + v <= T::ONE) {
			return None;
		}

		let t0 = f * self.v.dot(q);

		if !(t0 > T::EPSILON && t0 <= ray.distance) {
			return None;
		}

		let normal = self.normal();
		Some(Hit3 {
			distance: t0,
			normal,
			index: 0,
		})
	}
}
