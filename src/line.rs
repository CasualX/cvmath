/*!
Line segment.
*/

use ::{Point2, Point3};

/// A 2D line segment.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Line2<T> {
	pub start: Point2<T>,
	pub end: Point2<T>,
}

/// A 3D line segment.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Line3<T> {
	pub start: Point3<T>,
	pub end: Point3<T>,
}
