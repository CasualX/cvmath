/// 3D shapes.

use super::*;

mod point;
mod bounds;
mod line;
mod plane;
mod sphere;
mod triangle;
mod ray;
mod shape;

pub use self::point::Point3;
pub use self::bounds::Bounds3;
pub use self::line::Line3;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle3;
pub use self::ray::{Ray, TraceHit, TraceRay};
pub use self::shape::Shape3;

#[doc(hidden)]
pub use self::point::{Point3f, Point3d, Point3i};

#[doc(hidden)]
pub use self::bounds::{Bounds3f, Bounds3d, Bounds3i};
