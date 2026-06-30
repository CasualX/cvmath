/// Types which can be linearly interpolated.
pub trait Lerp: Copy {
	type T;

	fn lerp(self, other: Self, t: Self::T) -> Self;
}

/// Linearly interpolates between `start` and `end` by `t`.
#[inline]
pub fn lerp<T: Lerp>(start: T, end: T, t: T::T) -> T {
	start.lerp(end, t)
}
