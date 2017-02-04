/*!
*/

// #![no_std]

macro_rules! infix {
	($op:tt $e:expr) => ($e);
	($op:tt $e:expr, $($tail:expr),*) => ($e $op infix!($op $($tail),*));
}

pub mod vec;
pub mod point;
pub mod bools;

pub mod num;

pub use self::vec::{Vec2, Vec3, Vec4};
pub use self::point::{Point2, Point3};
pub use self::bools::{Bools2, Bools3, Bools4};
