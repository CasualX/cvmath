/*!
Numeric traits.
*/

use std::{cmp, fmt, ops};

mod as_cast;

pub trait Zero where Self: Sized + ops::Add<Output = Self> + ops::Mul<Output = Self> {
	fn zero() -> Self;
}
pub trait One where Self: Sized + ops::Mul<Output = Self> {
	fn one() -> Self;
}
pub trait Extrema<Rhs = Self>: Sized {
	type Output: Extrema;
	fn min(self, rhs: Rhs) -> Self::Output;
	fn max(self, rhs: Rhs) -> Self::Output;
	fn min_max(self, rhs: Rhs) -> (Self::Output, Self::Output);
	fn clamp(self, min: Rhs, max: Rhs) -> Self::Output where Self::Output: Extrema<Rhs, Output = Self::Output> {
		self.min(min).max(max)
	}
}
pub trait SpatialOrd<Rhs = Self> {
	fn spatial_lt(&self, rhs: &Rhs) -> bool;
	fn spatial_le(&self, rhs: &Rhs) -> bool;
	fn spatial_gt(&self, rhs: &Rhs) -> bool;
	fn spatial_ge(&self, rhs: &Rhs) -> bool;
}
pub trait Abs {
	type Output;
	fn abs(self) -> Self::Output;
}

pub use self::as_cast::AsCast;

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
		impl Zero for $ty {
			fn zero() -> $ty { 0.0 }
		}
		impl One for $ty {
			fn one() -> $ty { 1.0 }
		}
		impl Extrema<$ty> for $ty {
			type Output = $ty;
			fn min(self, rhs: $ty) -> $ty { if self < rhs { self } else { rhs } }
			fn max(self, rhs: $ty) -> $ty { if self > rhs { self } else { rhs } }
			fn min_max(self, rhs: $ty) -> ($ty, $ty) { if self < rhs { (self, rhs) } else { (rhs, self) } }
		}
		impl SpatialOrd<$ty> for $ty {
			fn spatial_lt(&self, rhs: &$ty) -> bool { *self < *rhs }
			fn spatial_le(&self, rhs: &$ty) -> bool { *self <= *rhs }
			fn spatial_gt(&self, rhs: &$ty) -> bool { *self > *rhs }
			fn spatial_ge(&self, rhs: &$ty) -> bool { *self >= *rhs }
		}
		impl<'a> Extrema<&'a $ty> for &'a $ty {
			type Output = &'a $ty;
			fn min(self, rhs: &'a $ty) -> &'a $ty { if self < rhs { self } else { rhs } }
			fn max(self, rhs: &'a $ty) -> &'a $ty { if self > rhs { self } else { rhs } }
			fn min_max(self, rhs: &'a $ty) -> (&'a $ty, &'a $ty) { if self < rhs { (self, rhs) } else { (rhs, self) } }
		}
		impl<'a> SpatialOrd<&'a $ty> for &'a $ty {
			fn spatial_lt(&self, rhs: &&'a $ty) -> bool { **self < **rhs }
			fn spatial_le(&self, rhs: &&'a $ty) -> bool { **self <= **rhs }
			fn spatial_gt(&self, rhs: &&'a $ty) -> bool { **self > **rhs }
			fn spatial_ge(&self, rhs: &&'a $ty) -> bool { **self >= **rhs }
		}
		impl Abs for $ty {
			type Output = $ty;
			fn abs(self) -> $ty { self.abs() }
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
		impl Zero for $ty {
			fn zero() -> $ty { 0 }
		}
		impl One for $ty {
			fn one() -> $ty { 1 }
		}
		impl Extrema<$ty> for $ty {
			type Output = $ty;
			fn min(self, rhs: $ty) -> $ty { cmp::min(self, rhs) }
			fn max(self, rhs: $ty) -> $ty { cmp::max(self, rhs) }
			fn min_max(self, rhs: $ty) -> ($ty, $ty) { (cmp::min(self, rhs), cmp::max(self, rhs)) }
		}
		impl SpatialOrd<$ty> for $ty {
			fn spatial_lt(&self, rhs: &$ty) -> bool { *self < *rhs }
			fn spatial_le(&self, rhs: &$ty) -> bool { *self <= *rhs }
			fn spatial_gt(&self, rhs: &$ty) -> bool { *self > *rhs }
			fn spatial_ge(&self, rhs: &$ty) -> bool { *self >= *rhs }
		}
		impl<'a> Extrema<&'a $ty> for &'a $ty {
			type Output = &'a $ty;
			fn min(self, rhs: &'a $ty) -> &'a $ty { cmp::min(self, rhs) }
			fn max(self, rhs: &'a $ty) -> &'a $ty { cmp::max(self, rhs) }
			fn min_max(self, rhs: &'a $ty) -> (&'a $ty, &'a $ty) { (cmp::min(self, rhs), cmp::max(self, rhs)) }
		}
		impl<'a> SpatialOrd<&'a $ty> for &'a $ty {
			fn spatial_lt(&self, rhs: &&'a $ty) -> bool { **self < **rhs }
			fn spatial_le(&self, rhs: &&'a $ty) -> bool { **self <= **rhs }
			fn spatial_gt(&self, rhs: &&'a $ty) -> bool { **self > **rhs }
			fn spatial_ge(&self, rhs: &&'a $ty) -> bool { **self >= **rhs }
		}
		impl Abs for $ty {
			type Output = $ty;
			fn abs(self) -> $ty { self.abs() }
		}
		impl Scalar for $ty {}
		impl Int for $ty {}
	}
}

int!(i32);
int!(i64);
float!(f32);
float!(f64);
