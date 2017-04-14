/*!
Line segment.
*/

use ::{Point2, Point3};

use ::num::{Scalar, Float};

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

macro_rules! line {
	($ty:ident $pt:ident) => {
		impl<T: Scalar> $ty<T> {
			/// Projects the point on the line.
			pub fn project(self, pt: $pt<T>) -> $pt<T> {
				self.start + (self.end - self.start) * $pt::dot(self.end - self.start, pt - self.start)
			}
			/// Squared point to line distance.
			pub fn dist_sqr(self, pt: $pt<T>) -> T {
				self.project(pt).dist_sqr(pt)
			}
			/// Point to line distance.
			pub fn dist(self, pt: $pt<T>) -> T where T: Float {
				self.project(pt).dist(pt)
			}
		}
	};
}

line!(Line2 Point2);
line!(Line3 Point3);
