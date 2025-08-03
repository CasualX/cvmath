use super::*;

/// Point3 shape.
pub type Point3<T> = Vec3<T>;

/// Point3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Point3<T>(x: T, y: T, z: T) -> Point3<T> {
	Point3 { x, y, z }
}

specialized_type!(Point3, Point3f, f32, x, y, z);
specialized_type!(Point3, Point3d, f64, x, y, z);
specialized_type!(Point3, Point3i, i32, x, y, z);
