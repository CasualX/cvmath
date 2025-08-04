use super::*;

/// Triangle2 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Triangle2<T> {
	/// Base point of the triangle.
	pub p: Point2<T>,
	/// First edge vector of the triangle.
	pub u: Vec2<T>,
	/// Second edge vector of the triangle.
	pub v: Vec2<T>,
}

/// Triangle2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Triangle2<T>(p: Point2<T>, u: Vec2<T>, v: Vec2<T>) -> Triangle2<T> {
	Triangle2 { p, u, v }
}

impl<T> Triangle2<T> {
	/// Constructs a new triangle.
	#[inline]
	pub const fn new(p: Point2<T>, u: Vec2<T>, v: Vec2<T>) -> Triangle2<T> {
		Triangle2 { p, u, v }
	}
}

impl<T: Copy> Triangle2<T> {
	/// Constructs a triangle from three points.
	///
	/// `p1` is the base point. The edges are computed as:
	/// - `u = p2 - p1`
	/// - `v = p3 - p1`
	pub fn points(p1: Point2<T>, p2: Point2<T>, p3: Point2<T>) -> Triangle2<T> where T: ops::Sub<Output = T> {
		let u = p2 - p1;
		let v = p3 - p1;
		Triangle2 { p: p1, u, v }
	}

	/// Returns the first point of the triangle.
	#[inline]
	pub fn p1(&self) -> Point2<T> {
		self.p
	}
	/// Returns the second point: `p + u`.
	#[inline]
	pub fn p2(&self) -> Point2<T> where T: ops::Add<T, Output = T> {
		self.p + self.u
	}
	/// Returns the third point: `p + v`.
	#[inline]
	pub fn p3(&self) -> Point2<T> where T: ops::Add<T, Output = T> {
		self.p + self.v
	}

	/// Returns the centroid of the triangle.
	#[inline]
	pub fn centroid(&self) -> Point2<T> where T: Scalar {
		let p1 = self.p;
		let p2 = self.p + self.u;
		let p3 = self.p + self.v;
		let three = T::ONE + T::ONE + T::ONE;

		(p1 + p2 + p3) / three
	}
}

impl<T: Float> Triangle2<T> {
	/// Decomposes a point relative to the triangle's local basis.
	///
	/// Returns `(x, y)` such that: `q = p + x·u + y·v`,
	#[inline]
	pub fn decompose(&self, q: Point2<T>) -> Vec2<T> {
		let u = self.u;
		let v = self.v;
		let p = self.p;

		let x = (q - p).dot(u) / u.dot(u);
		let y = (q - p).dot(v) / v.dot(v);

		Vec2(x, y)
	}

	/// Computes the barycentric coordinates of a point relative to the triangle.
	///
	/// `α`, `β`, and `γ` are the weights for vertices `p1`, `p2`, and `p3`, respectively, such that `project(q) = α·p1 + β·p2 + γ·p3`.
	#[inline]
	pub fn barycentric(&self, q: Point2<T>) -> Vec3<T> {
		let area_inv = T::ONE / (self.u.cross(self.v));

		let a = (q - self.p).cross(self.v) * area_inv;
		let b = self.u.cross(q - self.p) * area_inv;
		let c = T::ONE - a - b;

		Vec3(a, b, c)
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Triangle2<T> {
	fn inside(&self, pt: Point2<T>) -> bool {
		let a = (pt - self.p).cross(self.u);
		let b = (pt - self.p).cross(self.v);
		let c = self.u.cross(self.v);

		a >= T::ZERO && b >= T::ZERO && a + b <= c
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let edges = [
			(self.p, self.u),
			(self.p + self.u, self.v - self.u),
			(self.p + self.v, -self.v),
		];

		let mut min_hit: Option<Hit2<T>> = None;

		for &(q, edge) in edges.as_slice() {
			// Solve for intersection between ray and segment
			let p = ray.origin;
			let d = ray.direction;

			let denom = d.cross(edge);
			if denom == T::ZERO {
				continue; // Parallel to edge
			}

			let qp = q - p;
			let t = qp.cross(edge) / denom;
			let s = qp.cross(d) / denom;

			if !(t > T::EPSILON && t <= ray.distance && s >= T::ZERO && s <= T::ONE) {
				continue; // No intersection
			}

			if min_hit.is_none() || t < min_hit.as_ref().unwrap().distance {
				min_hit = Some(Hit2 {
					distance: t,
					normal: edge.cw().norm(),
					index: 0,
				});
			}
		}

		min_hit
	}
}
