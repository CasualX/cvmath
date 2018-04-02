
/// Like the `From` trait implemented for numeric `as` conversions.
pub trait CastFrom<T>: Sized {
	/// Performs the `as` conversion.
	fn cast_from(_: T) -> Self;
	/// Legacy name.
	fn literal(lit: T) -> Self { Self::cast_from(lit) }
}
/// Like the `Into` trait implemented for numeric `as` conversions.
pub trait CastTo<T>: Sized {
	/// Performs the `as` conversion.
	fn cast_to(self) -> T;
}

//----------------------------------------------------------------
// Implementation

impl<T> CastTo<T> for T where T: CastFrom<Self> {
	fn cast_to(self) -> T {
		T::cast_from(self)
	}
}

macro_rules! impl_as_cast {
	($ty:ty) => {

impl CastFrom<u8> for $ty { fn cast_from(from: u8) -> $ty { from as $ty } }
impl CastFrom<u16> for $ty { fn cast_from(from: u16) -> $ty { from as $ty } }
impl CastFrom<u32> for $ty { fn cast_from(from: u32) -> $ty { from as $ty } }
impl CastFrom<u64> for $ty { fn cast_from(from: u64) -> $ty { from as $ty } }

impl CastFrom<i8> for $ty { fn cast_from(from: i8) -> $ty { from as $ty } }
impl CastFrom<i16> for $ty { fn cast_from(from: i16) -> $ty { from as $ty } }
impl CastFrom<i32> for $ty { fn cast_from(from: i32) -> $ty { from as $ty } }
impl CastFrom<i64> for $ty { fn cast_from(from: i64) -> $ty { from as $ty } }

impl CastFrom<f32> for $ty { fn cast_from(from: f32) -> $ty { from as $ty } }
impl CastFrom<f64> for $ty { fn cast_from(from: f64) -> $ty { from as $ty } }

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
