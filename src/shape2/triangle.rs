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
	/// Constructs a triangle with positive area from three points.
	#[inline]
	pub fn points(p: Point2<T>, p2: Point2<T>, p3: Point2<T>) -> Triangle2<T> where T: Scalar {
		let u = p2 - p;
		let v = p3 - p;
		Triangle2 { p, u, v }.norm()
	}

	/// Normalizes the triangle to have positive area.
	#[inline]
	pub fn norm(self) -> Triangle2<T> where T: Scalar {
		if self.u.cross(self.v) >= T::ZERO { self } else { -self }
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

	/// Returns the signed area of the triangle.
	#[inline]
	pub fn area(&self) -> T where T: Scalar {
		self.u.cross(self.v) / (T::ONE + T::ONE)
	}
}

impl<T> ops::Neg for Triangle2<T> {
	type Output = Triangle2<T>;

	#[inline]
	fn neg(self) -> Triangle2<T> {
		Triangle2 { p: self.p, u: self.v, v: self.u }
	}
}

impl<T: Scalar> Triangle2<T> {
	/// Bounds of the triangle.
	#[inline]
	pub fn bounds(&self) -> Bounds2<T> {
		let mins = self.p.min(self.p + self.u).min(self.p + self.v);
		let maxs = self.p.max(self.p + self.u).max(self.p + self.v);
		Bounds2 { mins, maxs }
	}
}

impl<T: Float> Triangle2<T> {
	/// Decomposes a point relative to the triangle's local basis.
	///
	/// Returns `(x, y)` such that: `q = p + x·u + y·v`.
	///
	/// If the triangle is degenerate (i.e., has zero area), the returned coordinates will contain infinite values.
	#[inline]
	pub fn decompose(&self, q: Point2<T>) -> Vec2<T> {
		let w = q - self.p;
		let area_inv = T::ONE / (self.u.cross(self.v));

		let x = w.cross(self.v) * area_inv;
		let y = self.u.cross(w) * area_inv;

		Vec2(x, y)
	}

	/// Computes the barycentric coordinates of a point relative to the triangle.
	///
	/// `α`, `β`, and `γ` are the weights for vertices `p1`, `p2`, and `p3`, respectively, such that `project(q) = α·p1 + β·p2 + γ·p3`.
	///
	/// If the triangle is degenerate (i.e., has zero area), the returned coordinates will contain infinite values.
	#[inline]
	pub fn barycentric(&self, q: Point2<T>) -> Vec3<T> {
		let Vec2 { x: beta, y: gamma } = self.decompose(q);
		let alpha = T::ONE - beta - gamma;

		Vec3(alpha, beta, gamma)
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Triangle2<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<Point2<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Triangle2<T> {
		let distr = urandom::distr::StandardUniform;
		let p = distr.sample(rand);
		let u = distr.sample(rand);
		let v = distr.sample(rand);
		Triangle2 { p, u, v }
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Triangle2<T> {
	type Sampler = Triangle2<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Triangle2<T>> for Triangle2<urandom::distr::Uniform<T>> where Point2<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Triangle2<T>, high: Triangle2<T>) -> Result<Self, urandom::distr::UniformError> {
		let p = Vec2::try_new(low.p, high.p)?;
		let u = Vec2::try_new(low.u, high.u)?;
		let v = Vec2::try_new(low.v, high.v)?;
		Ok(Triangle2 { p, u, v })
	}
	#[inline]
	fn try_new_inclusive(low: Triangle2<T>, high: Triangle2<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let p = Vec2::try_new_inclusive(low.p, high.p)?;
		let u = Vec2::try_new_inclusive(low.u, high.u)?;
		let v = Vec2::try_new_inclusive(low.v, high.v)?;
		Ok(Triangle2 { p, u, v })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Triangle2<T>> for Triangle2<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Triangle2<T> {
		let p = self.p.sample(rand);
		let u = self.u.sample(rand);
		let v = self.v.sample(rand);
		Triangle2 { p, u, v }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Triangle2<T> {
	fn inside(&self, pt: Point2<T>) -> bool {
		let d = pt - self.p;

		let a = self.u.cross(d);
		let b = (self.v - self.u).cross(d - self.u);
		let c = (-self.v).cross(d - self.v);

		a >= T::ZERO && b >= T::ZERO && c >= T::ZERO
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let edges = [
			(self.p, self.u),
			(self.p + self.u, self.v - self.u),
			(self.p + self.v, -self.v),
		];

		let mut min_hit: Option<Hit2<T>> = None;

		for &(q, edge) in edges.as_slice() {
			let p = ray.origin;
			let d = ray.direction;

			let denom = d.cross(edge);
			if denom == T::ZERO {
				continue; // Parallel to edge
			}

			let pq = q - p;
			let distance = pq.cross(edge) / denom;
			let s = pq.cross(d) / denom;

			if !(s >= T::ZERO && s <= T::ONE && distance > ray.distance.min && distance <= ray.distance.max) {
				continue; // No intersection
			}

			if min_hit.is_none() || distance < min_hit.as_ref().unwrap().distance {
				let point = ray.at(distance);
				let normal = edge.ccw().norm();
				let (normal, side) = if denom < T::ZERO { (normal, HitSide::Entry) } else { (-normal, HitSide::Exit) };
				min_hit = Some(Hit2 { point, distance, normal, index: 0, side });
			}
		}

		min_hit
	}
}
