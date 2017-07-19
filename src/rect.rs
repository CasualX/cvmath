/*!
Rectangle and Cuboid.
*/

use ::std::ops;

use ::point::{Point2, Point3};
use ::line::{Line2};

use ::num::{Scalar, Zero, One};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Rect<T> {
	pub mins: Point2<T>,
	pub maxs: Point2<T>,
}

impl<T: Scalar> Rect<T> {
	pub fn left(self) -> T { self.mins.x }
	pub fn right(self) -> T { self.maxs.x }
	pub fn top(self) -> T { self.mins.y }
	pub fn bottom(self) -> T { self.maxs.y }
	pub fn width(self) -> T { self.maxs.x - self.mins.x }
	pub fn height(self) -> T { self.maxs.y - self.mins.y }
	pub fn area(self) -> T { (self.maxs.x - self.mins.x) * (self.maxs.y - self.mins.y) }
	pub fn center(self) -> Point2<T> {
		let half = T::one() / (T::one() + T::one());
		(self.mins + self.maxs) * half
	}
	pub fn top_left(self) -> Point2<T> { self.mins }
	pub fn top_right(self) -> Point2<T> { Point2 { x: self.maxs.x, y: self.mins.y } }
	pub fn bottom_left(self) -> Point2<T> { Point2 { x: self.mins.x, y: self.maxs.y } }
	pub fn bottom_right(self) -> Point2<T> { self.maxs }
	pub fn top_side(self) -> Line2<T> {
		Line2 { 
			start: self.top_left(),
			end: self.top_right(),
		}
	}
	pub fn right_side(self) -> Line2<T> {
		Line2 {
			start: self.top_right(),
			end: self.bottom_right(),
		}
	}
	pub fn bottom_side(self) -> Line2<T> {
		Line2 {
			start: self.bottom_right(),
			end: self.bottom_left(),
		}
	}
	pub fn left_side(self) -> Line2<T> {
		Line2 {
			start: self.bottom_left(),
			end: self.top_left(),
		}
	}
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Cuboid<T> {
	pub mins: Point3<T>,
	pub maxs: Point3<T>,
}

macro_rules! rect {
	($ty:ident $pt:ident { $($field:ident),+ }) => {
		impl<T> $ty<T> {
			pub fn unit() -> $ty<T> where T: Zero + One + Copy {
				$ty {
					mins: $pt::dup(T::zero()),
					maxs: $pt::dup(T::one()),
				}
			}
			pub fn new(mins: $pt<T>, maxs: $pt<T>) -> $ty<T> {
				$ty { mins, maxs }
			}
			pub fn point(pt: $pt<T>) -> $ty<T> where $pt<T>: Copy {
				$ty {
					mins: pt,
					maxs: pt,
				}
			}
			pub fn swap(self) -> $ty<T> {
				$ty {
					mins: self.maxs,
					maxs: self.mins,
				}
			}
		}

		#[allow(non_snake_case)]
		pub fn $ty<T>(mins: $pt<T>, maxs: $pt<T>) -> $ty<T> {
			$ty { mins, maxs }
		}

		impl<T: Scalar> $ty<T> {
			pub fn contains(self, pt: $pt<T>) -> bool {
				pt >= self.mins && pt <= self.maxs
			}
			pub fn is_superset(self, rhs: $ty<T>) -> bool {
				rhs.mins >= self.mins && rhs.maxs <= self.maxs
			}
			pub fn is_disjoint(self, rhs: $ty<T>) -> bool {
				$(self.maxs.$field <= rhs.mins.$field ||)* $(self.mins.$field >= rhs.maxs.$field ||)* true
			}
			pub fn union(self, rhs: $ty<T>) -> $ty<T> {
				let mins = $pt::min(self.mins, rhs.mins);
				let maxs = $pt::max(self.maxs, rhs.maxs);
				$ty { mins, maxs }
			}
			pub fn intersect(self, rhs: $ty<T>) -> Option<$ty<T>> {
				let mins = $pt::max(self.mins, rhs.mins);
				let maxs = $pt::min(self.maxs, rhs.maxs);
				if mins <= maxs {
					Some($ty { mins, maxs })
				}
				else {
					None
				}
			}
		}

		impl<U, T: ops::Add<U>> ops::Add<$pt<U>> for $ty<T> where $pt<U>: Copy {
			type Output = $ty<T::Output>;
			fn add(self, rhs: $pt<U>) -> $ty<T::Output> {
				$ty { mins: self.mins + rhs, maxs: self.maxs + rhs }
			}
		}
		impl<U, T: ops::Sub<U>> ops::Sub<$pt<U>> for $ty<T> where $pt<U>: Copy {
			type Output = $ty<T::Output>;
			fn sub(self, rhs: $pt<U>) -> $ty<T::Output> {
				$ty { mins: self.mins - rhs, maxs: self.maxs - rhs }
			}
		}
		impl<U, T: ops::AddAssign<U>> ops::AddAssign<$pt<U>> for $ty<T> where $pt<U>: Copy {
			fn add_assign(&mut self, rhs: $pt<U>) {
				self.mins += rhs;
				self.maxs += rhs;
			}
		}
		impl<U, T: ops::SubAssign<U>> ops::SubAssign<$pt<U>> for $ty<T> where $pt<U>: Copy {
			fn sub_assign(&mut self, rhs: $pt<U>) {
				self.mins -= rhs;
				self.maxs -= rhs;
			}
		}
	};
}

rect!(Rect Point2 { x, y });
rect!(Cuboid Point3 { x, y, z });
