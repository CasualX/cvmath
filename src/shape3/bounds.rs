use super::*;

//----------------------------------------------------------------

/// Bounds3 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Bounds3<T> {
	pub mins: Vec3<T>,
	pub maxs: Vec3<T>,
}

/// Bounds3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Bounds3<T>(mins: Vec3<T>, maxs: Vec3<T>) -> Bounds3<T> {
	Bounds3 { mins, maxs }
}

specialized_type!(Bounds3, Bounds3f, f32, mins: Point3f, maxs: Point3f);
specialized_type!(Bounds3, Bounds3d, f64, mins: Point3d, maxs: Point3d);
specialized_type!(Bounds3, Bounds3i, i32, mins: Point3i, maxs: Point3i);

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Bounds3<T> {}

impl<T: Zero> Bounds3<T> {
	/// Zero bounds.
	pub const ZERO: Bounds3<T> = Bounds3 { mins: Point3::ZERO, maxs: Point3::ZERO };
}
impl<T: Zero + One> Bounds3<T> {
	/// Unit bounds.
	pub const UNIT: Bounds3<T> = Bounds3 { mins: Point3::ZERO, maxs: Point3::ONE };
}

impl<T> Bounds3<T> {
	/// Constructs a new bounds.
	#[inline]
	pub const fn new(mins: Point3<T>, maxs: Point3<T>) -> Bounds3<T> {
		Bounds3 { mins, maxs }
	}
	/// Bounds from the origin to the vector.
	#[inline]
	pub fn vec(vec: Vec3<T>) -> Bounds3<T> where T: Default {
		Bounds3 {
			mins: Point3::default(),
			maxs: vec,
		}
	}
	/// Creates a bounds at the given point with size.
	#[inline]
	pub fn point(point: Point3<T>, size: Vec3<T>) -> Bounds3<T> where T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> {
		Bounds3 { mins: point - size, maxs: point + size }
	}
	/// Normalizes the min and max values ensuring that `self.mins <= self.maxs`.
	///
	/// Because the constructors don't implicitly do this for you,
	/// it is typical to have this call follow the construction of the bounds.
	#[inline]
	pub fn norm(self) -> Bounds3<T> where T: Extrema {
		let (mins, maxs) = self.mins.min_max(self.maxs);
		Bounds3 { mins, maxs }
	}
	/// Returns the size of the bounds.
	///
	/// ```
	/// use cvmath::{Bounds3, Point3, Vec3};
	///
	/// let bounds = Bounds3(Point3(1, 1, 3), Point3(3, 2, 3));
	/// assert_eq!(Vec3(2, 1, 0), bounds.size());
	/// ```
	#[inline]
	pub fn size(self) -> Vec3<T> where T: ops::Sub<Output = T> {
		self.maxs - self.mins
	}
}

impl<T> Bounds3<T> {
	/// Returns whether the point `rhs` is contained within `self`.
	#[inline]
	pub fn contains(&self, rhs: Point3<T>) -> bool where T: PartialOrd {
		rhs.spatial_ge(&self.mins) && rhs.spatial_le(&self.maxs)
	}
	/// Returns whether the bounds `rhs` is fully contained within `self`.
	#[inline]
	pub fn encloses(&self, rhs: Bounds3<T>) -> bool where T: PartialOrd {
		rhs.mins.spatial_ge(&self.mins) && rhs.maxs.spatial_le(&self.maxs)
	}
	/// Returns whether `rhs` is overlapped with `self`.
	#[inline]
	pub fn overlaps(&self, rhs: Bounds3<T>) -> bool where T: PartialOrd {
		rhs.maxs.spatial_ge(&self.mins) && rhs.mins.spatial_le(&self.maxs)
	}
	/// Includes the point in the bounds.
	pub fn include(self, pt: Point3<T>) -> Bounds3<T> where T: Copy + Extrema {
		let mins = self.mins.min(pt);
		let maxs = self.maxs.max(pt);
		Bounds3 { mins, maxs }
	}
	/// Returns the new bounds containing both `rhs` and `self`.
	#[inline]
	pub fn union(self, rhs: Bounds3<T>) -> Bounds3<T> where T: Extrema {
		let mins = self.mins.min(rhs.mins);
		let maxs = self.maxs.max(rhs.maxs);
		Bounds3 { mins, maxs }
	}
	/// Returns the overlapping area (if any) between `rhs` and `self`.
	#[inline]
	pub fn intersect(self, rhs: Bounds3<T>) -> Option<Bounds3<T>> where T: PartialOrd + Extrema {
		let mins = self.mins.max(rhs.mins);
		let maxs = self.maxs.min(rhs.maxs);
		if mins.spatial_le(&maxs) {
			Some(Bounds3 { mins, maxs })
		}
		else {
			None
		}
	}
}
impl<T> Bounds3<T> {
	/// Returns whether `rhs` is strictly contained within `self`.
	#[inline]
	pub fn strictly_contains(&self, rhs: Point3<T>) -> bool where T: PartialOrd {
		rhs.spatial_gt(&self.mins) && rhs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly contained within `self`.
	#[inline]
	pub fn strictly_encloses(&self, rhs: Bounds3<T>) -> bool where T: PartialOrd {
		rhs.mins.spatial_gt(&self.mins) && rhs.maxs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly overlapped with `self`.
	#[inline]
	pub fn strictly_overlaps(&self, rhs: Bounds3<T>) -> bool where T: PartialOrd {
		rhs.maxs.spatial_gt(&self.mins) && rhs.mins.spatial_lt(&self.maxs)
	}
	/// Returns the overlapping area (not empty) between `rhs` and `self`.
	#[inline]
	pub fn strictly_intersect(self, rhs: Bounds3<T>) -> Option<Bounds3<T>> where T: PartialOrd + Extrema {
		let mins = self.mins.max(rhs.mins);
		let maxs = self.maxs.min(rhs.maxs);
		if mins.spatial_lt(&maxs) {
			Some(Bounds3 { mins, maxs })
		}
		else {
			None
		}
	}
}

impl<T: Copy + ops::Add<T, Output = T>> ops::Add<Vec3<T>> for Bounds3<T> {
	type Output = Bounds3<T>;
	#[inline]
	fn add(self, rhs: Vec3<T>) -> Bounds3<T> {
		Bounds3 {
			mins: self.mins + rhs,
			maxs: self.maxs + rhs,
		}
	}
}
impl<T: Copy + ops::Sub<T, Output = T>> ops::Sub<Vec3<T>> for Bounds3<T> {
	type Output = Bounds3<T>;
	#[inline]
	fn sub(self, rhs: Vec3<T>) -> Bounds3<T> {
		Bounds3 {
			mins: self.mins - rhs,
			maxs: self.maxs - rhs,
		}
	}
}
impl<T: Copy + ops::AddAssign<T>> ops::AddAssign<Vec3<T>> for Bounds3<T> {
	#[inline]
	fn add_assign(&mut self, rhs: Vec3<T>) {
		self.mins += rhs;
		self.maxs += rhs;
	}
}
impl<T: Copy + ops::SubAssign<T>> ops::SubAssign<Vec3<T>> for Bounds3<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: Vec3<T>) {
		self.mins -= rhs;
		self.maxs -= rhs;
	}
}

impl<T> AsRef<[Point3<T>; 2]> for Bounds3<T> {
	#[inline]
	fn as_ref(&self) -> &[Point3<T>; 2] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> AsMut<[Point3<T>; 2]> for Bounds3<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [Point3<T>; 2] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> From<[Point3<T>; 2]> for Bounds3<T> {
	#[inline]
	fn from([mins, maxs]: [Point3<T>; 2]) -> Bounds3<T> {
		Bounds3 { mins, maxs }
	}
}
impl<T> From<Bounds3<T>> for [Point3<T>; 2] {
	#[inline]
	fn from(bounds: Bounds3<T>) -> [Point3<T>; 2] {
		[bounds.mins, bounds.maxs]
	}
}

//----------------------------------------------------------------

impl<T> Bounds3<T> {
	/// Bounds3 constructor from components.
	#[inline]
	pub const fn c(mins_x: T, mins_y: T, mins_z: T, maxs_x: T, maxs_y: T, maxs_z: T) -> Bounds3<T> {
		let mins = Point3 { x: mins_x, y: mins_y, z: mins_z };
		let maxs = Point3 { x: maxs_x, y: maxs_y, z: maxs_z };
		Bounds3 { mins, maxs }
	}
	/// Casts the bounds to a different unit type.
	#[inline]
	pub fn cast<U>(self) -> Bounds3<U> where T: CastTo<U> {
		Bounds3 {
			mins: self.mins.cast(),
			maxs: self.maxs.cast(),
		}
	}
}

impl<T: Scalar> Bounds3<T> {
	/// Width of the Bounds3.
	#[inline]
	pub fn width(&self) -> T {
		self.maxs.x - self.mins.x
	}
	/// Height of the Bounds3.
	#[inline]
	pub fn height(&self) -> T {
		self.maxs.y - self.mins.y
	}
	/// Depth of the Bounds3.
	#[inline]
	pub fn depth(&self) -> T {
		self.maxs.z - self.mins.z
	}
	/// Volume of the Bounds3.
	#[inline]
	pub fn volume(&self) -> T {
		(self.maxs.x - self.mins.x) * (self.maxs.y - self.mins.y) * (self.maxs.z - self.mins.z)
	}
	/// Center of the Bounds3.
	#[inline]
	pub fn center(&self) -> Point3<T> {
		(self.mins + self.maxs) / (T::ONE + T::ONE)
	}
	/// Linear interpolation between the bounds.
	#[inline]
	pub fn lerp(self, rhs: Bounds3<T>, t: T) -> Bounds3<T> where T: Scalar {
		Bounds3 {
			mins: self.mins.lerp(rhs.mins, t),
			maxs: self.maxs.lerp(rhs.maxs, t),
		}
	}
	/// Transform of the unit cube.
	#[inline]
	pub fn transform(self) -> Transform3<T> {
		Transform3::compose(
			Vec3(self.width(), T::ZERO, T::ZERO),
			Vec3(T::ZERO, self.height(), T::ZERO),
			Vec3(T::ZERO, T::ZERO, self.depth()),
			self.mins,
		)
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T: Scalar> urandom::Distribution<Bounds3<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<Point3<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Bounds3<T> {
		let distr = urandom::distr::StandardUniform;
		let mins = distr.sample(rand);
		let maxs = distr.sample(rand);
		Bounds3 { mins, maxs }.norm()
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Bounds3<T> {
	type Sampler = Bounds3<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Bounds3<T>> for Bounds3<urandom::distr::Uniform<T>> where Point3<T>: urandom::distr::SampleUniform {
	#[inline]
	fn try_new(low: Bounds3<T>, high: Bounds3<T>) -> Result<Self, urandom::distr::UniformError> {
		let mins = Vec3::try_new(low.mins, high.mins)?;
		let maxs = Vec3::try_new(low.maxs, high.maxs)?;
		Ok(Bounds3 { mins, maxs })
	}
	#[inline]
	fn try_new_inclusive(low: Bounds3<T>, high: Bounds3<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let mins = Vec3::try_new_inclusive(low.mins, high.mins)?;
		let maxs = Vec3::try_new_inclusive(low.maxs, high.maxs)?;
		Ok(Bounds3 { mins, maxs })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Bounds3<T>> for Bounds3<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Bounds3<T> {
		let mins = self.mins.sample(rand);
		let maxs = self.maxs.sample(rand);
		Bounds3 { mins, maxs }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace3<T> for Bounds3<T> {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		self.contains(pt)
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let inv_dir = ray.direction.map(|d| T::ONE / d);

		let tmin = (self.mins - ray.origin) * inv_dir;
		let tmax = (self.maxs - ray.origin) * inv_dir;
		let (tmin, tmax) = tmin.min_max(tmax);

		let t0 = tmin.vmax();
		let t1 = tmax.vmin();

		let t = if !(t0 <= t1) { return None }
		else if t0 > T::EPSILON && t0 <= ray.distance { t0 }
		else if t1 > T::EPSILON && t1 <= ray.distance { t1 }
		else { return None };

		// Outward shape normal: use direction sign per axis
		let sign = ray.direction.map(T::signum);

		// Calculate the normal based on which axis was hit
		let normal = (
			Vec3::dup(t).eq(tmin).select(-sign, Vec3::ZERO) +
			Vec3::dup(t).eq(tmax).select( sign, Vec3::ZERO)
		).norm();

		Some(Hit3 {
			distance: t0,
			normal,
			index: 0,
		})
	}
}
