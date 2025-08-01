/*!
Computer Vector Graphics Math Library
=====================================
*/

use std::{fmt, mem, ops, slice};
use std::error::Error;
use std::str::FromStr;

mod macros;

mod num;
mod angle;

mod vec;
mod bools;
mod packed;
mod complex;
mod polar;
mod mat;
mod quat;
mod rotvec;

mod point;
mod bounds;
mod line;
mod plane;
mod sphere;
mod triangle;
mod ray;
mod shape;

#[cfg(test)]
mod tests;

use crate::num::*;

pub use crate::angle::Angle;
pub use crate::vec::{Vec2, Vec3, Vec4, X, Y, Z, W};
pub use crate::bools::{Bool2, Bool3, Bool4};
pub use crate::complex::Complex;
pub use crate::polar::Polar;
pub use crate::mat::{Mat2, Mat3, Mat4, Transform2, Transform3, Hand, Clip};
pub use crate::quat::Quat;
pub use crate::rotvec::RotationVector;

pub use crate::point::{Point2, Point3};
pub use crate::bounds::{Bounds, Bounds2, Bounds3};
pub use crate::line::{Line, Line2, Line3};
pub use crate::plane::Plane;
pub use crate::sphere::Sphere;
pub use crate::triangle::{Triangle, Triangle2, Triangle3};
pub use crate::ray::{Ray, TraceHit, TraceRay};
pub use crate::shape::Shape3;

#[doc(hidden)]
pub use crate::angle::{Anglef, Angled};
#[doc(hidden)]
pub use crate::vec::{Vec2f, Vec3f, Vec4f, Vec2d, Vec3d, Vec4d, Vec2i, Vec3i, Vec4i};
#[doc(hidden)]
pub use crate::point::{Point2f, Point3f, Point2d, Point3d, Point2i, Point3i};
#[doc(hidden)]
pub use crate::complex::{Complexf, Complexd};
#[doc(hidden)]
pub use crate::mat::{Mat2f, Mat3f, Mat4f, Transform2f, Transform3f, Mat2d, Mat3d, Mat4d, Transform2d, Transform3d};
#[doc(hidden)]
pub use crate::quat::{Quatf, Quatd};

#[doc(hidden)]
pub use crate::bounds::{Bounds2f, Bounds3f, Bounds2d, Bounds3d, Bounds2i, Bounds3i};
