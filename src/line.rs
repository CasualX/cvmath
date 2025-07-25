/*!
Line 2D segment.
*/

use super::*;

/// Line shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Line<T> {
	pub start: T,
	pub end: T,
}

/// Line constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Line<T>(start: T, end: T) -> Line<T> {
	Line { start, end }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Line<T> {}

impl<T> Line<T> {
	/// Constructs a new line.
	#[inline]
	pub const fn new(start: T, end: T) -> Line<T> {
		Line { start, end }
	}

	/// Pinches the line at the given point.
	#[inline]
	pub const fn pinch(self, pt: T) -> (Line<T>, Line<T>) where T: Copy {
		let Line { start, end } = self;
		(Line::new(start, pt), Line::new(pt, end))
	}
}

impl<T: ops::Sub<Output = T>> Line<T> {
	/// Line direction.
	#[inline]
	pub fn direction(self) -> T {
		self.end - self.start
	}
}

impl<T: Float> Line2<T> {
	/// Projects the point onto the line.
	#[inline]
	pub fn project(self, pt: Point2<T>) -> Point2<T> {
		self.start + (pt - self.start).project(self.direction())
	}

	/// Point to line distance.
	#[inline]
	pub fn dist_pt(self, pt: Point2<T>) -> T {
		self.project(pt).distance(pt)
	}

	/// Intersect a line and line segment.
	///
	/// The result is scalar with which to scale the segment to find the intersection point, none if the line and line segment are parallel.
	///
	/// To test if the line segment actually intersects the line, check if this result lies inside the [0; 1] range.
	/// To calculate the intersection point scale the segment by this function's result.
	///
	/// ```
	/// use cvmath::{Line2, Point2};
	///
	/// let line = Line2(Point2(1.0, 1.0), Point2(2.0, 2.0));
	/// let segment = Line2(Point2(-1.0, 1.0), Point2(1.0, -1.0));
	///
	/// let result = Line2::segment_x(line, segment);
	/// assert_eq!(result, Some(0.5));
	///
	/// let x = segment.start + (segment.end - segment.start) * result.unwrap();
	/// assert_eq!(x, Point2(0.0, 0.0));
	/// ```
	#[inline]
	pub fn segment_x(self, rhs: Line2<T>) -> Option<T> {
		let p = self.start;
		let r = self.end - self.start;
		let q = rhs.start;
		let s = rhs.end - rhs.start;

		let denom = r.cross(s);
		if denom == T::ZERO {
			return None;
		}

		let u = (q - p).cross(r) / denom;
		Some(u)
	}

	/// Intersect two lines.
	///
	/// The result is some point if the two lines intersect, none if they are parallel.
	///
	/// ```
	/// use cvmath::{Line2, Point2};
	///
	/// let line1 = Line2(Point2(1.0, 1.0), Point2(2.0, 2.0));
	/// let line2 = Line2(Point2(-1.0, 1.0), Point2(1.0, -1.0));
	///
	/// let result = Line2::intersect_pt(line1, line2);
	///
	/// assert_eq!(result, Some(Point2(0.0, 0.0)));
	/// ```
	#[inline]
	pub fn intersect_pt(self, rhs: Line2<T>) -> Option<Point2<T>> {
		let denom = self.direction().cross(rhs.direction());
		if denom == T::ZERO {
			return None;
		}

		let p = rhs.direction() * self.start.cross(self.start + self.direction()) - self.direction() * rhs.start.cross(rhs.start + rhs.direction());
		Some(p / denom)
	}

	/// Calculates the y coordinate where the line intercepts the Y axis.
	///
	/// Returns none if the line is parallel with the Y axis.
	#[inline]
	pub fn y_intercept(self) -> Option<T> {
		if self.direction().x == T::ZERO {
			return None;
		}
		let f = self.start.x / self.direction().x;
		let y = self.start.y + self.direction().y * f;
		Some(y)
	}
	/// Calculates the x coordinate where the line intercepts the X axis.
	///
	/// Returns none if the line is parallel with the X axis.
	#[inline]
	pub fn x_intercept(self) -> Option<T> {
		if self.direction().y == T::ZERO {
			return None;
		}
		let f = self.start.y / self.direction().y;
		let x = self.start.x + self.direction().x * f;
		Some(x)
	}
	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Line2<T>, t: T) -> Line2<T> {
		Line2 {
			start: self.start.lerp(target.start, t),
			end: self.end.lerp(target.end, t),
		}
	}
}

impl<T: Float> Line3<T> {
	/// Projects the point onto the line.
	#[inline]
	pub fn project(self, pt: Point3<T>) -> Point3<T> {
		self.start + (pt - self.start).project(self.direction())
	}
	/// Point to line distance.
	#[inline]
	pub fn dist_pt(self, pt: Point3<T>) -> T {
		self.project(pt).distance(pt)
	}
	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Line3<T>, t: T) -> Line3<T> {
		Line3 {
			start: self.start.lerp(target.start, t),
			end: self.end.lerp(target.end, t),
		}
	}
}

/// Line2 shape.
pub type Line2<T> = Line<Point2<T>>;

/// Line3 shape.
pub type Line3<T> = Line<Point3<T>>;

/// Line2 constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Line2<T>(start: Point2<T>, end: Point2<T>) -> Line2<T> {
	Line2 { start, end }
}

/// Line3 constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Line3<T>(start: Point3<T>, end: Point3<T>) -> Line3<T> {
	Line3 { start, end }
}
