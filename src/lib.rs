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

pub use self::vec::{Vec2, Vec3, Vec4};
pub use self::mask::{Mask2, Mask3, Mask4};
pub use self::mat::{Affine2};
pub use self::euler::{Euler};

pub use self::point::{Point2, Point3};
pub use self::rect::{Rect, Box};
pub use self::line::{Line2, Line3};
