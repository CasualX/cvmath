/*!
*/

mod macros;

pub mod num;
pub mod angle;

pub mod vec;
pub mod mask;
pub mod mat;
pub mod euler;

pub mod point;
pub mod rect;
pub mod line;

pub mod prelude {
	pub use ::vec::{Vec2, Vec3, Vec4};
	pub use ::mask::{Mask2, Mask3, Mask4};
	pub use ::mat::{Mat2, Affine2};
	pub use ::euler::{Euler};

	pub use ::point::{Point2, Point3};
	pub use ::rect::{Rect, Box};
	pub use ::line::{Line2, Line3};
}
