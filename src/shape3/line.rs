use super::*;

/// Line3 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Line3<T> {
	pub start: Point3<T>,
	pub end: Point3<T>,
}

/// Line3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Line3<T>(start: Point3<T>, end: Point3<T>) -> Line3<T> {
	Line3 { start, end }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Line3<T> {}

impl<T> Line3<T> {
	/// Constructs a new line.
	#[inline]
	pub const fn new(start: Point3<T>, end: Point3<T>) -> Line3<T> {
		Line3 { start, end }
	}

	/// Pinches the line at the given point.
	#[inline]
	pub const fn pinch(self, pt: Point3<T>) -> (Line3<T>, Line3<T>) where T: Copy {
		let Line3 { start, end } = self;
		(Line3::new(start, pt), Line3::new(pt, end))
	}
}

impl<T: ops::Sub<Output = T>> Line3<T> {
	/// Line direction.
	#[inline]
	pub fn direction(self) -> Vec3<T> {
		self.end - self.start
	}
}

impl<T: Float> Line3<T> {
	/// Projects the point onto the line.
	#[inline]
	pub fn project(self, pt: Point3<T>) -> Point3<T> {
		self.start + (pt - self.start).project(self.direction())
	}

	/// Point to line distance.
	#[inline]
	pub fn distance(self, pt: Point3<T>) -> T {
		self.project(pt).distance(pt)
	}

	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Line3<T>, t: T) -> Line3<T> {
		Line3 {
			start: self.start.lerp(target.start, t),
			end: self.end.lerp(target.end, t),
		}
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Line3<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<Point3<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Line3<T> {
		let distr = urandom::distr::StandardUniform;
		let start = distr.sample(rand);
		let end = distr.sample(rand);
		Line3 { start, end }
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Line3<T> {
	type Sampler = Line3<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Line3<T>> for Line3<urandom::distr::Uniform<T>> where Point3<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Line3<T>, high: Line3<T>) -> Result<Self, urandom::distr::UniformError> {
		let start = Vec3::try_new(low.start, high.start)?;
		let end = Vec3::try_new(low.end, high.end)?;
		Ok(Line3 { start, end })
	}
	#[inline]
	fn try_new_inclusive(low: Line3<T>, high: Line3<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let start = Vec3::try_new_inclusive(low.start, high.start)?;
		let end = Vec3::try_new_inclusive(low.end, high.end)?;
		Ok(Line3 { start, end })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Line3<T>> for Line3<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Line3<T> {
		let start = self.start.sample(rand);
		let end = self.end.sample(rand);
		Line3 { start, end }
	}
}

//----------------------------------------------------------------

// Lines are not solid
impl<T: Float> Trace3<T> for Line3<T> {
	#[inline]
	fn inside(&self, _pt: Point3<T>) -> bool {
		false
	}

	#[inline]
	fn trace(&self, _ray: &Ray3<T>) -> Option<Hit3<T>> {
		None
	}
}
