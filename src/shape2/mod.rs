/// 2D shapes.

use super::*;

mod bounds;
mod line;
mod plane;
mod point;
mod circle;
mod triangle;
mod ray;
mod shape;
mod union;

pub use self::bounds::Bounds2;
pub use self::point::Point2;
pub use self::plane::Plane2;
pub use self::line::Line2;
pub use self::circle::Circle;
pub use self::triangle::Triangle2;
pub use self::ray::{Ray2, Hit2, Trace2};
pub use self::shape::Shape2;

#[doc(hidden)]
pub use self::point::{Point2f, Point2d, Point2i};
#[doc(hidden)]
pub use self::line::{Line2f, Line2d, Line2i};
#[doc(hidden)]
pub use self::bounds::{Bounds2f, Bounds2d, Bounds2i};

#[cfg(test)]
mod tests;
