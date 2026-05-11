use super::*;

/// Point2 shape.
pub type Point2<T> = Vec2<T>;

/// Point2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Point2<T>(x: T, y: T) -> Point2<T> {
	Point2 { x, y }
}

/// Point2 constructor.
///
/// ```
/// use cvmath::Point2;
///
/// let splat = cvmath::Point2!(3);
/// let zero: Point2<i32> = cvmath::Point2!();
///
/// assert_eq!(splat, Point2(3, 3));
/// assert_eq!(zero, Point2::ZERO);
/// ```
#[macro_export]
macro_rules! Point2 {
	($value:expr) => {
		$crate::Point2 { x: $value, y: $value }
	};
	() => {
		$crate::Point2::ZERO
	};
}

specialized_type!(Point2, Point2f, f32, x, y);
specialized_type!(Point2, Point2d, f64, x, y);
specialized_type!(Point2, Point2i, i32, x, y);

//----------------------------------------------------------------

// Due to accuracy concerns, rays never hit points
impl<T: Float> Trace2<T> for Point2<T> {
	#[inline]
	fn inside(&self, _pt: Point2<T>) -> bool {
		false
	}

	#[inline]
	fn trace(&self, _ray: &Ray2<T>) -> Option<Hit2<T>> {
		None
	}
}
