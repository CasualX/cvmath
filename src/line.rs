/*!
Line segment.
*/

use ::{Point2, Point3};

use ::num::{Float};

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
	($line:ident $pt:ident) => {
		impl<T: Float> $line<T> {
			/// Projects the point on the line.
			pub fn project(self, pt: $pt<T>) -> $pt<T> {
				let p = (self.end - self.start).project(pt - self.start);
				self.start + p
			}
			/// Point to line distance.
			pub fn dist(self, pt: $pt<T>) -> T {
				self.project(pt).dist(pt)
			}
			/// Projects the point on the line segment, clamping at the end points.
			pub fn segment_project(self, pt: $pt<T>) -> $pt<T> {
				let base = self.end - self.start;
				let v = pt - self.start;
				let p = base.project(v);
				if p.len_sqr() < T::zero() {
					self.start
				}
				else if p.len_sqr() > base.len_sqr() {
					self.end
				}
				else {
					self.start + p
				}
			}
			/// Point to line segment distance.
			pub fn segment_dist(self, pt: $pt<T>) -> T {
				self.segment_project(pt).dist(pt)
			}
		}
	};
}

line!(Line2 Point2);
line!(Line3 Point3);
