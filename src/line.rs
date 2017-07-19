/*!
Line segment.
*/

use ::std::ops::Range;

use ::point::{Point2, Point3};

use ::num::{Scalar, Float};

/// A line segment.
pub trait Line<P> {
	type T;
	fn swap(self) -> Self;
	/// Projects the point on the line.
	fn project(&self, pt: P) -> P where Self::T: Float;
	/// Point to line distance.
	fn dist(&self, pt: P) -> Self::T where Self::T: Float;
	/// Projects the point on the line segment, clamping at the end points.
	fn segment_project(&self, pt: P) -> P where Self::T: Float;
	/// Point to line segment distance.
	fn segment_dist(&self, pt: P) -> Self::T where Self::T: Float;
}

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
				$line { start: start, end: end }
			}
			pub fn point(pt: $pt<T>) -> $line<T> where $pt<T>: Copy {
				$line { start: pt, end: pt }
			}
			pub fn swap(self) -> $line<T> {
				$line { start: self.end, end: self.start }
			}
		}

		#[allow(non_snake_case)]
		pub fn $line<T>(start: $pt<T>, end: $pt<T>) -> $line<T> {
			$line { start, end }
		}

		impl<T: Scalar> $line<T> where T: Float {
			/// Projects the point on the line.
			pub fn project(&self, pt: $pt<T>) -> $pt<T> {
				self.start + (self.end - self.start).project(pt - self.start)
			}
			/// Point to line distance.
			pub fn dist(&self, pt: $pt<T>) -> T {
				self.project(pt).dist(pt)
			}
			/// Projects the point on the line segment, clamping at the end points.
			pub fn segment_project(&self, pt: $pt<T>) -> $pt<T> {
				self.start + (self.end - self.start).project_sat(pt - self.start)
			}
			/// Point to line segment distance.
			pub fn segment_dist(&self, pt: $pt<T>) -> T {
				self.segment_project(pt).dist(pt)
			}
		}

		impl<T> Line<$pt<T>> for $line<T> {
			type T = T;
			fn swap(self) -> $line<T> {
				$line { start: self.end, end: self.start }
			}
			fn project(&self, pt: $pt<T>) -> $pt<T> where T: Float {
				$line(self.start, self.end).project(pt)
			}
			fn dist(&self, pt: $pt<T>) -> T where T: Float {
				$line(self.start, self.end).dist(pt)
			}
			fn segment_project(&self, pt: $pt<T>) -> $pt<T> where T: Float {
				$line(self.start, self.end).segment_project(pt)
			}
			fn segment_dist(&self, pt: $pt<T>) -> T where T: Float {
				$line(self.start, self.end).segment_dist(pt)
			}
		}

		impl<T> Line<$pt<T>> for Range<$pt<T>> {
			type T = T;
			fn swap(self) -> Range<$pt<T>> {
				self.end..self.start
			}
			fn project(&self, pt: $pt<T>) -> $pt<T> where T: Float {
				$line(self.start, self.end).project(pt)
			}
			fn dist(&self, pt: $pt<T>) -> T where T: Float {
				$line(self.start, self.end).dist(pt)
			}
			fn segment_project(&self, pt: $pt<T>) -> $pt<T> where T: Float {
				$line(self.start, self.end).segment_project(pt)
			}
			fn segment_dist(&self, pt: $pt<T>) -> T where T: Float {
				$line(self.start, self.end).segment_dist(pt)
			}
		}

		impl<'a, T> Line<$pt<T>> for Range<&'a $pt<T>> {
			type T = T;
			fn swap(self) -> Range<&'a $pt<T>> {
				self.end..self.start
			}
			fn project(&self, pt: $pt<T>) -> $pt<T> where T: Float {
				$line(*self.start, *self.end).project(pt)
			}
			fn dist(&self, pt: $pt<T>) -> T where T: Float {
				self.project(pt).dist(pt)
			}
			fn segment_project(&self, pt: $pt<T>) -> $pt<T> where T: Float {
				$line(*self.start, *self.end).segment_project(pt)
			}
			fn segment_dist(&self, pt: $pt<T>) -> T where T: Float {
				self.segment_project(pt).dist(pt)
			}
		}
	};
}

line!(Line2 Point2);
line!(Line3 Point3);
