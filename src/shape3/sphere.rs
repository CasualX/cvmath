use super::*;

/// Sphere shape.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Sphere<T> {
	/// The center of the sphere.
	pub center: Point3<T>,

	/// The radius of the sphere.
	pub radius: T,
}

/// Sphere constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Sphere<T>(center: Point3<T>, radius: T) -> Sphere<T> {
	Sphere { center, radius }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Sphere<T> {}

impl<T> Sphere<T> {
	/// Constructs a new sphere.
	#[inline]
	pub const fn new(center: Point3<T>, radius: T) -> Sphere<T> {
		Sphere { center, radius }
	}
}

impl<T: Scalar> Sphere<T> {
	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Sphere<T>, t: T) -> Sphere<T> {
		Sphere {
			center: self.center.lerp(target.center, t),
			radius: self.radius + (target.radius - self.radius) * t,
		}
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Sphere<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<T> + urandom::Distribution<Point3<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Sphere<T> {
		let distr = urandom::distr::StandardUniform;
		let center = distr.sample(rand);
		let radius = distr.sample(rand);
		Sphere { center, radius }
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Sphere<T> {
	type Sampler = Sphere<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Sphere<T>> for Sphere<urandom::distr::Uniform<T>> where Point3<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Sphere<T>, high: Sphere<T>) -> Result<Self, urandom::distr::UniformError> {
		let center = Vec3::try_new(low.center, high.center)?;
		let radius = urandom::distr::Uniform::try_new(low.radius, high.radius)?;
		Ok(Sphere { center, radius })
	}
	#[inline]
	fn try_new_inclusive(low: Sphere<T>, high: Sphere<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let center = Vec3::try_new_inclusive(low.center, high.center)?;
		let radius = urandom::distr::Uniform::try_new_inclusive(low.radius, high.radius)?;
		Ok(Sphere { center, radius })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Sphere<T>> for Sphere<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Sphere<T> {
		let center = self.center.sample(rand);
		let radius = self.radius.sample(rand);
		Sphere { center, radius }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace3<T> for Sphere<T> {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		pt.distance_sqr(self.center) < self.radius * self.radius
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let oc = self.center - ray.origin;
		let tc = oc.dot(ray.direction);

		let d2 = oc.dot(oc) - tc * tc;
		let r2 = self.radius * self.radius;
		let t1c2 = r2 - d2;

		if !(t1c2 >= T::ZERO) {
			return None;
		}
		let t1c = t1c2.sqrt();

		let t1 = tc - t1c;
		let t2 = tc + t1c;

		if t1 > ray.distance.min && t1 <= ray.distance.max {
			let point = ray.at(t1);
			let normal = (point - self.center).norm();
			return Some(Hit3 { point, distance: t1, normal, index: 0, side: HitSide::Entry });
		}
		if t2 > ray.distance.min && t2 <= ray.distance.max {
			let point = ray.at(t2);
			let normal = (self.center - point).norm();
			return Some(Hit3 { point, distance: t2, normal, index: 0, side: HitSide::Exit });
		}
		return None;
	}
}
