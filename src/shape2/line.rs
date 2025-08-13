use super::*;

/// Line2 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Line2<T> {
	pub start: Point2<T>,
	pub end: Point2<T>,
}

/// Line2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Line2<T>(start: Point2<T>, end: Point2<T>) -> Line2<T> {
	Line2 { start, end }
}

specialized_type!(Line2, Line2f, f32, start: Point2f, end: Point2f);
specialized_type!(Line2, Line2d, f64, start: Point2d, end: Point2d);
specialized_type!(Line2, Line2i, i32, start: Point2i, end: Point2i);

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Line2<T> {}

impl<T> Line2<T> {
	/// Constructs a new line.
	#[inline]
	pub const fn new(start: Point2<T>, end: Point2<T>) -> Line2<T> {
		Line2 { start, end }
	}

	/// Pinches the line at the given point.
	#[inline]
	pub const fn pinch(self, pt: Point2<T>) -> (Line2<T>, Line2<T>) where T: Copy {
		let Line2 { start, end } = self;
		(Line2::new(start, pt), Line2::new(pt, end))
	}

	/// Line direction.
	#[inline]
	pub fn delta(self) -> Vec2<T> where T: ops::Sub<Output = T> {
		self.end - self.start
	}
}

impl<T: Scalar> Line2<T> {
	/// Bounds of the line.
	#[inline]
	pub fn bounds(&self) -> Bounds2<T> {
		let (mins, maxs) = self.start.min_max(self.end);
		Bounds2 { mins, maxs }
	}
}

impl<T: Float> Line2<T> {
	/// Projects the point onto the line segment.
	#[inline]
	pub fn project(self, pt: Point2<T>) -> Point2<T> {
		self.start + (pt - self.start).project_sat(self.end - self.start)
	}

	/// Point to line segment distance.
	#[inline]
	pub fn distance(self, pt: Point2<T>) -> T {
		self.project(pt).distance(pt)
	}

	/// Intersect a line and line segment.
	///
	/// The result is scalar with which to scale the segment to find the intersection point, none if the line and line segment are parallel.
	///
	/// To test if the line segment actually intersects the line, check if this result lies inside the [0; 1] range.
	/// To calculate the intersection point scale the segment by this function's result.
	///
	/// ```
	/// use cvmath::{Line2, Point2};
	///
	/// let line = Line2(Point2(1.0, 1.0), Point2(2.0, 2.0));
	/// let segment = Line2(Point2(-1.0, 1.0), Point2(1.0, -1.0));
	///
	/// let result = Line2::segment_x(line, segment);
	/// assert_eq!(result, Some(0.5));
	///
	/// let x = segment.start + (segment.end - segment.start) * result.unwrap();
	/// assert_eq!(x, Point2(0.0, 0.0));
	/// ```
	#[inline]
	pub fn segment_x(self, rhs: Line2<T>) -> Option<T> {
		let r = self.end - self.start;
		let s = rhs.end - rhs.start;

		let denom = r.cross(s);
		if denom == T::ZERO {
			return None;
		}

		let u = (rhs.start - self.start).cross(r) / denom;
		Some(u)
	}

	/// Intersect two lines.
	///
	/// The result is some point if the two lines intersect, none if they are parallel.
	///
	/// ```
	/// use cvmath::{Line2, Point2};
	///
	/// let line1 = Line2(Point2(1.0, 1.0), Point2(2.0, 2.0));
	/// let line2 = Line2(Point2(-1.0, 1.0), Point2(1.0, -1.0));
	///
	/// let result = Line2::intersect(line1, line2);
	///
	/// assert_eq!(result, Some(Point2(0.0, 0.0)));
	/// ```
	#[inline]
	pub fn intersect(self, rhs: Line2<T>) -> Option<Point2<T>> {
		let denom = self.delta().cross(rhs.delta());
		if denom == T::ZERO {
			return None;
		}

		let p = rhs.delta() * self.start.cross(self.start + self.delta()) - self.delta() * rhs.start.cross(rhs.start + rhs.delta());
		Some(p / denom)
	}

	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Line2<T>, t: T) -> Line2<T> {
		Line2 {
			start: self.start.lerp(target.start, t),
			end: self.end.lerp(target.end, t),
		}
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Line2<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<Point2<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Line2<T> {
		let distr = urandom::distr::StandardUniform;
		let start = distr.sample(rand);
		let end = distr.sample(rand);
		Line2 { start, end }
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Line2<T> {
	type Sampler = Line2<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Line2<T>> for Line2<urandom::distr::Uniform<T>> where Point2<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Line2<T>, high: Line2<T>) -> Result<Self, urandom::distr::UniformError> {
		let start = Vec2::try_new(low.start, high.start)?;
		let end = Vec2::try_new(low.end, high.end)?;
		Ok(Line2 { start, end })
	}
	#[inline]
	fn try_new_inclusive(low: Line2<T>, high: Line2<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let start = Vec2::try_new_inclusive(low.start, high.start)?;
		let end = Vec2::try_new_inclusive(low.end, high.end)?;
		Ok(Line2 { start, end })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Line2<T>> for Line2<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Line2<T> {
		let start = self.start.sample(rand);
		let end = self.end.sample(rand);
		Line2 { start, end }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Line2<T> {
	// Line has no inherent orientation
	#[inline]
	fn inside(&self, _pt: Point2<T>) -> bool {
		false
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let delta = self.end - self.start;
		let denom = ray.direction.cross(delta);

		// Parallel => no intersection
		if denom == T::ZERO {
			return None;
		}

		let qp = self.start - ray.origin;
		let distance = qp.cross(delta) / denom;
		let u = qp.cross(ray.direction) / denom;

		if !(distance > T::EPSILON && distance <= ray.distance && u >= T::ZERO && u <= T::ONE) {
			return None;
		}

		// Line has no inherent orientation, flip normal to always face ray.origin
		let mut normal = delta.norm().ccw();
		if normal.dot(ray.direction) < T::ZERO {
			normal = -normal;
		}

		Some(Hit2 { distance, normal, index: 0 })
	}
}
