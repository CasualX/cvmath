/*!
Bounding boxes.
*/

use super::*;

//----------------------------------------------------------------

/// Bounds shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Bounds<T> {
	pub mins: T,
	pub maxs: T,
}

/// Bounds constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Bounds<T>(mins: T, maxs: T) -> Bounds<T> {
	Bounds { mins, maxs }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Bounds<T> {}

impl<T: Zero> Bounds<T> {
	/// Zero bounds.
	pub const ZERO: Bounds<T> = Bounds { mins: T::ZERO, maxs: T::ZERO };
}
impl<T: Zero + One> Bounds<T> {
	/// Unit bounds.
	pub const UNIT: Bounds<T> = Bounds { mins: T::ZERO, maxs: T::ONE };
}

impl<T> Bounds<T> {
	/// Constructs a new bounds.
	#[inline]
	pub const fn new(mins: T, maxs: T) -> Bounds<T> {
		Bounds { mins, maxs }
	}
	/// Bounds from the origin to the vector.
	#[inline]
	pub fn vec(vec: T) -> Bounds<T> where T: Default {
		Bounds {
			mins: T::default(),
			maxs: vec,
		}
	}
	/// Creates a bounds at the given point with size.
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let p = Point2(3, -4);
	/// let size = Point2(1, 1);
	/// let bounds = Bounds::point(p, size);
	/// assert_eq!(bounds.mins, p - size);
	/// assert_eq!(bounds.maxs, p + size);
	/// ```
	#[inline]
	pub fn point(point: T, size: T) -> Bounds<T> where T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> {
		Bounds { mins: point - size, maxs: point + size }
	}
	/// Normalizes the min and max values ensuring that `self.mins <= self.maxs`.
	///
	/// Because the constructors don't implicitly do this for you,
	/// it is typical to have this call follow the construction of the bounds.
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let bounds = Bounds(Point2(1, 0), Point2(-2, 3)).norm();
	/// assert_eq!(Bounds(Point2(-2, 0), Point2(1, 3)), bounds);
	/// ```
	#[inline]
	pub fn norm(self) -> Bounds<T> where T: Extrema {
		let (mins, maxs) = self.mins.min_max(self.maxs);
		Bounds { mins, maxs }
	}
	/// Returns the size of the bounds.
	///
	/// ```
	/// use cvmath::{Bounds, Point2, Vec2};
	///
	/// let bounds = Bounds(Point2(1, 2), Point2(3, 1));
	/// assert_eq!(Vec2(2, -1), bounds.size());
	/// ```
	#[inline]
	pub fn size(self) -> T::Output where T: ops::Sub {
		self.maxs - self.mins
	}
}

impl<T> Bounds<T> {
	/// Returns whether the point `rhs` is contained within `self`.
	///
	/// <!--CONTAINS-->
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let bounds = Bounds(Point2(1, 1), Point2(4, 3));
	/// assert!(bounds.contains(Point2(1, 1)));
	/// assert!(bounds.contains(Point2(3, 2)));
	///
	/// assert!(!bounds.contains(Point2(0, 0)));
	/// ```
	#[inline]
	pub fn contains(&self, rhs: T) -> bool where T: SpatialOrd {
		rhs.spatial_ge(&self.mins) && rhs.spatial_le(&self.maxs)
	}
	/// Returns whether the bounds `rhs` is fully contained within `self`.
	///
	/// <!--ENCLOSES-->
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let bounds = Bounds(Point2(1, 1), Point2(4, 3));
	/// let small = Bounds(Point2(2, 2), Point2(3, 3));
	/// assert!(bounds.encloses(small));
	/// assert!(!small.encloses(bounds));
	///
	/// let overlap = Bounds(Point2(2, 0), Point2(3, 2));
	/// assert!(!bounds.encloses(overlap));
	/// assert!(!overlap.encloses(bounds));
	/// ```
	#[inline]
	pub fn encloses(&self, rhs: Bounds<T>) -> bool where T: SpatialOrd {
		rhs.mins.spatial_ge(&self.mins) && rhs.maxs.spatial_le(&self.maxs)
	}
	/// Returns whether `rhs` is overlapped with `self`.
	///
	/// <!--OVERLAPS-->
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let bounds = Bounds(Point2(1, 1), Point2(4, 3));
	/// let overlap = Bounds(Point2(2, 0), Point2(3, 2));
	/// assert!(bounds.overlaps(overlap));
	/// assert!(overlap.overlaps(bounds));
	/// ```
	#[inline]
	pub fn overlaps(&self, rhs: Bounds<T>) -> bool where T: SpatialOrd {
		rhs.maxs.spatial_ge(&self.mins) && rhs.mins.spatial_le(&self.maxs)
	}
	/// Includes the point in the bounds.
	pub fn include(self, pt: T) -> Bounds<T> where T: Copy + Extrema {
		let mins = self.mins.min(pt);
		let maxs = self.maxs.max(pt);
		Bounds { mins, maxs }
	}
	/// Returns the new bounds containing both `rhs` and `self`.
	///
	/// <!--UNION-->
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let bounds = Bounds(Point2(1, 1), Point2(4, 3));
	/// let other = Bounds(Point2(2, 0), Point2(3, 2));
	/// let result = Bounds(Point2(1, 0), Point2(4, 3));
	///
	/// assert_eq!(result, bounds.union(other));
	/// assert_eq!(result, other.union(bounds));
	/// ```
	#[inline]
	pub fn union(self, rhs: Bounds<T>) -> Bounds<T> where T: Extrema {
		let mins = self.mins.min(rhs.mins);
		let maxs = self.maxs.max(rhs.maxs);
		Bounds { mins, maxs }
	}
	/// Returns the overlapping area (if any) between `rhs` and `self`.
	///
	/// <!--INTERSECT-->
	///
	/// ```
	/// use cvmath::{Bounds, Point2};
	///
	/// let bounds = Bounds(Point2(1, 1), Point2(4, 3));
	/// let other = Bounds(Point2(2, 0), Point2(3, 2));
	/// let result = Bounds(Point2(2, 1), Point2(3, 2));
	///
	/// assert_eq!(Some(result), bounds.intersect(other));
	/// assert_eq!(Some(result), other.intersect(bounds));
	///
	/// let touching = Bounds(Point2(0, 1), Point2(1, 3));
	/// let result = Bounds(Point2(1, 1), Point2(1, 3));
	///
	/// assert_eq!(Some(result), bounds.intersect(touching));
	/// assert_eq!(Some(result), touching.intersect(bounds));
	///
	/// let negative = Bounds(Point2(-1, -1), Point2(0, 0));
	/// assert_eq!(None, bounds.intersect(negative));
	/// assert_eq!(None, negative.intersect(bounds));
	/// ```
	#[inline]
	pub fn intersect(self, rhs: Bounds<T>) -> Option<Bounds<T>> where T: Extrema + SpatialOrd {
		let mins = self.mins.max(rhs.mins);
		let maxs = self.maxs.min(rhs.maxs);
		if mins.spatial_le(&maxs) {
			Some(Bounds { mins, maxs })
		}
		else {
			None
		}
	}
}
impl<T> Bounds<T> {
	/// Returns whether `rhs` is strictly contained within `self`.
	///
	/// <!--STRICTLY_CONTAINS-->
	#[inline]
	pub fn strictly_contains(&self, rhs: T) -> bool where T: SpatialOrd {
		rhs.spatial_ge(&self.mins) && rhs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly contained within `self`.
	///
	/// <!--STRICTLY_ENCLOSES-->
	#[inline]
	pub fn strictly_encloses(&self, rhs: Bounds<T>) -> bool where T: SpatialOrd {
		rhs.mins.spatial_gt(&self.mins) && rhs.maxs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly overlapped with `self`.
	///
	/// <!--STRICTLY_OVERLAPS-->
	#[inline]
	pub fn strictly_overlaps(&self, rhs: Bounds<T>) -> bool where T: SpatialOrd {
		rhs.maxs.spatial_gt(&self.mins) && rhs.mins.spatial_lt(&self.maxs)
	}
	/// Returns the overlapping area (not empty) between `rhs` and `self`.
	///
	/// <!--STRICTLY_INTERSECT-->
	#[inline]
	pub fn strictly_intersect(self, rhs: Bounds<T>) -> Option<Bounds<T>> where T: Extrema + SpatialOrd {
		let mins = self.mins.max(rhs.mins);
		let maxs = self.maxs.min(rhs.maxs);
		if mins.spatial_lt(&maxs) {
			Some(Bounds { mins, maxs })
		}
		else {
			None
		}
	}
}

impl<U: Copy, T: ops::Add<U>> ops::Add<U> for Bounds<T> {
	type Output = Bounds<T::Output>;
	#[inline]
	fn add(self, rhs: U) -> Bounds<T::Output> {
		Bounds {
			mins: self.mins + rhs,
			maxs: self.maxs + rhs,
		}
	}
}
impl<U: Copy, T: ops::Sub<U>> ops::Sub<U> for Bounds<T> {
	type Output = Bounds<T::Output>;
	#[inline]
	fn sub(self, rhs: U) -> Bounds<T::Output> {
		Bounds {
			mins: self.mins - rhs,
			maxs: self.maxs - rhs,
		}
	}
}
impl<U: Copy, T: ops::AddAssign<U>> ops::AddAssign<U> for Bounds<T> {
	#[inline]
	fn add_assign(&mut self, rhs: U) {
		self.mins += rhs;
		self.maxs += rhs;
	}
}
impl<U: Copy, T: ops::SubAssign<U>> ops::SubAssign<U> for Bounds<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: U) {
		self.mins -= rhs;
		self.maxs -= rhs;
	}
}

impl<T> AsRef<[T; 2]> for Bounds<T> {
	#[inline]
	fn as_ref(&self) -> &[T; 2] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> AsMut<[T; 2]> for Bounds<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T; 2] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> From<[T; 2]> for Bounds<T> {
	#[inline]
	fn from([mins, maxs]: [T; 2]) -> Bounds<T> {
		Bounds { mins, maxs }
	}
}
impl<T> From<Bounds<T>> for [T; 2] {
	#[inline]
	fn from(bounds: Bounds<T>) -> [T; 2] {
		[bounds.mins, bounds.maxs]
	}
}

//----------------------------------------------------------------

/// Bounds2 shape.
pub type Bounds2<T> = Bounds<Point2<T>>;

/// Bounds2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Bounds2<T>(mins: Point2<T>, maxs: Point2<T>) -> Bounds2<T> {
	Bounds2 { mins, maxs }
}

impl<T> Bounds2<T> {
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
}

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

specialized_type!(Bounds, Bounds2f, Vec2f, mins, maxs);
specialized_type!(Bounds, Bounds2d, Vec2d, mins, maxs);
specialized_type!(Bounds, Bounds2i, Vec2i, mins, maxs);

//----------------------------------------------------------------

/// Bounds3 shape.
pub type Bounds3<T> = Bounds<Point3<T>>;

/// Bounds3 constructor.
#[allow(non_snake_case)]
pub const fn Bounds3<T>(mins: Point3<T>, maxs: Point3<T>) -> Bounds3<T> {
	Bounds3 { mins, maxs }
}

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

specialized_type!(Bounds, Bounds3f, Vec3f, mins, maxs);
specialized_type!(Bounds, Bounds3d, Vec3d, mins, maxs);
specialized_type!(Bounds, Bounds3i, Vec3i, mins, maxs);

//----------------------------------------------------------------

impl<T: Float> TraceRay<T> for Bounds3<T> {
	fn inside(&self, ray: &Ray<T>) -> bool {
		self.contains(ray.origin)
	}

	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize {
		let inv_dir = Vec3::new(T::ONE / ray.direction.x, T::ONE / ray.direction.y, T::ONE / ray.direction.z);
		let mut tmin = (self.mins - ray.origin) * inv_dir;
		let mut tmax = (self.maxs - ray.origin) * inv_dir;
		if tmin.x > tmax.x {
			tmin.x = tmax.x;
			tmax.x = tmin.x;
		}
		if tmin.y > tmax.y {
			tmin.y = tmax.y;
			tmax.y = tmin.y;
		}
		if tmin.z > tmax.z {
			tmin.z = tmax.z;
			tmax.z = tmin.z;
		}
		let t0 = tmin.vmax();
		let t1 = tmax.vmin();
		if t0 <= t1 {
			hits[0] = TraceHit {
				distance: t0,
				normal: Vec3::new(
					if t0 == tmin.x { -T::ONE } else { if t0 == tmax.x { T::ONE } else { T::ZERO } },
					if t0 == tmin.y { -T::ONE } else { if t0 == tmax.y { T::ONE } else { T::ZERO } },
					if t0 == tmin.z { -T::ONE } else { if t0 == tmax.z { T::ONE } else { T::ZERO } },
				),
			};
			return 1;
		}
		else {
			return 0;
		}
	}
}
