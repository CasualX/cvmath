/*!
Numeric traits.
*/

use ::std::{cmp, ops};

pub trait Zero {
	fn zero() -> Self;
}
pub trait One {
	fn one() -> Self;
}
pub trait Min<Rhs = Self> {
	type Output;
	fn min(self, rhs: Rhs) -> Self::Output;
}
pub trait Max<Rhs = Self> {
	type Output;
	fn max(self, rhs: Rhs) -> Self::Output;
}
pub trait Abs {
	type Output;
	fn abs(self) -> Self::Output;
}

pub trait Cast<T> {
	fn cast(self) -> T;
}

pub trait Scalar: Copy + Default + Zero + One +
	ops::Add<Output = Self> + ops::Sub<Output = Self> +
	ops::Mul<Output = Self> + ops::Div<Output = Self> +
	ops::Neg<Output = Self> + ops::Rem<Output = Self> +
	Min<Output = Self> + Max<Output = Self> + Abs<Output = Self> +
	cmp::PartialEq + cmp::PartialOrd {}

pub trait Int: cmp::Eq + cmp::Ord {}
pub trait Float {
	fn sqrt(self) -> Self;
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
		impl Min<$ty> for $ty {
			type Output = $ty;
			fn min(self, rhs: $ty) -> $ty { if self < rhs { self } else { rhs } }
		}
		impl Max<$ty> for $ty {
			type Output = $ty;
			fn max(self, rhs: $ty) -> $ty { if self > rhs { self } else { rhs } }
		}
		impl Abs for $ty {
			type Output = $ty;
			fn abs(self) -> $ty { self.abs() }
		}
		impl Cast<i32> for $ty {
			fn cast(self) -> i32 { self as i32 }
		}
		impl Cast<i64> for $ty {
			fn cast(self) -> i64 { self as i64 }
		}
		impl Cast<f32> for $ty {
			fn cast(self) -> f32 { self as f32 }
		}
		impl Cast<f64> for $ty {
			fn cast(self) -> f64 { self as f64 }
		}
		impl Scalar for $ty {}
		impl Float for $ty {
			fn sqrt(self) -> $ty {
				self.sqrt()
			}
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
		impl Min<$ty> for $ty {
			type Output = $ty;
			fn min(self, rhs: $ty) -> $ty {
				cmp::min(self, rhs)
			}
		}
		impl Max<$ty> for $ty {
			type Output = $ty;
			fn max(self, rhs: $ty) -> $ty {
				cmp::max(self, rhs)
			}
		}
		impl Abs for $ty {
			type Output = $ty;
			fn abs(self) -> $ty {
				self.abs()
			}
		}
		impl Cast<i32> for $ty {
			fn cast(self) -> i32 { self as i32 }
		}
		impl Cast<i64> for $ty {
			fn cast(self) -> i64 { self as i64 }
		}
		impl Cast<f32> for $ty {
			fn cast(self) -> f32 { self as f32 }
		}
		impl Cast<f64> for $ty {
			fn cast(self) -> f64 { self as f64 }
		}
		impl Scalar for $ty {}
		impl Int for $ty {}
	}
}

int!(i32);
int!(i64);
float!(f32);
float!(f64);
