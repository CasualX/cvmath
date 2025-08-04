use std::cmp;

pub trait FloatOps: Copy {
	#[must_use]
	fn total_cmp(&self, rhs: &Self) -> cmp::Ordering;
	#[must_use]
	fn is_finite(self) -> bool;
	#[must_use]
	fn is_infinite(self) -> bool;
	#[must_use]
	fn signum(self) -> Self;
	#[must_use]
	fn sqrt(self) -> Self;
	#[must_use]
	fn exp(self) -> Self;
	#[must_use]
	fn floor(self) -> Self;
	#[must_use]
	fn ln(self) -> Self;
	#[must_use]
	fn powf(self, exp: Self) -> Self;
	#[must_use]
	fn ceil(self) -> Self;
	#[must_use]
	fn round(self) -> Self;
	#[must_use]
	fn fract(self) -> Self;
	#[must_use]
	fn sin(self) -> Self;
	#[must_use]
	fn cos(self) -> Self;
	#[must_use]
	fn tan(self) -> Self;
	#[must_use]
	fn sin_cos(self) -> (Self, Self);
	#[must_use]
	fn asin(self) -> Self;
	#[must_use]
	fn acos(self) -> Self;
	#[must_use]
	fn atan(self) -> Self;
	#[must_use]
	fn atan2(self, _: Self) -> Self;

	/// Returns true if the values are approximately equal.
	///
	/// The comparison is done with both absolute tolerances (for absolute values below 1.0) and relative tolerances (for absolute values above 1.0).
	#[must_use]
	fn is_close(self, other: Self) -> bool;
}

//----------------------------------------------------------------
// Implementation

macro_rules! impl_float_ops {
	($ty:ty) => {
		impl FloatOps for $ty {
			#[inline]
			fn total_cmp(&self, rhs: &Self) -> cmp::Ordering {
				<$ty>::total_cmp(self, rhs)
			}

			#[inline]
			fn is_finite(self) -> bool {
				self.is_finite()
			}
			#[inline]
			fn is_infinite(self) -> bool {
				self.is_infinite()
			}
			#[inline]
			fn signum(self) -> $ty {
				self.signum()
			}
			#[inline]
			fn sqrt(self) -> $ty {
				self.sqrt()
			}
			#[inline]
			fn exp(self) -> $ty {
				self.exp()
			}
			#[inline]
			fn ln(self) -> $ty {
				self.ln()
			}
			#[inline]
			fn powf(self, exp: $ty) -> $ty {
				self.powf(exp)
			}
			#[inline]
			fn floor(self) -> $ty {
				self.floor()
			}
			#[inline]
			fn ceil(self) -> $ty {
				self.ceil()
			}
			#[inline]
			fn round(self) -> $ty {
				self.round()
			}
			#[inline]
			fn fract(self) -> $ty {
				self.fract()
			}
			#[inline]
			fn sin(self) -> $ty {
				self.sin()
			}
			#[inline]
			fn cos(self) -> $ty {
				self.cos()
			}
			#[inline]
			fn tan(self) -> $ty {
				self.tan()
			}
			#[inline]
			fn sin_cos(self) -> ($ty, $ty) {
				self.sin_cos()
			}
			#[inline]
			fn asin(self) -> $ty {
				self.asin()
			}
			#[inline]
			fn acos(self) -> $ty {
				self.acos()
			}
			#[inline]
			fn atan(self) -> $ty {
				self.atan()
			}
			#[inline]
			fn atan2(self, x: $ty) -> $ty {
				self.atan2(x)
			}
			#[inline]
			fn is_close(self, other: Self) -> bool {
				const ATOL: $ty = <$ty>::EPSILON; // Comparisons below 1.0
				let rtol = 1.0 / <$ty>::powi(10.0, <$ty>::DIGITS as i32); // Comparisons above 1.0
				(self - other).abs() <= <$ty>::max(ATOL, rtol * self.abs().max(other.abs()))
			}
		}
	}
}

impl_float_ops!(f32);
impl_float_ops!(f64);
