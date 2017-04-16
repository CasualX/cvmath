/*!
Rectangle and Box.
*/

use ::point::{Point2, Point3};

use ::num::{Scalar};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Rect<T> {
	mins: Point2<T>,
	maxs: Point2<T>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Box<T> {
	mins: Point3<T>,
	maxs: Point3<T>,
}

macro_rules! rect {
	($ty:ident $pt:ident { $($field:ident),+ }) => {
		impl<T> $ty<T> {
			pub fn new(mins: $pt<T>, maxs: $pt<T>) -> $ty<T> {
				$ty {
					mins: mins,
					maxs: maxs,
				}
			}
			pub fn point(pt: $pt<T>) -> $ty<T> where T: Copy {
				$ty {
					mins: pt,
					maxs: pt,
				}
			}
		}

		//----------------------------------------------------------------
		// Operations

		impl<T> $ty<T> {
			pub fn intersect(self, rhs: $ty<T>) -> Option<$ty<T>> where T: Scalar {
				let mins = $pt::max(self.mins, rhs.mins);
				let maxs = $pt::min(self.maxs, rhs.maxs);
				if mins <= maxs {
					Some($ty {
						mins: mins,
						maxs: maxs,
					})
				}
				else {
					None
				}
			}
		}
	};
}

rect!(Rect Point2 { x, y });
rect!(Box Point3 { x, y, z });
