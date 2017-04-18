/*!
Line segment.
*/

use ::point::{Point2, Point3};

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
	($line:ident $pt:ident) => {
		impl<T> $line<T> {
			pub fn new(start: $pt<T>, end: $pt<T>) -> $line<T> {
				$line {
					start: start,
					end: end,
				}
			}
			pub fn point(pt: $pt<T>) -> $line<T> where T: Copy {
				$line {
					start: pt,
					end: pt,
				}
			}
			pub fn swap(self) -> $line<T> {
				$line {
					start: self.end,
					end: self.start,
				}
			}
		}
		impl<T: Scalar> $line<T> {
			/// Projects the point on the line.
			pub fn project(self, pt: $pt<T>) -> $pt<T> where T: Float {
				let p = (self.end - self.start).project(pt - self.start);
				self.start + p
			}
			/// Point to line distance.
			pub fn dist(self, pt: $pt<T>) -> T where T: Float {
				self.project(pt).dist(pt)
			}
			/// Projects the point on the line segment, clamping at the end points.
			pub fn segment_project(self, pt: $pt<T>) -> $pt<T> where T: Float {
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
			pub fn segment_dist(self, pt: $pt<T>) -> T where T: Float {
				self.segment_project(pt).dist(pt)
			}
		}
	};
}

line!(Line2 Point2);
line!(Line3 Point3);
