
use ::std::{fmt, ops};

use ::num::{Cast, Zero};

pub trait Angle<T>:
	Copy + Default + PartialEq + Zero +
	fmt::Debug + fmt::Display +
	ops::Add<Output = Self> + ops::Sub<Output = Self> +
	ops::Mul<T, Output = Self> + ops::Div<T, Output = Self> +
{
	fn turn() -> Self;
	fn half() -> Self;
	fn quarter() -> Self;
	fn sin(self) -> T;
	fn cos(self) -> T;
	fn tan(self) -> T;
	fn sin_cos(self) -> (T, T);
	fn asin(T) -> Self;
	fn acos(T) -> Self;
	fn atan(T) -> Self;
	fn atan2(T, T) -> Self;
	fn from_deg(T) -> Self;
	fn from_rad(T) -> Self;
	fn to_deg(self) -> Deg<T>;
	fn to_rad(self) -> Rad<T>;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Deg<T>(T);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
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

macro_rules! fmt_str {
	(Deg) => ("{}°");
	(Rad) => ("{} rad");
	(Deg .*) => ("{:.*}°");
	(Rad .*) => ("{:.*} rad");
}

macro_rules! angle {
	(for $ty:ident<$f:ty>) => {
		impl Angle<$f> for $ty<$f> {
			fn turn() -> $ty<$f> { $ty(turn!($ty)) }
			fn half() -> $ty<$f> { $ty(turn!($ty) / 2.0) }
			fn quarter() -> $ty<$f> { $ty(turn!($ty) / 4.0) }
			fn sin(self) -> $f { cvt!($ty to Rad self.0).sin() }
			fn cos(self) -> $f { cvt!($ty to Rad self.0).cos() }
			fn tan(self) -> $f { cvt!($ty to Rad self.0).tan() }
			fn sin_cos(self) -> ($f, $f) { cvt!($ty to Rad self.0).sin_cos() }
			fn asin(sin: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty sin.asin())) }
			fn acos(cos: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty cos.acos())) }
			fn atan(tan: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty tan.atan())) }
			fn atan2(y: $f, x: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty y.atan2(x))) }
			fn from_deg(deg: $f) -> $ty<$f> { $ty(cvt!(Deg to $ty deg)) }
			fn from_rad(rad: $f) -> $ty<$f> { $ty(cvt!(Rad to $ty rad)) }
			fn to_deg(self) -> Deg<$f> { Deg(cvt!($ty to Deg self.0)) }
			fn to_rad(self) -> Rad<$f> { Rad(cvt!($ty to Rad self.0)) }
		}
	};
	($ty:ident $fmt:expr) => {

		//----------------------------------------------------------------
		// Implement Angle

		angle!(for $ty<f32>);
		angle!(for $ty<f64>);

		impl<T: Zero> Zero for $ty<T> {
			fn zero() -> $ty<T> {
				$ty(T::zero())
			}
		}

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
		// impl<T> Into<T> for $ty<T> {
		// 	fn into(self) -> T {
		// 		self.0
		// 	}
		// }

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
				// This is unfortunate but it really wants the precision argument...
				match f.precision() {
					Some(p) => write!(f, fmt_str!($ty .*), p, self.0),
					None => write!(f, fmt_str!($ty), self.0),
				}
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
	}
}
