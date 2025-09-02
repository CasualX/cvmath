use super::*;

/// Circle shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Circle<T> {
	/// The center of the sphere.
	pub center: Point2<T>,

	/// The radius of the sphere.
	pub radius: T,
}

/// Circle constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Circle<T>(center: Point2<T>, radius: T) -> Circle<T> {
	Circle { center, radius }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Circle<T> {}

impl<T> Circle<T> {
	/// Constructs a new circle.
	#[inline]
	pub const fn new(center: Point2<T>, radius: T) -> Circle<T> {
		Circle { center, radius }
	}
}

impl<T: Scalar> Circle<T> {
	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Circle<T>, t: T) -> Circle<T> {
		Circle {
			center: self.center.lerp(target.center, t),
			radius: self.radius + (target.radius - self.radius) * t,
		}
	}

	/// Bounds of the circle.
	#[inline]
	pub fn bounds(&self) -> Bounds2<T> {
		let half = Vec2::dup(self.radius);
		Bounds2 {
			mins: self.center - half,
			maxs: self.center + half,
		}
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Circle<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<T> + urandom::Distribution<Point2<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Circle<T> {
		let distr = urandom::distr::StandardUniform;
		let center = distr.sample(rand);
		let radius = distr.sample(rand);
		Circle { center, radius }
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Circle<T> {
	type Sampler = Circle<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Circle<T>> for Circle<urandom::distr::Uniform<T>> where Point2<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Circle<T>, high: Circle<T>) -> Result<Self, urandom::distr::UniformError> {
		let center = Vec2::try_new(low.center, high.center)?;
		let radius = urandom::distr::Uniform::try_new(low.radius, high.radius)?;
		Ok(Circle { center, radius })
	}
	#[inline]
	fn try_new_inclusive(low: Circle<T>, high: Circle<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let center = Vec2::try_new_inclusive(low.center, high.center)?;
		let radius = urandom::distr::Uniform::try_new_inclusive(low.radius, high.radius)?;
		Ok(Circle { center, radius })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Circle<T>> for Circle<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Circle<T> {
		let center = self.center.sample(rand);
		let radius = self.radius.sample(rand);
		Circle { center, radius }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Circle<T> {
	#[inline]
	fn inside(&self, pt: Point2<T>) -> bool {
		self.center.distance_sqr(pt) < self.radius * self.radius
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let oc = self.center - ray.origin;
		let tc = oc.dot(ray.direction);

		let d2 = oc.dot(oc) - tc * tc;
		let r2 = self.radius * self.radius;
		let t1c2 = r2 - d2;

		if t1c2 < T::ZERO {
			return None;
		}
		let t1c = t1c2.sqrt();

		let t1 = tc - t1c;
		let t2 = tc + t1c;

		if t1 > ray.distance.min && t1 <= ray.distance.max {
			let point = ray.at(t1);
			let normal = (point - self.center) * (T::ONE / self.radius);
			return Some(Hit2 { point, distance: t1, normal, index: 0, side: HitSide::Entry });
		}
		if t2 > ray.distance.min && t2 <= ray.distance.max {
			let point = ray.at(t2);
			let normal = (self.center - point) * (T::ONE / self.radius);
			return Some(Hit2 { point, distance: t2, normal, index: 0, side: HitSide::Exit });
		}
		return None;
	}
}
