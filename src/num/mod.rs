/*!
Numeric traits.
*/

use std::{cmp, fmt, ops};

mod zero;
mod one;
mod as_cast;
mod extrema;
mod abs;
mod spatial_ord;
mod float_ops;

pub use self::zero::Zero;
pub use self::one::One;
pub use self::as_cast::AsCast;
pub use self::extrema::Extrema;
pub use self::abs::Abs;
pub use self::spatial_ord::SpatialOrd;
pub use self::float_ops::FloatOps;

pub trait Scalar where Self
	: Copy + Default + Zero + One
	+ fmt::Display + fmt::Debug
	+ ops::Add<Output = Self> + ops::Sub<Output = Self>
	+ ops::Mul<Output = Self> + ops::Div<Output = Self>
	+ ops::Neg<Output = Self> + ops::Rem<Output = Self>
	+ ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign
	+ Extrema<Output = Self> + Abs<Output = Self>
	+ cmp::PartialEq + cmp::PartialOrd {}

pub trait Int where Self
	: Scalar + cmp::Eq + cmp::Ord {}

pub trait Float where Self
	: Scalar + FloatOps
{
	fn literal(f: f64) -> Self;
}

//----------------------------------------------------------------
// Implementation

impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}

impl Scalar for f32 {}
impl Scalar for f64 {}

impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}

impl Float for f32 {
	fn literal(f: f64) -> f32 { f as f32 }
}
impl Float for f64 {
	fn literal(f: f64) -> f64 { f }
}
