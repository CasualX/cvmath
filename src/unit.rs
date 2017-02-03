// Numeric traits plz...

use ::std::{ops, cmp};

/// Coordinate units
///
/// This marker trait tags types used as coordinates.
pub trait Unit: Sized + Copy + Default +
	ops::Add<Output = Self> + ops::Sub<Output = Self> +
	ops::Mul<Output = Self> + ops::Div<Output = Self> +
	ops::Neg<Output = Self> + ops::Rem<Output = Self> +
	cmp::PartialEq + cmp::PartialOrd
{
	fn zero() -> Self;
	fn one() -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn abs(self) -> Self;
}

impl Unit for i32 {
	fn zero() -> i32 { 0i32 }
	fn one() -> i32 { 1i32 }
	fn min(self, rhs: i32) -> i32 { cmp::min(self, rhs) }
	fn max(self, rhs: i32) -> i32 { cmp::max(self, rhs) }
	fn abs(self) -> i32 { self.abs() }
}
impl Unit for f32 {
	fn zero() -> f32 { 0f32 }
	fn one() -> f32 { 1f32 }
	fn min(self, rhs: f32) -> f32 { if self < rhs { self } else { rhs } }
	fn max(self, rhs: f32) -> f32 { if self > rhs { self } else { rhs } }
	fn abs(self) -> f32 { self.abs() }
}
impl Unit for i64 {
	fn zero() -> i64 { 0i64 }
	fn one() -> i64 { 1i64 }
	fn min(self, rhs: i64) -> i64 { cmp::min(self, rhs) }
	fn max(self, rhs: i64) -> i64 { cmp::max(self, rhs) }
	fn abs(self) -> i64 { self.abs() }
}
impl Unit for f64 {
	fn zero() -> f64 { 0f64 }
	fn one() -> f64 { 1f64 }
	fn min(self, rhs: f64) -> f64 { if self < rhs { self } else { rhs } }
	fn max(self, rhs: f64) -> f64 { if self > rhs { self } else { rhs } }
	fn abs(self) -> f64 { self.abs() }
}

pub trait Int: cmp::Eq + cmp::Ord {}
impl Int for i32 {}
impl Int for i64 {}

pub trait Float {
	fn sqrt(self) -> Self;
}
impl Float for f32 {
	fn sqrt(self) -> f32 { self.sqrt() }
}
impl Float for f64 {
	fn sqrt(self) -> f64 { self.sqrt() }
}

pub trait Cast<T> {
	fn cast(self) -> T;
}
impl Cast<i32> for i32 { fn cast(self) -> i32 { self } }
impl Cast<i64> for i32 { fn cast(self) -> i64 { self as i64 } }
impl Cast<f32> for i32 { fn cast(self) -> f32 { self as f32 } }
impl Cast<f64> for i32 { fn cast(self) -> f64 { self as f64 } }

impl Cast<i32> for f32 { fn cast(self) -> i32 { self as i32 } }
impl Cast<i64> for f32 { fn cast(self) -> i64 { self as i64 } }
impl Cast<f32> for f32 { fn cast(self) -> f32 { self } }
impl Cast<f64> for f32 { fn cast(self) -> f64 { self as f64 } }

impl Cast<i32> for i64 { fn cast(self) -> i32 { self as i32 } }
impl Cast<i64> for i64 { fn cast(self) -> i64 { self } }
impl Cast<f32> for i64 { fn cast(self) -> f32 { self as f32 } }
impl Cast<f64> for i64 { fn cast(self) -> f64 { self as f64 } }

impl Cast<i32> for f64 { fn cast(self) -> i32 { self as i32 } }
impl Cast<i64> for f64 { fn cast(self) -> i64 { self as i64 } }
impl Cast<f32> for f64 { fn cast(self) -> f32 { self as f32 } }
impl Cast<f64> for f64 { fn cast(self) -> f64 { self } }
