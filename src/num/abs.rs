
pub trait Abs {
	type Output;
	fn abs(self) -> Self::Output;
}

//----------------------------------------------------------------
// Implementation

macro_rules! impl_abs {
	($ty:ty) => {
		impl Abs for $ty {
			type Output = $ty;
			#[inline]
			fn abs(self) -> $ty {
				self.abs()
			}
		}
	}
}

impl_abs!(i8);
impl_abs!(i16);
impl_abs!(i32);
impl_abs!(i64);

impl_abs!(f32);
impl_abs!(f64);
