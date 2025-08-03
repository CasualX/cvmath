use super::*;

/// Line3 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Line3<T> {
	pub start: Point3<T>,
	pub end: Point3<T>,
}

/// Line3 constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Line3<T>(start: Point3<T>, end: Point3<T>) -> Line3<T> {
	Line3 { start, end }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Line3<T> {}

impl<T> Line3<T> {
	/// Constructs a new line.
	#[inline]
	pub const fn new(start: Point3<T>, end: Point3<T>) -> Line3<T> {
		Line3 { start, end }
	}

	/// Pinches the line at the given point.
	#[inline]
	pub const fn pinch(self, pt: Point3<T>) -> (Line3<T>, Line3<T>) where T: Copy {
		let Line3 { start, end } = self;
		(Line3::new(start, pt), Line3::new(pt, end))
	}
}

impl<T: ops::Sub<Output = T>> Line3<T> {
	/// Line direction.
	#[inline]
	pub fn direction(self) -> Vec3<T> {
		self.end - self.start
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
	pub fn distance(self, pt: Point3<T>) -> T {
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
