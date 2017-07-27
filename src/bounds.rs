/*!
Rectangle and Cuboid bounding boxes.
*/

use ::std::{ops};

use ::point::{Point2, Point3};
use ::line::{Line2};

use ::num::{Scalar, Zero, One, Extrema, SpatialOrd};

//----------------------------------------------------------------

/// General bounds.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Bounds<T> {
	pub mins: T,
	pub maxs: T,
}
/// Bounds constructor.
#[allow(non_snake_case)]
pub fn Bounds<T>(mins: T, maxs: T) -> Bounds<T> {
	Bounds { mins, maxs }
}

impl<T> Bounds<T> {
	/// Constructor.
	pub fn new(mins: T, maxs: T) -> Bounds<T> {
		Bounds { mins, maxs }
	}
	pub fn vec(vec: T) -> Bounds<T> where T: Default {
		Bounds {
			mins: T::default(),
			maxs: vec,
		}
	}
	/// Creates a bounds at the given point.
	///
	/// ```
	/// use cvmath::prelude::{Bounds, Point2};
	///
	/// let p = Point2(3, -4);
	/// let bounds = Bounds::point(p);
	/// assert_eq!(bounds.mins, p);
	/// assert_eq!(bounds.maxs, p);
	/// ```
	pub fn point(point: T) -> Bounds<T> where T: Copy {
		Bounds { mins: point, maxs: point }
	}
	/// Sorts the min and max values ensuring that `self.mins <= self.maxs`.
	///
	/// Because the constructors don't implicitly do this for you,
	/// it is typical to have this call follow the construction of the bounds.
	///
	/// ```
	/// use cvmath::prelude::{Bounds, Point2};
	///
	/// let bounds = Bounds::new(Point2(1, 0), Point2(-2, 3)).sort();
	/// assert_eq!(Bounds(Point2(-2, 0), Point2(1, 3)), bounds);
	/// ```
	pub fn sort(self) -> Bounds<T::Output> where T: Extrema {
		let (mins, maxs) = self.mins.min_max(self.maxs);
		Bounds { mins, maxs }
	}
	/// Returns the size of the bounds.
	///
	/// ```
	/// use cvmath::prelude::{Bounds, Point2, Vec2};
	///
	/// let bounds = Bounds::new(Point2(1, 2), Point2(3, 1));
	/// assert_eq!(Vec2(2, -1), bounds.size());
	/// ```
	pub fn size(self) -> T::Output where T: ops::Sub {
		self.maxs - self.mins
	}
}

impl<T> Bounds<T> {
	/// Returns whether the point `rhs` is contained within `self`.
	pub fn contains(&self, rhs: &T) -> bool where T: SpatialOrd {
		rhs.spatial_ge(&self.mins) && rhs.spatial_le(&self.maxs)
	}
	/// Returns whether the bounds `rhs` is fully contained within `self`.
	pub fn encloses(&self, rhs: &Bounds<T>) -> bool where T: SpatialOrd {
		rhs.mins.spatial_ge(&self.mins) && rhs.maxs.spatial_le(&self.maxs)
	}
	/// Returns whether `rhs` is overlapped with `self`.
	pub fn overlaps(&self, rhs: &Bounds<T>) -> bool where T: SpatialOrd {
		rhs.maxs.spatial_ge(&self.mins) && rhs.mins.spatial_le(&self.maxs)
	}
	/// Returns the new bounds containing both `rhs` and `self`.
	pub fn union(self, rhs: Bounds<T>) -> Bounds<T::Output> where T: Extrema {
		let mins = self.mins.min(rhs.mins);
		let maxs = self.maxs.max(rhs.maxs);
		Bounds { mins, maxs }
	}
	/// Returns the overlapping area (if any) between `rhs` and `self`.
	pub fn intersect(self, rhs: Bounds<T>) -> Option<Bounds<T::Output>> where T: Extrema, T::Output: SpatialOrd {
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
	pub fn strictly_contains(&self, rhs: &T) -> bool where T: SpatialOrd {
		rhs.spatial_ge(&self.mins) && rhs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly contained within `self`.
	pub fn strictly_encloses(&self, rhs: &Bounds<T>) -> bool where T: SpatialOrd {
		rhs.mins.spatial_gt(&self.mins) && rhs.maxs.spatial_lt(&self.maxs)
	}
	/// Returns whether `rhs` is strictly overlapped with `self`.
	pub fn strictly_overlaps(&self, rhs: &Bounds<T>) -> bool where T: SpatialOrd {
		rhs.maxs.spatial_gt(&self.mins) && rhs.mins.spatial_lt(&self.maxs)
	}
	/// Returns the overlapping area (not empty) between `rhs` and `self`.
	pub fn strictly_intersect(self, rhs: Bounds<T>) -> Option<Bounds<T::Output>> where T: Extrema, T::Output: SpatialOrd {
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
	fn add(self, rhs: U) -> Bounds<T::Output> {
		Bounds {
			mins: self.mins + rhs,
			maxs: self.maxs + rhs,
		}
	}
}
impl<U: Copy, T: ops::Sub<U>> ops::Sub<U> for Bounds<T> {
	type Output = Bounds<T::Output>;
	fn sub(self, rhs: U) -> Bounds<T::Output> {
		Bounds {
			mins: self.mins - rhs,
			maxs: self.maxs - rhs,
		}
	}
}
impl<U: Copy, T: ops::AddAssign<U>> ops::AddAssign<U> for Bounds<T> {
	fn add_assign(&mut self, rhs: U) {
		self.mins += rhs;
		self.maxs += rhs;
	}
}
impl<U: Copy, T: ops::SubAssign<U>> ops::SubAssign<U> for Bounds<T> {
	fn sub_assign(&mut self, rhs: U) {
		self.mins -= rhs;
		self.maxs -= rhs;
	}
}

//----------------------------------------------------------------

pub type Rect<T> = Bounds<Point2<T>>;
#[allow(non_snake_case)]
pub fn Rect<T>(mins: Point2<T>, maxs: Point2<T>) -> Rect<T> {
	Rect { mins, maxs }
}

impl<T: Scalar> Rect<T> {
	pub fn unit() -> Rect<T> where T: Zero + One + Copy {
		Rect {
			mins: Point2::dup(T::zero()),
			maxs: Point2::dup(T::one()),
		}
	}
	/// X coordinate of the left side.
	pub fn left(&self) -> T { self.mins.x }
	/// X coordinate of the right side.
	pub fn right(&self) -> T { self.maxs.x }
	/// Y coordinate of the top side.
	pub fn top(&self) -> T { self.mins.y }
	/// Y coordinate of the bottom side.
	pub fn bottom(&self) -> T { self.maxs.y }
	/// Width of the rectangle.
	pub fn width(&self) -> T { self.maxs.x - self.mins.x }
	/// Height of the rectangle.
	pub fn height(&self) -> T { self.maxs.y - self.mins.y }
	/// Area of the rectangle.
	pub fn area(&self) -> T { (self.maxs.x - self.mins.x) * (self.maxs.y - self.mins.y) }
	/// Center of the rectangle.
	pub fn center(&self) -> Point2<T> {
		(self.mins + self.maxs) / (T::one() + T::one())
	}
	/// Top left corner of the rectangle.
	pub fn top_left(&self) -> Point2<T> { self.mins }
	/// Top right corner of the rectangle.
	pub fn top_right(&self) -> Point2<T> { Point2 { x: self.maxs.x, y: self.mins.y } }
	/// Bottom left corner of the rectangle.
	pub fn bottom_left(&self) -> Point2<T> { Point2 { x: self.mins.x, y: self.maxs.y } }
	/// Bottom right corner of the rectangle.
	pub fn bottom_right(&self) -> Point2<T> { self.maxs }
	pub fn top_side(&self) -> Line2<T> {
		Line2 {
			start: self.top_left(),
			end: self.top_right(),
		}
	}
	pub fn right_side(&self) -> Line2<T> {
		Line2 {
			start: self.top_right(),
			end: self.bottom_right(),
		}
	}
	pub fn bottom_side(&self) -> Line2<T> {
		Line2 {
			start: self.bottom_right(),
			end: self.bottom_left(),
		}
	}
	pub fn left_side(&self) -> Line2<T> {
		Line2 {
			start: self.bottom_left(),
			end: self.top_left(),
		}
	}
}

//----------------------------------------------------------------

pub type Cuboid<T> = Bounds<Point3<T>>;
#[allow(non_snake_case)]
pub fn Cuboid<T>(mins: Point3<T>, maxs: Point3<T>) -> Cuboid<T> {
	Cuboid { mins, maxs }
}

impl<T> Cuboid<T> {
	pub fn unit() -> Cuboid<T> where T: Zero + One + Copy {
		Cuboid {
			mins: Point3::dup(T::zero()),
			maxs: Point3::dup(T::one()),
		}
	}
	pub fn xy(self) -> Rect<T> {
		Rect {
			mins: self.mins.xy(),
			maxs: self.maxs.xy(),
		}
	}
	pub fn xz(self) -> Rect<T> {
		Rect {
			mins: Point2(self.mins.x, self.mins.z),
			maxs: Point2(self.maxs.x, self.maxs.z),
		}
	}
	pub fn yz(self) -> Rect<T> {
		Rect {
			mins: Point2(self.mins.y, self.mins.z),
			maxs: Point2(self.maxs.y, self.maxs.z),
		}
	}
}
