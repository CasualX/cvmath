
use ::std::{fmt, ops};

use ::num::{Cast};

pub trait Angle<T> {
	fn turn() -> Self;
	fn sin(self) -> T;
	fn cos(self) -> T;
	fn tan(self) -> T;
	fn sin_cos(self) -> (T, T);
	fn asin(T) -> Self;
	fn acos(T) -> Self;
	fn atan(T) -> Self;
	fn atan2(T, T) -> Self;
	fn to_degrees(self) -> Deg<T>;
	fn to_radians(self) -> Rad<T>;
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

macro_rules! to_rad {
	(Deg $e:expr) => ($e * (6.283185307179586476925286766559 / 360.0));
	(Rad $e:expr) => ($e);
}
macro_rules! to_deg {
	(Deg $e:expr) => ($e);
	(Rad $e:expr) => ($e * (360.0 / 6.283185307179586476925286766559));
}
macro_rules! from_rad {
	(Deg $e:expr) => ($e * (360.0 / 6.283185307179586476925286766559));
	(Rad $e:expr) => ($e);
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
			fn sin(self) -> $f { to_rad!($ty self.0).sin() }
			fn cos(self) -> $f { to_rad!($ty self.0).cos() }
			fn tan(self) -> $f { to_rad!($ty self.0).tan() }
			fn sin_cos(self) -> ($f, $f) { to_rad!($ty self.0).sin_cos() }
			fn asin(val: $f) -> $ty<$f> { $ty(from_rad!($ty val.asin())) }
			fn acos(val: $f) -> $ty<$f> { $ty(from_rad!($ty val.acos())) }
			fn atan(val: $f) -> $ty<$f> { $ty(from_rad!($ty val.atan())) }
			fn atan2(y: $f, x: $f) -> $ty<$f> { $ty(from_rad!($ty y.atan2(x))) }
			fn to_radians(self) -> Rad<$f> {
				Rad(to_rad!($ty self.0))
			}
			fn to_degrees(self) -> Deg<$f> {
				Deg(to_deg!($ty self.0))
			}
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
