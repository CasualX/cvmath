use std::ops;

use super::{Mat2, Affine2, Mat3, Affine3};
use vec::{Vec2, Vec3};

pub trait Transform2<T>
	: ops::Mul<Vec2<T>, Output = Vec2<T>>
	+ ops::Mul<Mat2<T>>
	+ ops::Mul<Affine2<T>> {}

pub trait Transform3<T>
	: ops::Mul<Vec3<T>, Output = Vec3<T>>
	+ ops::Mul<Mat3<T>>
	+ ops::Mul<Affine3<T>> {}
