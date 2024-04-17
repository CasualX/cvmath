/*!
Affine point.
*/

use super::*;

/// 2D point.
pub type Point2<T> = Vec2<T>;

/// 3D point.
pub type Point3<T> = Vec3<T>;

/// Point2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Point2<T>(x: T, y: T) -> Point2<T> {
	Point2 { x, y }
}

/// Point3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Point3<T>(x: T, y: T, z: T) -> Point3<T> {
	Point3 { x, y, z }
}

specialized_type!(Point2, Point2f, f32, x, y);
specialized_type!(Point3, Point3f, f32, x, y, z);

specialized_type!(Point2, Point2d, f64, x, y);
specialized_type!(Point3, Point3d, f64, x, y, z);

specialized_type!(Point2, Point2i, i32, x, y);
specialized_type!(Point3, Point3i, i32, x, y, z);
