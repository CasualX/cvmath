/*!
Angles.
*/

use ::std::{fmt, ops};

use ::num::{Cast, Float};

pub trait Angle where Self:
	Copy + Default + PartialEq + PartialOrd +
	fmt::Debug + fmt::Display +
	ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Neg<Output = Self> +
	ops::Mul<<Self as Angle>::T, Output = Self> + ops::Div<<Self as Angle>::T, Output = Self> +
{
	type T: Float;
	/// Returns a full turn of 360° or 2pi.
	fn turn() -> Self;
	/// Returns a half turn of 180° or pi.
	fn half() -> Self;
	/// Returns a quarter turn of 90° or pi/2.
	fn quarter() -> Self;
	/// Returns a turn of 0° or 0pi.
	fn zero() -> Self;
	/// Normalizes the angle to range [-180°, 180°] or [-pi, pi].
	fn norm(self) -> Self;
	fn sin(self) -> Self::T;
	fn cos(self) -> Self::T;
	fn tan(self) -> Self::T;
	fn sin_cos(self) -> (Self::T, Self::T);
	fn asin(Self::T) -> Self;
	fn acos(Self::T) -> Self;
	fn atan(Self::T) -> Self;
	fn atan2(y: Self::T, x: Self::T) -> Self;
	/// Converts from degrees.
	fn from_deg(Deg<Self::T>) -> Self;
	/// Converts from radians.
	fn from_rad(Rad<Self::T>) -> Self;
	/// Converts to degrees.
	fn to_deg(self) -> Deg<Self::T>;
	/// Converts to radians.
	fn to_rad(self) -> Rad<Self::T>;
}

/// Angle in degrees.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Deg<T>(T);

/// Angle in radians.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Rad<T>(T);

macro_rules! turn {
	(Deg) => (360.0);
	(Rad) => (6.283185307179586476925286766559);
}
macro_rules! cvt {
	(Deg to Deg $e:expr) => ($e);
	(Deg to Rad $e:expr) => ($e * (turn!(Rad) / turn!(Deg)));
	(Rad to Deg $e:expr) => ($e * (turn!(Deg) / turn!(Rad)));
	(Rad to Rad $e:expr) => ($e);
}

macro_rules! angle {
	(for $ty:ident<$f:ty>) => {
		impl Angle for $ty<$f> {
			type T = $f;
			fn turn() -> $ty<$f> { $ty(turn!($ty)) }
			fn half() -> $ty<$f> { $ty(turn!($ty) / 2.0) }
			fn quarter() -> $ty<$f> { $ty(turn!($ty) / 4.0) }
			fn zero() -> $ty<$f> { $ty(0.0) }
			fn norm(self) -> $ty<$f> { $ty(self.0.remainder(turn!($ty))) }
			fn sin(self) -> $f { cvt!($ty to Rad self.0).sin() }
			fn cos(self) -> $f { cvt!($ty to Rad self.0).cos() }
			fn tan(self) -> $f { cvt!($ty to Rad self.0).tan() }
			fn sin_cos(self) -> ($f, $f) { cvt!($ty to Rad self.0).sin_cos() }
			fn asin(sin: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty sin.asin())) }
			fn acos(cos: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty cos.acos())) }
			fn atan(tan: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty tan.atan())) }
			fn atan2(y: $f, x: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty y.atan2(x))) }
			fn from_deg(deg: Deg<$f>) -> $ty<$f> { $ty(cvt!(Deg to $ty deg.0)) }
			fn from_rad(rad: Rad<$f>) -> $ty<$f> { $ty(cvt!(Rad to $ty rad.0)) }
			fn to_deg(self) -> Deg<$f> { Deg(cvt!($ty to Deg self.0)) }
			fn to_rad(self) -> Rad<$f> { Rad(cvt!($ty to Rad self.0)) }
		}
	};
	($ty:ident $fmt:expr) => {

		//----------------------------------------------------------------
		// Implement Angle

		angle!(for $ty<f32>);
		angle!(for $ty<f64>);

		//----------------------------------------------------------------
		// Conversions

		impl<T> $ty<T> {
			pub fn cast<U>(self) -> $ty<U> where T: Cast<U> {
				$ty(self.0.cast())
			}
		}

		impl<T> From<T> for $ty<T> {
			fn from(val: T) -> $ty<T> {
				$ty(val)
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

		impl<T: fmt::Display> fmt::Display for $ty<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.0.fmt(f)?;
				f.write_str($fmt)
			}
		}

	};
}

angle!(Deg "°");
angle!(Rad " rad");

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formatting() {
		assert_eq!("12°", format!("{:.0}", Deg::from(12.1f32)));
		assert_eq!(" 12.0°", format!("{:>5.1}", Deg::from(12.0)));

		assert_eq!(Deg(179.0), Deg(-181.0).norm());
		assert_eq!(Deg(0.125), Deg(360.125).norm());
		assert_eq!(Deg(180.0), Deg(-180.0).norm());
		assert_eq!(Deg(-180.0), Deg(180.0).norm());
	}
}
