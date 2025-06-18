use std::cmp;

/// Calculate mins and maxs.
pub trait Extrema<Rhs = Self>: Sized {
	#[must_use]
	fn min(self, rhs: Rhs) -> Self;
	#[must_use]
	fn max(self, rhs: Rhs) -> Self;
	#[must_use]
	fn min_max(self, rhs: Rhs) -> (Self, Self);

	#[inline]
	#[must_use]
	fn clamp(self, min: Rhs, max: Rhs) -> Self {
		self.max(min).min(max)
	}
}

//----------------------------------------------------------------
// Implementation

macro_rules! impl_int {
	($ty:ty) => {
		impl Extrema for $ty {
			#[inline]
			fn min(self, rhs: $ty) -> $ty {
				cmp::min(self, rhs)
			}
			#[inline]
			fn max(self, rhs: $ty) -> $ty {
				cmp::max(self, rhs)
			}
			#[inline]
			fn min_max(self, rhs: $ty) -> ($ty, $ty) {
				(cmp::min(self, rhs), cmp::max(self, rhs))
			}
		}

		impl<'a> Extrema for &'a $ty {
			#[inline]
			fn min(self, rhs: &'a $ty) -> &'a $ty {
				cmp::min(self, rhs)
			}
			#[inline]
			fn max(self, rhs: &'a $ty) -> &'a $ty {
				cmp::max(self, rhs)
			}
			#[inline]
			fn min_max(self, rhs: &'a $ty) -> (&'a $ty, &'a $ty) {
				(cmp::min(self, rhs), cmp::max(self, rhs))
			}
		}
	};
}

macro_rules! impl_float {
	($ty:ty) => {
		impl Extrema for $ty {
			#[inline]
			fn min(self, rhs: $ty) -> $ty {
				<$ty>::min(self, rhs)
			}
			#[inline]
			fn max(self, rhs: $ty) -> $ty {
				<$ty>::max(self, rhs)
			}
			#[inline]
			fn min_max(self, rhs: $ty) -> ($ty, $ty) {
				if self < rhs { (self, rhs) } else { (rhs, self) }
			}
		}

		impl<'a> Extrema<&'a $ty> for &'a $ty {
			#[inline]
			fn min(self, rhs: &'a $ty) -> &'a $ty {
				if self < rhs { self } else { rhs }
			}
			#[inline]
			fn max(self, rhs: &'a $ty) -> &'a $ty {
				if self > rhs { self } else { rhs }
			}
			#[inline]
			fn min_max(self, rhs: &'a $ty) -> (&'a $ty, &'a $ty) {
				if self < rhs { (self, rhs) } else { (rhs, self) }
			}
		}
	}
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);

impl_float!(f32);
impl_float!(f64);
