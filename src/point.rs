/*!
Affine point.
*/

use ::vec::{Vec2, Vec3};

pub type Point2<T> = Vec2<T>;
pub type Point3<T> = Vec3<T>;

#[allow(non_snake_case)]
pub fn Point2<T>(x: T, y: T) -> Point2<T> {
	Point2 { x, y }
}
#[allow(non_snake_case)]
pub fn Point3<T>(x: T, y: T, z: T) -> Point3<T> {
	Point3 { x, y, z }
}
