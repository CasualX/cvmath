/*!
Angles.
*/

use ::std::{fmt, ops};

use ::num::{Cast, Float};

/// Angle units.
pub trait Angle where Self:
	Copy + Default + PartialEq + PartialOrd +
	fmt::Debug + fmt::Display +
	From<Deg<<Self as Angle>::T>> + From<Rad<<Self as Angle>::T>> +
	Into<Deg<<Self as Angle>::T>> + Into<Rad<<Self as Angle>::T>> +
	ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Neg<Output = Self> +
	ops::Mul<<Self as Angle>::T, Output = Self> + ops::Div<<Self as Angle>::T, Output = Self> +
{
	/// The underlying float type.
	type T: Float;
	/// Returns a full turn of `360°` or `2π rad`.
	fn turn() -> Self;
	/// Returns a half turn of `180°` or `π rad`.
	fn half() -> Self { Self::turn() / Self::T::literal(2.0) }
	/// Returns a third turn of `120°` or `2π/3 rad`.
	fn third() -> Self { Self::turn() / Self::T::literal(3.0) }
	/// Returns a quarter turn of `90°` or `π/2 rad`.
	fn quarter() -> Self { Self::turn() / Self::T::literal(4.0) }
	/// Returns a sixth turn of `60°` or `π/3 rad`.
	fn sixth() -> Self { Self::turn() / Self::T::literal(6.0) }
	/// Returns a turn of `0°` or `0π rad`.
	fn zero() -> Self { Self::default() }
	/// Normalizes the angle to range `[-180°, 180°]` or `[-π rad, π rad]`.
	fn norm(self) -> Self;
	/// Sine.
	fn sin(self) -> Self::T;
	/// Cosine.
	fn cos(self) -> Self::T;
	/// Tangent.
	fn tan(self) -> Self::T;
	/// Calculates the sine and cosine efficiently.
	fn sin_cos(self) -> (Self::T, Self::T);
	fn asin(Self::T) -> Self;
	fn acos(Self::T) -> Self;
	fn atan(Self::T) -> Self;
	fn atan2(y: Self::T, x: Self::T) -> Self;
	/// Converts from degrees.
	fn from_deg(deg: Deg<Self::T>) -> Self { deg.into() }
	/// Converts from radians.
	fn from_rad(rad: Rad<Self::T>) -> Self { rad.into() }
	/// Converts to degrees.
	fn to_deg(self) -> Deg<Self::T> { self.into() }
	/// Converts to radians.
	fn to_rad(self) -> Rad<Self::T> { self.into() }
}

/// Angle in degrees.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Deg<T>(pub T);

/// Angle in radians.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Rad<T>(pub T);

macro_rules! turn {
	(Deg<$T:ident>) => ($T::literal(360.0));
	(Rad<$T:ident>) => ($T::literal(6.283185307179586476925286766559));
}
macro_rules! cvt {
	(Deg<$T:ident> to Deg $e:expr) => ($e);
	(Deg<$T:ident> to Rad $e:expr) => ($e * (turn!(Rad<$T>) / turn!(Deg<$T>)));
	(Rad<$T:ident> to Deg $e:expr) => ($e * (turn!(Deg<$T>) / turn!(Rad<$T>)));
	(Rad<$T:ident> to Rad $e:expr) => ($e);
}

#[cfg(feature = "format-rad-pi")]
macro_rules! unit {
	(Deg $e:expr) => { ("°", $e) };
	(Rad $e:expr) => { ("π rad", $e / T::half()) };
}
#[cfg(feature = "format-rad-tau")]
macro_rules! unit {
	(Deg $e:expr) => { ("°", $e) };
	(Rad $e:expr) => { ("τ rad", $e / T::turn()) };
}
#[cfg(all(not(feature = "format-rad-pi"), not(feature = "format-rad-tau")))]
macro_rules! unit {
	(Deg $e:expr) => { ("°", $e) };
	(Rad $e:expr) => { (" rad", $e) };
}

macro_rules! fmt {
	($ty:ident $fmt:path) => {
		impl<T: Copy + $fmt> $fmt for $ty<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				let (s, e) = unit!($ty self.0);
				e.fmt(f)?;
				f.write_str(s)
			}
		}
	};
	($ty:ident) => {
		fmt!($ty ::std::fmt::Display);
		fmt!($ty ::std::fmt::UpperExp);
		fmt!($ty ::std::fmt::LowerExp);
	};
}

macro_rules! angle {
	($ty:ident) => {
		impl<T: Float> Angle for $ty<T> {
			type T = T;
			fn turn() -> $ty<T> { $ty(turn!($ty<T>)) }
			fn half() -> $ty<T> { $ty(turn!($ty<T>) / T::literal(2.0)) }
			fn third() -> $ty<T> { $ty(turn!($ty<T>) / T::literal(3.0)) }
			fn quarter() -> $ty<T> { $ty(turn!($ty<T>) / T::literal(4.0)) }
			fn sixth() -> $ty<T> { $ty(turn!($ty<T>) / T::literal(6.0)) }
			fn zero() -> $ty<T> { $ty(T::literal(0.0)) }
			fn norm(self) -> $ty<T> { $ty(self.0.remainder(turn!($ty<T>))) }
			fn sin(self) -> T { cvt!($ty<T> to Rad self.0).sin() }
			fn cos(self) -> T { cvt!($ty<T> to Rad self.0).cos() }
			fn tan(self) -> T { cvt!($ty<T> to Rad self.0).tan() }
			fn sin_cos(self) -> (T, T) { cvt!($ty<T> to Rad self.0).sin_cos() }
			fn asin(sin: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty sin.asin())) }
			fn acos(cos: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty cos.acos())) }
			fn atan(tan: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty tan.atan())) }
			fn atan2(y: T, x: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty y.atan2(x))) }
			fn from_deg(deg: Deg<T>) -> $ty<T> { $ty(cvt!(Deg<T> to $ty deg.0)) }
			fn from_rad(rad: Rad<T>) -> $ty<T> { $ty(cvt!(Rad<T> to $ty rad.0)) }
			fn to_deg(self) -> Deg<T> { Deg(cvt!($ty<T> to Deg self.0)) }
			fn to_rad(self) -> Rad<T> { Rad(cvt!($ty<T> to Rad self.0)) }
		}

		//----------------------------------------------------------------
		// Conversions

		impl<T> $ty<T> {
			pub fn cast<U>(self) -> $ty<U> where T: Cast<U> {
				$ty(self.0.cast())
			}
		}

		impl<T> AsRef<T> for $ty<T> {
			fn as_ref(&self) -> &T {
				&self.0
			}
		}
		impl<T> AsMut<T> for $ty<T> {
			fn as_mut(&mut self) -> &mut T {
				&mut self.0
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T: ops::Add<Output = T>> ops::Add<$ty<T>> for $ty<T> {
			type Output = $ty<T>;
			fn add(self, rhs: $ty<T>) -> $ty<T> {
				$ty(self.0 + rhs.0)
			}
		}
		impl<T: ops::Sub<Output = T>> ops::Sub<$ty<T>> for $ty<T> {
			type Output = $ty<T>;
			fn sub(self, rhs: $ty<T>) -> $ty<T> {
				$ty(self.0 - rhs.0)
			}
		}
		impl<T: ops::Neg<Output = T>> ops::Neg for $ty<T> {
			type Output = $ty<T>;
			fn neg(self) -> $ty<T> {
				$ty(-self.0)
			}
		}

		impl<T: ops::Mul<Output = T>> ops::Mul<T> for $ty<T> {
			type Output = $ty<T>;
			fn mul(self, rhs: T) -> $ty<T> {
				$ty(self.0 * rhs)
			}
		}
		impl<T: ops::Div<Output = T>> ops::Div<T> for $ty<T> {
			type Output = $ty<T>;
			fn div(self, rhs: T) -> $ty<T> {
				$ty(self.0 / rhs)
			}
		}

		//----------------------------------------------------------------
		// Formatting

		fmt!($ty);

	};
}

angle!(Deg);
angle!(Rad);

impl<T: Float> From<Deg<T>> for Rad<T> {
	fn from(deg: Deg<T>) -> Rad<T> {
		deg.to_rad()
	}
}
impl<T: Float> From<Rad<T>> for Deg<T> {
	fn from(rad: Rad<T>) -> Deg<T> {
		rad.to_deg()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formatting() {
		assert_eq!("12°", format!("{:.0}", Deg(12.1f32)));
		assert_eq!(" 12.0°", format!("{:>5.1}", Deg(12.0)));

		assert_eq!(Deg(179.0), Deg(-181.0).norm());
		assert_eq!(Deg(0.125), Deg(360.125).norm());
		assert_eq!(Deg(180.0), Deg(-180.0).norm());
		assert_eq!(Deg(-180.0), Deg(180.0).norm());
	}

	#[test]
	fn from() {
		fn rad<A: Into<Rad<f64>>>(_: A) {}
		fn angle<A: Angle<T = f64>>(angle: A) {
			// Doesn't work because I can't `impl<A: Angle> From<A> for Deg/Rad<A::T>`...
			// rad(angle);
			rad(angle.to_rad());
		}
		angle(Deg(12.0));
	}
}
