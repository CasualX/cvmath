/*!
Rectangle and Box.
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

impl<T> Rect<T> {
	pub fn unit() -> Rect<T> where T: Zero + One {
		Rect {
			mins: Point2 { x: T::zero(), y: T::zero() },
			maxs: Point2 { x: T::one(), y: T::one() },
		}
	}
	pub fn new(mins: Point2<T>, maxs: Point2<T>) -> Rect<T> {
		Rect { mins, maxs }
	}
	pub fn point(pt: Point2<T>) -> Rect<T> where Point2<T>: Copy {
		Rect { mins: pt, maxs: pt }
	}
	pub fn swap(self) -> Rect<T> {
		Rect { mins: self.maxs, maxs: self.mins }
	}
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
pub struct Box<T> {
	pub mins: Point3<T>,
	pub maxs: Point3<T>,
}

macro_rules! rect {
	($ty:ident $pt:ident { $($field:ident),+ }) => {
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
	};
}

rect!(Rect Point2 { x, y });
rect!(Box Point3 { x, y, z });
