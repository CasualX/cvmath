/*!
Angles.
*/

use std::{fmt, ops};
use std::str::FromStr;

use num::{CastFrom, CastTo, Float};

/// Angle units.
pub trait Angle where Self:
	Copy + Default + PartialEq + PartialOrd +
	fmt::Debug + fmt::Display +
	From<Deg<<Self as Angle>::T>> + From<Rad<<Self as Angle>::T>> +
	Into<Deg<<Self as Angle>::T>> + Into<Rad<<Self as Angle>::T>> +
	/*addition*/ops::Add<Output = Self> + /*difference*/ops::Sub<Output = Self> + /*inverse*/ops::Neg<Output = Self> +
	/*scalar*/ops::Mul<<Self as Angle>::T, Output = Self> + /*scalar*/ops::Div<<Self as Angle>::T, Output = Self> +
	/*ratio*/ops::Div<Self, Output = <Self as Angle>::T>
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
	/// Returns a fifth turn of `72°` or `2π/5 rad`.
	fn fifth() -> Self { Self::turn() / Self::T::literal(5.0) }
	/// Returns a sixth turn of `60°` or `π/3 rad`.
	fn sixth() -> Self { Self::turn() / Self::T::literal(6.0) }
	/// Returns an eight turn of `45°` or `π/4 rad`.
	fn eight() -> Self { Self::turn() / Self::T::literal(8.0) }
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
	fn asin(sin: Self::T) -> Self;
	fn acos(cos: Self::T) -> Self;
	fn atan(tan: Self::T) -> Self;
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
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Deg<T>(pub T);

/// Angle in radians.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Rad<T>(pub T);

macro_rules! turn {
	(Deg) => (360.0);
	(Rad) => (6.283185307179586476925286766559);
}
macro_rules! cvt {
	(Deg<$T:ident> to Deg $e:expr) => ($e);
	(Deg<$T:ident> to Rad $e:expr) => ($e * $T::literal(turn!(Rad) / turn!(Deg)));
	(Rad<$T:ident> to Deg $e:expr) => ($e * $T::literal(turn!(Deg) / turn!(Rad)));
	(Rad<$T:ident> to Rad $e:expr) => ($e);
}

macro_rules! fmt {
	(Deg $fmt:path) => {
		impl<T: $fmt> $fmt for Deg<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				self.0.fmt(f)?;
				f.write_str("°")
			}
		}
	};
	(Rad $fmt:path) => {
		impl<T: $fmt> $fmt for Rad<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				self.0.fmt(f)?;
				f.write_str(" rad")
			}
		}
		#[cfg(feature = "format-rad-pi")]
		impl<T: Float + $fmt> $fmt for Rad<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				let e = *self / Self::half();
				<T as $fmt>::fmt(&e, f)?;
				f.write_str("π rad")
			}
		}
		#[cfg(feature = "format-rad-tau")]
		impl<T: Float + $fmt> $fmt for Rad<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				let e = *self / Self::turn();
				<T as $fmt>::fmt(&e, f)?;
				f.write_str("τ rad")
			}
		}
	};
	($ty:ident) => {
		fmt!($ty fmt::Display);
		fmt!($ty fmt::Debug);
		fmt!($ty fmt::UpperExp);
		fmt!($ty fmt::LowerExp);
	};
}

macro_rules! angle {
	($ty:ident) => {
		impl<T: Float> Angle for $ty<T> {
			type T = T;
			fn turn() -> $ty<T> { $ty(T::literal(turn!($ty))) }
			fn half() -> $ty<T> { $ty(T::literal(turn!($ty) / 2.0)) }
			fn third() -> $ty<T> { $ty(T::literal(turn!($ty) / 3.0)) }
			fn quarter() -> $ty<T> { $ty(T::literal(turn!($ty) / 4.0)) }
			fn fifth() -> $ty<T> { $ty(T::literal(turn!($ty) / 5.0)) }
			fn sixth() -> $ty<T> { $ty(T::literal(turn!($ty) / 6.0)) }
			fn eight() -> $ty<T> { $ty(T::literal(turn!($ty) / 8.0)) }
			fn zero() -> $ty<T> { $ty(T::literal(0.0)) }
			fn norm(self) -> $ty<T> { $ty(self.0.remainder(T::literal(turn!($ty)))) }
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
		// Inherent methods

		impl<T: Float> $ty<T> {
			/// Returns a full turn of `360°` or `2π rad`.
			pub fn turn() -> $ty<T> { Angle::turn() }
			/// Returns a half turn of `180°` or `π rad`.
			pub fn half() -> $ty<T> { Angle::half() }
			/// Returns a third turn of `120°` or `2π/3 rad`.
			pub fn third() -> $ty<T> { Angle::third() }
			/// Returns a quarter turn of `90°` or `π/2 rad`.
			pub fn quarter() -> $ty<T> { Angle::quarter() }
			/// Returns a fifth turn of `72°` or `2π/5 rad`.
			pub fn fifth() -> $ty<T> { Angle::fifth() }
			/// Returns a sixth turn of `60°` or `π/3 rad`.
			pub fn sixth() -> $ty<T> { Angle::sixth() }
			/// Returns an eight turn of `45°` or `π/4 rad`.
			pub fn eight() -> $ty<T> { Angle::eight() }
			/// Returns a turn of `0°` or `0π rad`.
			pub fn zero() -> $ty<T> { Angle::zero() }
			/// Normalizes the angle to range `[-180°, 180°]` or `[-π rad, π rad]`.
			pub fn norm(self) -> $ty<T> { Angle::norm(self) }
			/// Sine.
			pub fn sin(self) -> T { Angle::sin(self) }
			/// Cosine.
			pub fn cos(self) -> T { Angle::cos(self) }
			/// Tangent.
			pub fn tan(self) -> T { Angle::tan(self) }
			/// Calculates the sine and cosine efficiently.
			pub fn sin_cos(self) -> (T, T) { Angle::sin_cos(self) }
			pub fn asin(sin: T) -> $ty<T> { Angle::asin(sin) }
			pub fn acos(cos: T) -> $ty<T> { Angle::acos(cos) }
			pub fn atan(tan: T) -> $ty<T> { Angle::atan(tan) }
			pub fn atan2(y: T, x: T) -> $ty<T> { Angle::atan2(y, x) }
			/// Converts from degrees.
			pub fn from_deg(deg: Deg<T>) -> $ty<T> { Angle::from_deg(deg) }
			/// Converts from radians.
			pub fn from_rad(rad: Rad<T>) -> $ty<T> { Angle::from_rad(rad) }
			/// Converts to degrees.
			pub fn to_deg(self) -> Deg<T> { Angle::to_deg(self) }
			/// Converts to radians.
			pub fn to_rad(self) -> Rad<T> { Angle::to_rad(self) }
		}

		//----------------------------------------------------------------
		// Conversions

		impl<T> $ty<T> {
			pub fn cast<U>(self) -> $ty<U> where T: CastTo<U> {
				$ty(self.0.cast_to())
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
		impl<T: ops::Div<Output = T>> ops::Div<$ty<T>> for $ty<T> {
			type Output = T;
			fn div(self, rhs: $ty<T>) -> T {
				self.0 / rhs.0
			}
		}

		//----------------------------------------------------------------
		// Formatting

		fmt!($ty);

		//----------------------------------------------------------------
		// Parsing

		impl<T: Float + FromStr> FromStr for $ty<T> {
			type Err = T::Err;
			fn from_str(s: &str) -> Result<$ty<T>, T::Err> {
				if s.ends_with("°") {
					s[..s.len() - "°".len()].trim_right().parse().map(|a| Deg(a).into())
				}
				else if s.ends_with("rad") {
					s[..s.len() - "rad".len()].trim_right().parse().map(|a| Rad(a).into())
				}
				else {
					s.parse().map($ty)
				}
			}
		}
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

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn norm() {
		assert_eq!(Deg(179.0), Deg(-181.0).norm());
		assert_eq!(Deg(0.125), Deg(360.125).norm());
		assert_eq!(Deg(180.0), Deg(-180.0).norm());
		assert_eq!(Deg(-180.0), Deg(180.0).norm());
	}

	#[test]
	fn formatting() {
		assert_eq!("12°", format!("{:.0}", Deg(12.1f32)));
		assert_eq!(" 12.0°", format!("{:>5.1}", Deg(12.0)));
	}

	#[test]
	fn rad_fmt() {
		assert_eq!("2.00 rad", format!("{:.2}", Rad(2.0f32)));
	}

	#[test]
	fn parse() {
		assert_eq!(Rad::<f32>::turn(), "360°".parse().unwrap());
		assert_eq!(Rad(1.5f32), "1.5 rad".parse().unwrap());
		assert_eq!(Rad(2.5f64), "2.5".parse().unwrap());
		assert_eq!(Deg(180f32), "180°".parse().unwrap());
		assert_eq!(Deg(90f32), "90".parse().unwrap());
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
