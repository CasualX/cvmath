/*!
*/

mod macros;

pub mod num;
pub mod angle;

pub mod vec;
pub mod bools;
pub mod mat;
pub mod euler;

pub mod point;
pub mod rect;
pub mod line;
pub mod poly;

pub mod prelude {
	pub use ::angle::{Rad, Deg, Angle};
	pub use ::vec::{Vec2, Vec3, Vec4};
	pub use ::bools::{Bool2, Bool3, Bool4};
	pub use ::mat::{Mat2, Affine2, Mat3, Affine3};
	pub use ::euler::{Euler};

	pub use ::point::{Point2, Point3};
	pub use ::rect::{Rect, Cuboid};
	pub use ::line::{Line2, Line3};
	pub use ::poly::{Polyline2, Polyline3, Polygon2, Polygon3};
}
