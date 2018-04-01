
/// Numerical `as` cast.
///
/// Performs the same conversion as using the `as` keyword to convert between numeric types.
pub trait AsCast<T> {
	/// Performs the `as` conversion.
	fn as_cast(self) -> T;
}

macro_rules! impl_as_cast {
	($ty:ty) => {

impl AsCast<u8> for $ty { fn as_cast(self) -> u8 { self as u8 } }
impl AsCast<u16> for $ty { fn as_cast(self) -> u16 { self as u16 } }
impl AsCast<u32> for $ty { fn as_cast(self) -> u32 { self as u32 } }
impl AsCast<u64> for $ty { fn as_cast(self) -> u64 { self as u64 } }

impl AsCast<i8> for $ty { fn as_cast(self) -> i8 { self as i8 } }
impl AsCast<i16> for $ty { fn as_cast(self) -> i16 { self as i16 } }
impl AsCast<i32> for $ty { fn as_cast(self) -> i32 { self as i32 } }
impl AsCast<i64> for $ty { fn as_cast(self) -> i64 { self as i64 } }

impl AsCast<f32> for $ty { fn as_cast(self) -> f32 { self as f32 } }
impl AsCast<f64> for $ty { fn as_cast(self) -> f64 { self as f64 } }

	};
}

impl_as_cast!(u8);
impl_as_cast!(u16);
impl_as_cast!(u32);
impl_as_cast!(u64);
impl_as_cast!(i8);
impl_as_cast!(i16);
impl_as_cast!(i32);
impl_as_cast!(i64);
impl_as_cast!(f32);
impl_as_cast!(f64);
