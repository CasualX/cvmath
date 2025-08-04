/*!
Numeric traits.
*/

use std::{cmp, fmt, ops};

mod zero;
mod one;
mod cast;
mod extrema;
mod abs;
mod float_ops;

pub use self::zero::Zero;
pub use self::one::One;
pub use self::cast::{CastFrom, CastTo};
pub use self::extrema::Extrema;
pub use self::abs::Abs;
pub use self::float_ops::FloatOps;

pub trait Scalar where Self
	: Copy + Default + Zero + One
	+ fmt::Display + fmt::Debug
	+ ops::Add<Output = Self> + ops::Sub<Output = Self>
	+ ops::Mul<Output = Self> + ops::Div<Output = Self>
	+ ops::Neg<Output = Self> + ops::Rem<Output = Self>
	+ ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign
	+ Extrema + Abs<Output = Self>
	+ cmp::PartialEq + cmp::PartialOrd
{
	fn mul_add(self, a: Self, b: Self) -> Self;
}

#[allow(dead_code)]
pub trait Int where Self : Scalar + cmp::Eq + cmp::Ord {}

pub trait Float where Self : Scalar + FloatOps + CastFrom<f64> {
	const TWO: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
	const EPSILON: Self;
	const THREE_SIXTY: Self;
	const TAU: Self;
	const PI: Self;
	const FRAC_PI_2: Self;
}

//----------------------------------------------------------------
// Implementation

impl Scalar for i8 {
	#[inline]
	fn mul_add(self, a: i8, b: i8) -> i8 {
		self.wrapping_mul(a).wrapping_add(b)
	}
}
impl Scalar for i16 {
	#[inline]
	fn mul_add(self, a: i16, b: i16) -> i16 {
		self.wrapping_mul(a).wrapping_add(b)
	}
}
impl Scalar for i32 {
	#[inline]
	fn mul_add(self, a: i32, b: i32) -> i32 {
		self.wrapping_mul(a).wrapping_add(b)
	}
}
impl Scalar for i64 {
	#[inline]
	fn mul_add(self, a: i64, b: i64) -> i64 {
		self.wrapping_mul(a).wrapping_add(b)
	}
}

impl Scalar for f32 {
	#[inline]
	fn mul_add(self, a: f32, b: f32) -> f32 {
		self.mul_add(a, b)
	}
}
impl Scalar for f64 {
	#[inline]
	fn mul_add(self, a: f64, b: f64) -> f64 {
		self.mul_add(a, b)
	}
}

impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}

impl Float for f32 {
	const TWO: f32 = 2.0;
	const INFINITY: f32 = f32::INFINITY;
	const NEG_INFINITY: f32 = f32::NEG_INFINITY;
	const EPSILON: f32 = f32::EPSILON;
	const THREE_SIXTY: f32 = 360.0;
	const TAU: f32 = std::f32::consts::TAU;
	const PI: f32 = std::f32::consts::PI;
	const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2;
}
impl Float for f64 {
	const TWO: f64 = 2.0;
	const INFINITY: f64 = f64::INFINITY;
	const NEG_INFINITY: f64 = f64::NEG_INFINITY;
	const EPSILON: f64 = f64::EPSILON;
	const THREE_SIXTY: f64 = 360.0;
	const TAU: f64 = std::f64::consts::TAU;
	const PI: f64 = std::f64::consts::PI;
	const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
}
