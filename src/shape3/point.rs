use super::*;

/// Point3 shape.
pub type Point3<T> = Vec3<T>;

/// Point3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Point3<T>(x: T, y: T, z: T) -> Point3<T> {
	Point3 { x, y, z }
}

/// Point3 constructor.
///
/// ```
/// use cvmath::Point3;
///
/// let splat = cvmath::Point3!(3);
/// let zero: Point3<i32> = cvmath::Point3!();
///
/// assert_eq!(splat, Point3(3, 3, 3));
/// assert_eq!(zero, Point3::ZERO);
/// ```
#[macro_export]
macro_rules! Point3 {
	($value:expr) => {
		$crate::Point3 { x: $value, y: $value, z: $value }
	};
	() => {
		$crate::Point3::ZERO
	};
}

specialized_type!(Point3, Point3f, f32, x, y, z);
specialized_type!(Point3, Point3d, f64, x, y, z);
specialized_type!(Point3, Point3i, i32, x, y, z);

//----------------------------------------------------------------

// Points are not solid
impl<T: Float> Trace3<T> for Point3<T> {
	#[inline]
	fn inside(&self, _pt: Point3<T>) -> bool {
		false
	}

	#[inline]
	fn trace(&self, _ray: &Ray3<T>) -> Option<Hit3<T>> {
		None
	}
}
