/*!
Rectangle and Box.
*/

use ::{Point2, Point3};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Rect<T> {
	mins: Point2<T>,
	maxs: Point2<T>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Cuboid<T> {
	mins: Point3<T>,
	maxs: Point3<T>,
}

macro_rules! rect_box {
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

rect_box!(Rect Point2 { x, y });
rect_box!(Cuboid Point3 { x, y, z });
