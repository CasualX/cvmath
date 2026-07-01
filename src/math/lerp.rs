/// Types which can be linearly interpolated.
pub trait Lerp: Copy {
	type T;

	fn lerp(self, other: Self, t: Self::T) -> Self;
}

/// Linear interpolation between `a` and `b` by `t`.
///
/// <!--SCALAR_LERP--><svg width="420" height="80" viewBox="0 0 420 80" preserveAspectRatio="xMidYMid meet" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><line x1="40" y1="40" x2="380" y2="40" stroke="white" /><circle cx="40" cy="40" r="2" fill="white" /><circle cx="380" cy="40" r="2" fill="white" /><text x="36" y="60" fill="white">a</text><text x="376" y="60" fill="white">b</text><circle cx="125" cy="40" r="2" fill="lime" /><text x="113" y="25" fill="lime">t = 0.25</text><circle cx="210" cy="40" r="2" fill="deepskyblue" /><text x="198" y="25" fill="deepskyblue">t = 0.5</text></svg>
#[inline]
pub fn lerp<T: Lerp>(a: T, b: T, t: T::T) -> T {
	a.lerp(b, t)
}
