/*!
Numeric traits.
*/

use std::{cmp, fmt, ops};

mod zero;
mod one;
mod as_cast;
mod extrema;
mod abs;

pub use self::zero::Zero;
pub use self::one::One;
pub use self::as_cast::AsCast;
pub use self::extrema::Extrema;
pub use self::abs::Abs;

pub trait SpatialOrd<Rhs = Self> {
	fn spatial_lt(&self, rhs: &Rhs) -> bool;
	fn spatial_le(&self, rhs: &Rhs) -> bool;
	fn spatial_gt(&self, rhs: &Rhs) -> bool;
	fn spatial_ge(&self, rhs: &Rhs) -> bool;
}

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
pub trait Float where Self: Scalar {
	fn literal(f: f64) -> Self;
	fn is_finite(self) -> bool;
	fn is_infinite(self) -> bool;
	fn sqrt(self) -> Self;
	fn remainder(self, Self) -> Self;
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn sin_cos(self) -> (Self, Self);
	fn asin(self) -> Self;
	fn acos(self) -> Self;
	fn atan(self) -> Self;
	fn atan2(self, Self) -> Self;
}

//----------------------------------------------------------------

macro_rules! float {
	($ty:ty) => {
		impl SpatialOrd<$ty> for $ty {
			fn spatial_lt(&self, rhs: &$ty) -> bool { *self < *rhs }
			fn spatial_le(&self, rhs: &$ty) -> bool { *self <= *rhs }
			fn spatial_gt(&self, rhs: &$ty) -> bool { *self > *rhs }
			fn spatial_ge(&self, rhs: &$ty) -> bool { *self >= *rhs }
		}
		impl<'a> SpatialOrd<&'a $ty> for &'a $ty {
			fn spatial_lt(&self, rhs: &&'a $ty) -> bool { **self < **rhs }
			fn spatial_le(&self, rhs: &&'a $ty) -> bool { **self <= **rhs }
			fn spatial_gt(&self, rhs: &&'a $ty) -> bool { **self > **rhs }
			fn spatial_ge(&self, rhs: &&'a $ty) -> bool { **self >= **rhs }
		}
		impl Scalar for $ty {}
		impl Float for $ty {
			fn literal(f: f64) -> $ty { f as $ty }
			fn is_finite(self) -> bool { self.is_finite() }
			fn is_infinite(self) -> bool { self.is_infinite() }
			fn sqrt(self) -> $ty { self.sqrt() }
			fn remainder(self, y: $ty) -> $ty {
				self - ((self / y).round() * y)
			}
			fn sin(self) -> $ty { self.sin() }
			fn cos(self) -> $ty { self.cos() }
			fn tan(self) -> $ty { self.tan() }
			fn sin_cos(self) -> ($ty, $ty) { self.sin_cos() }
			fn asin(self) -> $ty { self.asin() }
			fn acos(self) -> $ty { self.acos() }
			fn atan(self) -> $ty { self.atan() }
			fn atan2(self, x: $ty) -> $ty { self.atan2(x) }
		}
	};
}

macro_rules! int {
	($ty:ty) => {
		impl SpatialOrd<$ty> for $ty {
			fn spatial_lt(&self, rhs: &$ty) -> bool { *self < *rhs }
			fn spatial_le(&self, rhs: &$ty) -> bool { *self <= *rhs }
			fn spatial_gt(&self, rhs: &$ty) -> bool { *self > *rhs }
			fn spatial_ge(&self, rhs: &$ty) -> bool { *self >= *rhs }
		}
		impl<'a> SpatialOrd<&'a $ty> for &'a $ty {
			fn spatial_lt(&self, rhs: &&'a $ty) -> bool { **self < **rhs }
			fn spatial_le(&self, rhs: &&'a $ty) -> bool { **self <= **rhs }
			fn spatial_gt(&self, rhs: &&'a $ty) -> bool { **self > **rhs }
			fn spatial_ge(&self, rhs: &&'a $ty) -> bool { **self >= **rhs }
		}
		impl Scalar for $ty {}
		impl Int for $ty {}
	}
}

int!(i32);
int!(i64);
float!(f32);
float!(f64);
