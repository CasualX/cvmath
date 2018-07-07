/*!
*/

mod macros;

pub mod num;
pub mod angle;

pub mod vec;
pub mod bools;
mod packed;
pub mod mat;
pub mod euler;

pub mod point;
pub mod bounds;
pub mod line2;

pub mod prelude {
	pub use angle::{Rad, Deg, Angle};
	pub use vec::{Vec2, Vec3, Vec4, X, Y, Z, W};
	pub use bools::{Bool2, Bool3, Bool4};
	pub use mat::{Mat2, Affine2, Mat3, Affine3};
	pub use euler::{Euler};

	pub use point::{Point2, Point3};
	pub use bounds::{Bounds, Rect, Cuboid};
	pub use line2::{self, Line2};
}
