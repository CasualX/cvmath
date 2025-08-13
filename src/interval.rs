
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Interval<T> {
	pub min: T,
	pub max: T,
}

#[allow(non_snake_case)]
#[inline]
pub const fn Interval<T>(min: T, max: T) -> Interval<T> {
	Interval { min, max }
}
