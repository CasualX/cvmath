
/// Types which can be linearly interpolated.
pub trait Lerp: Copy {
	type T;

	fn lerp(self, other: Self, t: Self::T) -> Self;
}

/// Linearly interpolates between `from` and `to` by `t`.
#[inline]
pub fn lerp<T: Lerp>(from: T, to: T, t: T::T) -> T {
	from.lerp(to, t)
}
