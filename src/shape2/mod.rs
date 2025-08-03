/// 2D shapes.

use super::*;

mod bounds;
mod line;
mod point;

pub use self::bounds::Bounds2;
pub use self::point::Point2;
pub use self::line::Line2;

#[doc(hidden)]
pub use self::point::{Point2f, Point2d, Point2i};
#[doc(hidden)]
pub use self::line::{Line2f, Line2d, Line2i};
#[doc(hidden)]
pub use self::bounds::{Bounds2f, Bounds2d, Bounds2i};
