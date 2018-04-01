/*!
Line 2D segment.
*/

use std::ops::Range;
use point::Point2;
use num::Float;

pub type Line2<T> = Range<Point2<T>>;

/// Projects the point onto the line.
pub fn line_project<T>(line: Line2<T>, pt: Point2<T>) -> Point2<T> where T: Float {
	line.start + (pt - line.start).project(line.end - line.start)
}
/// Point to line distance.
pub fn line_dist<T>(line: Line2<T>, pt: Point2<T>) -> T where T: Float {
	line_project(line, pt).dist(pt)
}
/// Projects the point onto the line segment, clamping at the end points.
pub fn segment_project<T>(segment: Line2<T>, pt: Point2<T>) -> Point2<T> where T: Float {
	segment.start + (pt - segment.start).project_sat(segment.end - segment.start)
}
/// Point to line segment distance.
pub fn segment_dist<T>(segment: Line2<T>, pt: Point2<T>) -> T where T: Float {
	segment_project(segment, pt).dist(pt)
}

/// Intersect two lines.
///
/// The result is some point if the two lines intersect, none if they are parallel.
///
/// ```
/// # use cvmath::line2::line_x;
/// # use cvmath::point::Point2;
///
/// let line1 = Point2(1.0, 1.0)..Point2(2.0, 2.0);
/// let line2 = Point2(-1.0, 1.0)..Point2(1.0, -1.0);
///
/// let result = line_x(line1, line2);
///
/// assert_eq!(result, Some(Point2(0.0, 0.0)));
/// ```
#[inline]
pub fn line_x<T: Float>(line1: Line2<T>, line2: Line2<T>) -> Option<Point2<T>> {
	let dline1 = line1.start - line1.end;
	let dline2 = line2.start - line2.end;

	let denom = dline1.cross(dline2);
	if denom == T::zero() {
		return None;
	}

	let p = dline2 * line1.start.cross(line1.end) - dline1 * line2.start.cross(line2.end);
	Some(p / denom)
}

/// Intersect a line and line segment.
///
/// The result is scalar with which to scale the segment to find the intersection point, none if the line and line segment are parallel.
///
/// To test if the line segment actually intersects the line, check if this result lies inside the [0; 1] range.
/// To calculate the intersection point scale the segment by this function's result.
///
/// ```
/// # use cvmath::line2::segment_x;
/// # use cvmath::point::Point2;
///
/// let line = Point2(1.0, 1.0)..Point2(2.0, 2.0);
/// let segment = Point2(-1.0, 1.0)..Point2(1.0, -1.0);
///
/// let result = segment_x(line, segment.clone());
/// assert_eq!(result, Some(0.5));
///
/// let x = segment.start + (segment.end - segment.start) * result.unwrap();
/// assert_eq!(x, Point2(0.0, 0.0));
/// ```
#[inline]
pub fn segment_x<T: Float>(line: Line2<T>, segment: Line2<T>) -> Option<T> {
	let p = line.start;
	let r = line.end - line.start;
	let q = segment.start;
	let s = segment.end - segment.start;

	let denom = r.cross(s);
	if denom == T::zero() {
		return None;
	}

	let u = (q - p).cross(r) / denom;
	Some(u)
}
