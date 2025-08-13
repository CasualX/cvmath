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
mod interval;
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
pub use crate::mat::{Mat2f, Mat3f, Mat4f, Transform2f, Transform3f, Mat2d, Mat3d, Mat4d, Transform2d, Transform3d};
#[doc(hidden)]
pub use crate::quat::{Quatf, Quatd};

#[cfg(test)]
mod tests;
