use super::*;

/// Bounds2 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Bounds2<T> {
	pub mins: Point2<T>,
	pub maxs: Point2<T>,
}

/// Bounds2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Bounds2<T>(mins: Point2<T>, maxs: Point2<T>) -> Bounds2<T> {
	Bounds2 { mins, maxs }
}

specialized_type!(Bounds2, Bounds2f, f32, mins: Point2f, maxs: Point2f);
specialized_type!(Bounds2, Bounds2d, f64, mins: Point2d, maxs: Point2d);
specialized_type!(Bounds2, Bounds2i, i32, mins: Point2i, maxs: Point2i);

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Bounds2<T> {}

impl<T: Zero> Bounds2<T> {
	/// Zero bounds.
	pub const ZERO: Bounds2<T> = Bounds2 { mins: Point2::ZERO, maxs: Point2::ZERO };
}
impl<T: Zero + One> Bounds2<T> {
	/// Unit bounds.
	pub const UNIT: Bounds2<T> = Bounds2 { mins: Point2::ZERO, maxs: Point2::ONE };
}

impl<T> Bounds2<T> {
	/// Constructs a new bounds.
	#[inline]
	pub const fn new(mins: Point2<T>, maxs: Point2<T>) -> Bounds2<T> {
		Bounds2 { mins, maxs }
	}
	/// Bounds from the origin to the vector.
	#[inline]
	pub fn vec(vec: Vec2<T>) -> Bounds2<T> where T: Default {
		Bounds2 {
			mins: Point2::default(),
			maxs: vec,
		}
	}
	/// Creates a bounds at the given point with size.
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let p = Point2(3, -4);
	/// let size = Point2(1, 1);
	/// let bounds = Bounds2::point(p, size);
	/// assert_eq!(bounds.mins, p - size);
	/// assert_eq!(bounds.maxs, p + size);
	/// ```
	#[inline]
	pub fn point(point: Point2<T>, size: Vec2<T>) -> Bounds2<T> where T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> {
		Bounds2 { mins: point - size, maxs: point + size }
	}
	/// Bounds2 constructor from components.
	#[inline]
	pub const fn c(mins_x: T, mins_y: T, maxs_x: T, maxs_y: T) -> Bounds2<T> {
		let mins = Point2 { x: mins_x, y: mins_y };
		let maxs = Point2 { x: maxs_x, y: maxs_y };
		Bounds2 { mins, maxs }
	}
	/// Casts the bounds to a different unit type.
	#[inline]
	pub fn cast<U>(self) -> Bounds2<U> where T: CastTo<U> {
		Bounds2 {
			mins: self.mins.cast(),
			maxs: self.maxs.cast(),
		}
	}
	/// Normalizes the min and max values ensuring that `self.mins <= self.maxs`.
	///
	/// Because the constructors don't implicitly do this for you,
	/// it is typical to have this call follow the construction of the bounds.
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let bounds = Bounds2(Point2(1, 0), Point2(-2, 3)).norm();
	/// assert_eq!(Bounds2(Point2(-2, 0), Point2(1, 3)), bounds);
	/// ```
	#[inline]
	pub fn norm(self) -> Bounds2<T> where T: Extrema {
		let (mins, maxs) = self.mins.min_max(self.maxs);
		Bounds2 { mins, maxs }
	}
	/// Returns the size of the bounds.
	///
	/// ```
	/// use cvmath::{Bounds2, Point2, Vec2};
	///
	/// let bounds = Bounds2(Point2(1, 2), Point2(3, 1));
	/// assert_eq!(Vec2(2, -1), bounds.size());
	/// ```
	#[inline]
	pub fn size(self) -> Vec2<T> where T: ops::Sub<Output = T> {
		self.maxs - self.mins
	}
}

impl<T> Bounds2<T> {
	/// Returns whether the point `rhs` is contained within `self`.
	///
	/// <!--CONTAINS-->
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let bounds = Bounds2(Point2(1, 1), Point2(4, 3));
	/// assert!(bounds.contains(Point2(1, 1)));
	/// assert!(bounds.contains(Point2(3, 2)));
	///
	/// assert!(!bounds.contains(Point2(0, 0)));
	/// ```
	#[inline]
	pub fn contains(&self, rhs: Point2<T>) -> bool where T: PartialOrd {
		rhs.spatial_ge(&self.mins) && rhs.spatial_le(&self.maxs)
	}
	/// Returns whether the bounds `rhs` is fully contained within `self`.
	///
	/// <!--ENCLOSES-->
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let bounds = Bounds2(Point2(1, 1), Point2(4, 3));
	/// let small = Bounds2(Point2(2, 2), Point2(3, 3));
	/// assert!(bounds.encloses(small));
	/// assert!(!small.encloses(bounds));
	///
	/// let overlap = Bounds2(Point2(2, 0), Point2(3, 2));
	/// assert!(!bounds.encloses(overlap));
	/// assert!(!overlap.encloses(bounds));
	/// ```
	#[inline]
	pub fn encloses(&self, rhs: Bounds2<T>) -> bool where T: PartialOrd + fmt::Debug {
		rhs.mins.spatial_ge(&self.mins) && rhs.maxs.spatial_le(&self.maxs)
	}
	/// Returns whether `rhs` is overlapped with `self`.
	///
	/// <!--OVERLAPS-->
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let bounds = Bounds2(Point2(1, 1), Point2(4, 3));
	/// let overlap = Bounds2(Point2(2, 0), Point2(3, 2));
	/// assert!(bounds.overlaps(overlap));
	/// assert!(overlap.overlaps(bounds));
	/// ```
	#[inline]
	pub fn overlaps(&self, rhs: Bounds2<T>) -> bool where T: PartialOrd {
		rhs.maxs.spatial_ge(&self.mins) && rhs.mins.spatial_le(&self.maxs)
	}
	/// Includes the point in the bounds.
	pub fn include(self, pt: Point2<T>) -> Bounds2<T> where T: Copy + Extrema {
		let mins = self.mins.min(pt);
		let maxs = self.maxs.max(pt);
		Bounds2 { mins, maxs }
	}
	/// Returns the new bounds containing both `rhs` and `self`.
	///
	/// <!--UNION-->
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let bounds = Bounds2(Point2(1, 1), Point2(4, 3));
	/// let other = Bounds2(Point2(2, 0), Point2(3, 2));
	/// let result = Bounds2(Point2(1, 0), Point2(4, 3));
	///
	/// assert_eq!(result, bounds.union(other));
	/// assert_eq!(result, other.union(bounds));
	/// ```
	#[inline]
	pub fn union(self, rhs: Bounds2<T>) -> Bounds2<T> where T: Extrema {
		let mins = self.mins.min(rhs.mins);
		let maxs = self.maxs.max(rhs.maxs);
		Bounds2 { mins, maxs }
	}
	/// Returns the overlapping area (if any) between `rhs` and `self`.
	///
	/// <!--INTERSECT-->
	///
	/// ```
	/// use cvmath::{Bounds2, Point2};
	///
	/// let bounds = Bounds2(Point2(1, 1), Point2(4, 3));
	/// let other = Bounds2(Point2(2, 0), Point2(3, 2));
	/// let result = Bounds2(Point2(2, 1), Point2(3, 2));
	///
	/// assert_eq!(Some(result), bounds.intersect(other));
	/// assert_eq!(Some(result), other.intersect(bounds));
	///
	/// let touching = Bounds2(Point2(0, 1), Point2(1, 3));
	/// let result = Bounds2(Point2(1, 1), Point2(1, 3));
	///
	/// assert_eq!(Some(result), bounds.intersect(touching));
	/// assert_eq!(Some(result), touching.intersect(bounds));
	///
	/// let negative = Bounds2(Point2(-1, -1), Point2(0, 0));
	/// assert_eq!(None, bounds.intersect(negative));
	/// assert_eq!(None, negative.intersect(bounds));
	/// ```
	#[inline]
	pub fn intersect(self, rhs: Bounds2<T>) -> Option<Bounds2<T>> where T: PartialOrd + Extrema {
		let mins = self.mins.max(rhs.mins);
		let maxs = self.maxs.min(rhs.maxs);
		if mins.spatial_le(&maxs) {
			Some(Bounds2 { mins, maxs })
		}
		else {
			None
		}
	}
}
impl<T> Bounds2<T> {
	/// Returns whether `rhs` is strictly contained within `self`.
	///
	/// <!--STRICTLY_CONTAINS-->
	#[inline]
	pub fn strictly_contains(&self, rhs: Point2<T>) -> bool where T: PartialOrd {
		rhs.spatial_gt(&self.mins) && rhs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly contained within `self`.
	///
	/// <!--STRICTLY_ENCLOSES-->
	#[inline]
	pub fn strictly_encloses(&self, rhs: Bounds2<T>) -> bool where T: PartialOrd {
		rhs.mins.spatial_gt(&self.mins) && rhs.maxs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly overlapped with `self`.
	///
	/// <!--STRICTLY_OVERLAPS-->
	#[inline]
	pub fn strictly_overlaps(&self, rhs: Bounds2<T>) -> bool where T: PartialOrd {
		rhs.maxs.spatial_gt(&self.mins) && rhs.mins.spatial_lt(&self.maxs)
	}
	/// Returns the overlapping area (not empty) between `rhs` and `self`.
	///
	/// <!--STRICTLY_INTERSECT-->
	#[inline]
	pub fn strictly_intersect(self, rhs: Bounds2<T>) -> Option<Bounds2<T>> where T: PartialOrd + Extrema {
		let mins = self.mins.max(rhs.mins);
		let maxs = self.maxs.min(rhs.maxs);
		if mins.spatial_lt(&maxs) {
			Some(Bounds2 { mins, maxs })
		}
		else {
			None
		}
	}
}

impl<T: Copy + ops::Add<T, Output = T>> ops::Add<Vec2<T>> for Bounds2<T> {
	type Output = Bounds2<T>;
	#[inline]
	fn add(self, rhs: Vec2<T>) -> Bounds2<T> {
		Bounds2 {
			mins: self.mins + rhs,
			maxs: self.maxs + rhs,
		}
	}
}
impl<T: Copy + ops::Sub<T, Output = T>> ops::Sub<Vec2<T>> for Bounds2<T> {
	type Output = Bounds2<T>;
	#[inline]
	fn sub(self, rhs: Vec2<T>) -> Bounds2<T> {
		Bounds2 {
			mins: self.mins - rhs,
			maxs: self.maxs - rhs,
		}
	}
}
impl<T: Copy + ops::AddAssign<T>> ops::AddAssign<Vec2<T>> for Bounds2<T> {
	#[inline]
	fn add_assign(&mut self, rhs: Vec2<T>) {
		self.mins += rhs;
		self.maxs += rhs;
	}
}
impl<T: Copy + ops::SubAssign<T>> ops::SubAssign<Vec2<T>> for Bounds2<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: Vec2<T>) {
		self.mins -= rhs;
		self.maxs -= rhs;
	}
}

impl<T> AsRef<[Point2<T>; 2]> for Bounds2<T> {
	#[inline]
	fn as_ref(&self) -> &[Point2<T>; 2] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> AsMut<[Point2<T>; 2]> for Bounds2<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [Point2<T>; 2] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> From<[Point2<T>; 2]> for Bounds2<T> {
	#[inline]
	fn from([mins, maxs]: [Point2<T>; 2]) -> Bounds2<T> {
		Bounds2 { mins, maxs }
	}
}
impl<T> From<Bounds2<T>> for [Point2<T>; 2] {
	#[inline]
	fn from(bounds: Bounds2<T>) -> [Point2<T>; 2] {
		[bounds.mins, bounds.maxs]
	}
}

//----------------------------------------------------------------

impl<T: Scalar> Bounds2<T> {
	/// X coordinate of the left side.
	#[inline]
	pub fn left(&self) -> T {
		self.mins.x
	}
	/// X coordinate of the right side.
	#[inline]
	pub fn right(&self) -> T {
		self.maxs.x
	}
	/// Y coordinate of the top side.
	#[inline]
	pub fn top(&self) -> T {
		self.mins.y
	}
	/// Y coordinate of the bottom side.
	#[inline]
	pub fn bottom(&self) -> T {
		self.maxs.y
	}
	/// Width of the rectangle.
	#[inline]
	pub fn width(&self) -> T {
		self.maxs.x - self.mins.x
	}
	/// Height of the rectangle.
	#[inline]
	pub fn height(&self) -> T {
		self.maxs.y - self.mins.y
	}
	/// Area of the rectangle.
	#[inline]
	pub fn area(&self) -> T {
		(self.maxs.x - self.mins.x) * (self.maxs.y - self.mins.y)
	}
	/// Center of the rectangle.
	#[inline]
	pub fn center(&self) -> Point2<T> {
		(self.mins + self.maxs) / (T::ONE + T::ONE)
	}
	/// Top left corner of the rectangle.
	#[inline]
	pub fn top_left(&self) -> Point2<T> {
		self.mins
	}
	/// Top right corner of the rectangle.
	#[inline]
	pub fn top_right(&self) -> Point2<T> {
		Point2 { x: self.maxs.x, y: self.mins.y }
	}
	/// Bottom left corner of the rectangle.
	#[inline]
	pub fn bottom_left(&self) -> Point2<T> {
		Point2 { x: self.mins.x, y: self.maxs.y }
	}
	/// Bottom right corner of the rectangle.
	#[inline]
	pub fn bottom_right(&self) -> Point2<T> {
		self.maxs
	}
	/// Top side of the rectangle.
	#[inline]
	pub fn top_side(&self) -> Line2<T> {
		Line2 {
			start: self.top_left(),
			end: self.top_right(),
		}
	}
	/// Right side of the rectangle.
	#[inline]
	pub fn right_side(&self) -> Line2<T> {
		Line2 {
			start: self.top_right(),
			end: self.bottom_right(),
		}
	}
	/// Bottom side of the rectangle.
	#[inline]
	pub fn bottom_side(&self) -> Line2<T> {
		Line2 {
			start: self.bottom_right(),
			end: self.bottom_left(),
		}
	}
	/// Left side of the rectangle.
	#[inline]
	pub fn left_side(&self) -> Line2<T> {
		Line2 {
			start: self.bottom_left(),
			end: self.top_left(),
		}
	}
	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Bounds2<T>, t: T) -> Bounds2<T> where T: Scalar {
		Bounds2 {
			mins: self.mins.lerp(target.mins, t),
			maxs: self.maxs.lerp(target.maxs, t),
		}
	}
	/// Transform of the unit square.
	#[inline]
	pub fn transform(self) -> Transform2<T> {
		Transform2::compose(
			Vec2(self.width(), T::ZERO),
			Vec2(T::ZERO, self.height()),
			self.mins,
		)
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T: Scalar> urandom::Distribution<Bounds2<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<Point2<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Bounds2<T> {
		let distr = urandom::distr::StandardUniform;
		let mins = distr.sample(rand);
		let maxs = distr.sample(rand);
		Bounds2 { mins, maxs }.norm()
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Bounds2<T> {
	type Sampler = Bounds2<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Bounds2<T>> for Bounds2<urandom::distr::Uniform<T>> where Point2<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Bounds2<T>, high: Bounds2<T>) -> Result<Self, urandom::distr::UniformError> {
		let mins = Vec2::try_new(low.mins, high.mins)?;
		let maxs = Vec2::try_new(low.maxs, high.maxs)?;
		Ok(Bounds2 { mins, maxs })
	}
	#[inline]
	fn try_new_inclusive(low: Bounds2<T>, high: Bounds2<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let mins = Vec2::try_new_inclusive(low.mins, high.mins)?;
		let maxs = Vec2::try_new_inclusive(low.maxs, high.maxs)?;
		Ok(Bounds2 { mins, maxs })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Bounds2<T>> for Bounds2<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Bounds2<T> {
		let mins = self.mins.sample(rand);
		let maxs = self.maxs.sample(rand);
		Bounds2 { mins, maxs }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Bounds2<T> {
	#[inline]
	fn inside(&self, pt: Point2<T>) -> bool {
		self.contains(pt)
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let inv_dir = ray.direction.map(|d| T::ONE / d);

		let tmin = (self.mins - ray.origin) * inv_dir;
		let tmax = (self.maxs - ray.origin) * inv_dir;
		let (tmin, tmax) = tmin.min_max(tmax);

		let t0 = tmin.vmax();
		let t1 = tmax.vmin();

		if !(t0 <= t1 && t0 > T::EPSILON && t0 <= ray.distance) {
			return None;
		}

		// Determine which face was hit
		let normal = if t0 == tmin.x {
			Vec2::new(-T::ONE, T::ZERO)
		}
		else if t0 == tmax.x {
			Vec2::new(T::ONE, T::ZERO)
		}
		else if t0 == tmin.y {
			Vec2::new(T::ZERO, -T::ONE)
		}
		else if t0 == tmax.y {
			Vec2::new(T::ZERO, T::ONE)
		}
		else {
			return None;
		};

		Some(Hit2 {
			distance: t0,
			normal,
			index: 0,
		})
	}
}
