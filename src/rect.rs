/*!
Rectangle and Box.
*/

use ::point::{Point2, Point3};

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
		}
	};
}

rect!(Rect Point2 { x, y });
rect!(Box Point3 { x, y, z });
