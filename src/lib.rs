/*!
Graphics-oriented math types for vectors, transforms, and ray tracing queries.

If you are looking for a specific part of the API:

- Vectors and masks: [struct@Vec2], [struct@Vec3], [struct@Vec4], [tyalias@Bool2], [tyalias@Bool3], [tyalias@Bool4]
- Angles and rotations: [struct@Angle], [struct@Complex], [struct@Polar], [struct@Quat], [struct@RotationVector]
- Matrices and transforms: [struct@Mat2], [struct@Mat3], [struct@Mat4], [struct@Transform2], [struct@Transform3], [enum@Hand], [enum@Clip]
- 2D geometry: [tyalias@Point2], [struct@Bounds2], [struct@Line2], [struct@Plane2], [struct@Circle], [struct@Triangle2], [enum@Shape2], [struct@Ray2], [struct@Hit2], [trait@Trace2], [struct@Bvh2]
- 3D geometry: [tyalias@Point3], [struct@Bounds3], [struct@Line3], [struct@Plane3], [struct@Sphere], [struct@Triangle3], [enum@Shape3], [struct@Ray3], [struct@Hit3], [trait@Trace3], [struct@Bvh3]
- Ray tracing utilities: [struct@Interval], [enum@HitSide]

Most implementations are generic over scalar types (integers and floats) and floating-point types.
*/

#![debugger_visualizer(natvis_file = "natvis/Angle.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Complex.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Polar.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Quat.natvis")]
#![debugger_visualizer(natvis_file = "natvis/RotationVector.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Mat2.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Mat3.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Mat4.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Transform2.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Transform3.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Vec2.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Vec3.natvis")]
#![debugger_visualizer(natvis_file = "natvis/Vec4.natvis")]

use std::{fmt, mem, ops, slice};
use std::error::Error;
use std::str::FromStr;

mod macros;

mod num;
mod angle;
pub mod scalar;

mod vec;
mod bools;
mod packed;
mod interval;
mod lerp;
mod ray;
mod complex;
mod polar;
mod mat;
mod quat;
mod rotvec;

mod shape2;
mod shape3;

use crate::num::*;

pub use crate::angle::Angle;
pub use crate::vec::{Vec2, Vec3, Vec4, X, Y, Z, W};
pub use crate::bools::{Bool2, Bool3, Bool4};
pub use crate::interval::Interval;
pub use crate::lerp::*;
pub use crate::ray::HitSide;
pub use crate::complex::Complex;
pub use crate::polar::Polar;
pub use crate::mat::{Mat2, Mat3, Mat4, Transform2, Transform3, Hand, Clip};
pub use crate::quat::Quat;
pub use crate::rotvec::RotationVector;

pub use crate::shape2::*;
pub use crate::shape3::*;

#[doc(hidden)]
pub use crate::angle::{Anglef, Angled};
#[doc(hidden)]
pub use crate::vec::{Vec2f, Vec3f, Vec4f, Vec2d, Vec3d, Vec4d, Vec2i, Vec3i, Vec4i};
#[doc(hidden)]
pub use crate::complex::{Complexf, Complexd};
#[doc(hidden)]
pub use crate::polar::{Polarf, Polard};
#[doc(hidden)]
pub use crate::mat::{Mat2f, Mat3f, Mat4f, Transform2f, Transform3f, Mat2d, Mat3d, Mat4d, Transform2d, Transform3d};
#[doc(hidden)]
pub use crate::quat::{Quatf, Quatd};

#[cfg(test)]
mod tests;
