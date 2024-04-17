/*!
Computer Vector Graphics Math Library
=====================================
*/

use std::{fmt, mem, ops, slice};
use std::str::FromStr;

mod macros;

mod num;
mod angle;

mod vec;
mod bools;
mod packed;
mod mat;
mod quaternion;

#[cfg(feature = "qangle")]
mod qangle;

mod point;
mod bounds;
mod line;
mod plane;
mod sphere;
mod ray;

use crate::num::*;
use crate::angle::Angle;

pub use crate::angle::{Rad, Deg};
pub use crate::vec::{Vec2, Vec3, Vec4, X, Y, Z, W};
pub use crate::bools::{Bool2, Bool3, Bool4};
pub use crate::mat::{Mat2, Mat3, Mat4, Transform2, Transform3, Hand, Clip};
pub use crate::mat::{Hand::*, Clip::*};
pub use crate::quaternion::Quaternion;

#[cfg(feature = "qangle")]
pub use crate::qangle::QAngle;

pub use crate::point::{Point2, Point3};
pub use crate::bounds::{Bounds, Rect, Cuboid};
pub use crate::line::{Line, Line2, Line3};
pub use crate::plane::Plane;
pub use crate::sphere::Sphere;
pub use crate::ray::{Ray, TraceHit, TraceRay};

#[doc(hidden)]
pub use crate::vec::{Vec2f, Vec3f, Vec4f, Vec2d, Vec3d, Vec4d, Vec2i, Vec3i, Vec4i};
#[doc(hidden)]
pub use crate::point::{Point2f, Point3f, Point2d, Point3d, Point2i, Point3i};
#[doc(hidden)]
pub use crate::mat::{Mat2f, Mat3f, Mat4f, Transform2f, Transform3f, Mat2d, Mat3d, Mat4d, Transform2d, Transform3d};
#[doc(hidden)]
pub use crate::quaternion::{Quaternionf, Quaterniond};
