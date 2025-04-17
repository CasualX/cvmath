/*!
Angles.
*/
use super::*;

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
	#[must_use]
	fn turn() -> Self;
	/// Returns a half turn of `180°` or `π rad`.
	#[must_use]
	fn half() -> Self { Self::turn() / Self::T::cast_from(2.0) }
	/// Returns a third turn of `120°` or `2π/3 rad`.
	#[must_use]
	fn third() -> Self { Self::turn() / Self::T::cast_from(3.0) }
	/// Returns a quarter turn of `90°` or `π/2 rad`.
	#[must_use]
	fn quarter() -> Self { Self::turn() / Self::T::cast_from(4.0) }
	/// Returns a fifth turn of `72°` or `2π/5 rad`.
	#[must_use]
	fn fifth() -> Self { Self::turn() / Self::T::cast_from(5.0) }
	/// Returns a sixth turn of `60°` or `π/3 rad`.
	#[must_use]
	fn sixth() -> Self { Self::turn() / Self::T::cast_from(6.0) }
	/// Returns an eight turn of `45°` or `π/4 rad`.
	#[must_use]
	fn eight() -> Self { Self::turn() / Self::T::cast_from(8.0) }
	/// Returns a turn of `0°` or `0π rad`.
	#[must_use]
	fn zero() -> Self { Self::default() }
	/// Normalizes the angle to range `[-180°, 180°]` or `[-π rad, π rad]`.
	#[must_use]
	fn normalize(self) -> Self;
	/// Normalizes the angle to range `[0°, 360°]` or `[0 rad, 2π rad]`.
	#[must_use]
	fn normalize_abs(self) -> Self;
	/// Sine.
	#[must_use]
	fn sin(self) -> Self::T;
	/// Cosine.
	#[must_use]
	fn cos(self) -> Self::T;
	/// Tangent.
	#[must_use]
	fn tan(self) -> Self::T;
	/// Calculates the sine and cosine efficiently.
	#[must_use]
	fn sin_cos(self) -> (Self::T, Self::T);
	#[must_use]
	fn asin(sin: Self::T) -> Self;
	#[must_use]
	fn acos(cos: Self::T) -> Self;
	#[must_use]
	fn atan(tan: Self::T) -> Self;
	#[must_use]
	fn atan2(y: Self::T, x: Self::T) -> Self;
	/// Converts from degrees.
	#[must_use]
	fn from_deg(deg: Deg<Self::T>) -> Self { deg.into() }
	/// Converts from radians.
	#[must_use]
	fn from_rad(rad: Rad<Self::T>) -> Self { rad.into() }
	/// Converts to degrees.
	#[must_use]
	fn to_deg(self) -> Deg<Self::T> { self.into() }
	/// Converts to radians.
	#[must_use]
	fn to_rad(self) -> Rad<Self::T> { self.into() }
}

/// Angle (degrees).
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Deg<T> {
	pub value: T,
}

/// Angle (degrees) constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Deg<T>(value: T) -> Deg<T> {
	Deg { value }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Deg<T> {}

/// Angle (radians).
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Rad<T> {
	pub value: T,
}

/// Angle (radians) constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Rad<T>(value: T) -> Rad<T> {
	Rad { value }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Rad<T> {}

macro_rules! turn {
	(Deg) => (360.0);
	(Rad) => (6.283185307179586476925286766559);
}
macro_rules! cvt {
	(Deg<$T:ident> to Deg $e:expr) => ($e);
	(Deg<$T:ident> to Rad $e:expr) => ($e * $T::cast_from(turn!(Rad) / turn!(Deg)));
	(Rad<$T:ident> to Deg $e:expr) => ($e * $T::cast_from(turn!(Deg) / turn!(Rad)));
	(Rad<$T:ident> to Rad $e:expr) => ($e);
}

macro_rules! fmt {
	(Deg $fmt:path) => {
		impl<T: $fmt> $fmt for Deg<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				self.value.fmt(f)?;
				f.write_str("°")
			}
		}
	};
	(Rad $fmt:path) => {
		impl<T: $fmt> $fmt for Rad<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				self.value.fmt(f)?;
				f.write_str(" rad")
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
			#[inline]
			fn turn() -> $ty<T> { $ty(T::cast_from(turn!($ty))) }
			#[inline]
			fn half() -> $ty<T> { $ty(T::cast_from(turn!($ty) / 2.0)) }
			#[inline]
			fn third() -> $ty<T> { $ty(T::cast_from(turn!($ty) / 3.0)) }
			#[inline]
			fn quarter() -> $ty<T> { $ty(T::cast_from(turn!($ty) / 4.0)) }
			#[inline]
			fn fifth() -> $ty<T> { $ty(T::cast_from(turn!($ty) / 5.0)) }
			#[inline]
			fn sixth() -> $ty<T> { $ty(T::cast_from(turn!($ty) / 6.0)) }
			#[inline]
			fn eight() -> $ty<T> { $ty(T::cast_from(turn!($ty) / 8.0)) }
			#[inline]
			fn zero() -> $ty<T> { $ty(T::cast_from(0.0)) }
			#[inline]
			fn normalize(self) -> $ty<T> {
				let div = self.value / T::cast_from(turn!($ty));
				$ty(self.value - div.round() * T::cast_from(turn!($ty)))
			}
			#[inline]
			fn normalize_abs(self) -> $ty<T> {
				let div = self.value / T::cast_from(turn!($ty));
				$ty(self.value - div.floor() * T::cast_from(turn!($ty)))
			}
			#[inline]
			fn sin(self) -> T { cvt!($ty<T> to Rad self.value).sin() }
			#[inline]
			fn cos(self) -> T { cvt!($ty<T> to Rad self.value).cos() }
			#[inline]
			fn tan(self) -> T { cvt!($ty<T> to Rad self.value).tan() }
			#[inline]
			fn sin_cos(self) -> (T, T) { cvt!($ty<T> to Rad self.value).sin_cos() }
			#[inline]
			fn asin(sin: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty sin.asin())) }
			#[inline]
			fn acos(cos: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty cos.acos())) }
			#[inline]
			fn atan(tan: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty tan.atan())) }
			#[inline]
			fn atan2(y: T, x: T) -> $ty<T> { $ty(cvt!(Rad<T> to $ty y.atan2(x))) }
			#[inline]
			fn from_deg(deg: Deg<T>) -> $ty<T> { $ty(cvt!(Deg<T> to $ty deg.value)) }
			#[inline]
			fn from_rad(rad: Rad<T>) -> $ty<T> { $ty(cvt!(Rad<T> to $ty rad.value)) }
			#[inline]
			fn to_deg(self) -> Deg<T> { Deg(cvt!($ty<T> to Deg self.value)) }
			#[inline]
			fn to_rad(self) -> Rad<T> { Rad(cvt!($ty<T> to Rad self.value)) }
		}

		//----------------------------------------------------------------
		// Inherent methods

		impl<T: Float> $ty<T> {
			/// Returns a full turn of `360°` or `2π rad`.
			#[inline]
			pub fn turn() -> $ty<T> { Angle::turn() }
			/// Returns a half turn of `180°` or `π rad`.
			#[inline]
			pub fn half() -> $ty<T> { Angle::half() }
			/// Returns a third turn of `120°` or `2π/3 rad`.
			#[inline]
			pub fn third() -> $ty<T> { Angle::third() }
			/// Returns a quarter turn of `90°` or `π/2 rad`.
			#[inline]
			pub fn quarter() -> $ty<T> { Angle::quarter() }
			/// Returns a fifth turn of `72°` or `2π/5 rad`.
			#[inline]
			pub fn fifth() -> $ty<T> { Angle::fifth() }
			/// Returns a sixth turn of `60°` or `π/3 rad`.
			#[inline]
			pub fn sixth() -> $ty<T> { Angle::sixth() }
			/// Returns an eight turn of `45°` or `π/4 rad`.
			#[inline]
			pub fn eight() -> $ty<T> { Angle::eight() }
			/// Returns a turn of `0°` or `0π rad`.
			#[inline]
			pub fn zero() -> $ty<T> { Angle::zero() }
			/// Normalizes the angle to range `[-180°, 180°]` or `[-π rad, π rad]`.
			#[inline]
			pub fn normalize(self) -> $ty<T> { Angle::normalize(self) }
			/// Normalizes the angle to range `[0°, 360°]` or `[0 rad, 2π rad]`.
			#[inline]
			pub fn normalize_abs(self) -> $ty<T> { Angle::normalize_abs(self) }
			/// Sine.
			#[inline]
			pub fn sin(self) -> T { Angle::sin(self) }
			/// Cosine.
			#[inline]
			pub fn cos(self) -> T { Angle::cos(self) }
			/// Tangent.
			#[inline]
			pub fn tan(self) -> T { Angle::tan(self) }
			/// Calculates the sine and cosine efficiently.
			#[inline]
			pub fn sin_cos(self) -> (T, T) { Angle::sin_cos(self) }
			#[inline]
			pub fn asin(sin: T) -> $ty<T> { Angle::asin(sin) }
			#[inline]
			pub fn acos(cos: T) -> $ty<T> { Angle::acos(cos) }
			#[inline]
			pub fn atan(tan: T) -> $ty<T> { Angle::atan(tan) }
			#[inline]
			pub fn atan2(y: T, x: T) -> $ty<T> { Angle::atan2(y, x) }
			/// Converts from degrees.
			#[inline]
			pub fn from_deg(deg: Deg<T>) -> $ty<T> { Angle::from_deg(deg) }
			/// Converts from radians.
			#[inline]
			pub fn from_rad(rad: Rad<T>) -> $ty<T> { Angle::from_rad(rad) }
			/// Converts to degrees.
			#[inline]
			pub fn to_deg(self) -> Deg<T> { Angle::to_deg(self) }
			/// Converts to radians.
			#[inline]
			pub fn to_rad(self) -> Rad<T> { Angle::to_rad(self) }
		}

		//----------------------------------------------------------------
		// Conversions

		impl<T> $ty<T> {
			#[inline]
			pub fn cast<U>(self) -> $ty<U> where T: CastTo<U> {
				$ty(self.value.cast_to())
			}
		}

		impl<T> AsRef<T> for $ty<T> {
			#[inline]
			fn as_ref(&self) -> &T {
				&self.value
			}
		}
		impl<T> AsMut<T> for $ty<T> {
			#[inline]
			fn as_mut(&mut self) -> &mut T {
				&mut self.value
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T: ops::Add<Output = T>> ops::Add<$ty<T>> for $ty<T> {
			type Output = $ty<T>;
			#[inline]
			fn add(self, rhs: $ty<T>) -> $ty<T> {
				$ty(self.value + rhs.value)
			}
		}
		impl<T: ops::Sub<Output = T>> ops::Sub<$ty<T>> for $ty<T> {
			type Output = $ty<T>;
			#[inline]
			fn sub(self, rhs: $ty<T>) -> $ty<T> {
				$ty(self.value - rhs.value)
			}
		}
		impl<T: ops::Neg<Output = T>> ops::Neg for $ty<T> {
			type Output = $ty<T>;
			#[inline]
			fn neg(self) -> $ty<T> {
				$ty(-self.value)
			}
		}

		impl<T: ops::Mul<Output = T>> ops::Mul<T> for $ty<T> {
			type Output = $ty<T>;
			#[inline]
			fn mul(self, rhs: T) -> $ty<T> {
				$ty(self.value * rhs)
			}
		}
		impl<T: ops::Div<Output = T>> ops::Div<T> for $ty<T> {
			type Output = $ty<T>;
			#[inline]
			fn div(self, rhs: T) -> $ty<T> {
				$ty(self.value / rhs)
			}
		}
		impl<T: ops::Div<Output = T>> ops::Div<$ty<T>> for $ty<T> {
			type Output = T;
			#[inline]
			fn div(self, rhs: $ty<T>) -> T {
				self.value / rhs.value
			}
		}

		impl<T: ops::AddAssign> ops::AddAssign for $ty<T> {
			#[inline]
			fn add_assign(&mut self, rhs: $ty<T>) {
				self.value += rhs.value;
			}
		}
		impl<T: ops::SubAssign> ops::SubAssign for $ty<T> {
			#[inline]
			fn sub_assign(&mut self, rhs: $ty<T>) {
				self.value -= rhs.value;
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
					s[..s.len() - "°".len()].trim_end().parse().map(|a| Deg(a).into())
				}
				else if s.ends_with("rad") {
					s[..s.len() - "rad".len()].trim_end().parse().map(|a| Rad(a).into())
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
	#[inline]
	fn from(deg: Deg<T>) -> Rad<T> {
		deg.to_rad()
	}
}
impl<T: Float> From<Rad<T>> for Deg<T> {
	#[inline]
	fn from(rad: Rad<T>) -> Deg<T> {
		rad.to_deg()
	}
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Deg<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.value.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Deg<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Deg<T>, D::Error> {
		T::deserialize(deserializer).map(Deg)
	}
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Rad<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.value.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Rad<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Rad<T>, D::Error> {
		T::deserialize(deserializer).map(Rad)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn normalize() {
		assert_eq!(Deg(179.0), Deg(-181.0).normalize());
		assert_eq!(Deg(-179.0), Deg(181.0).normalize());
		assert_eq!(Deg(0.125), Deg(360.125).normalize());
		assert_eq!(Deg(180.0), Deg(-180.0).normalize());
		assert_eq!(Deg(-180.0), Deg(180.0).normalize());
	}

	#[test]
	fn normalize_abs() {
		assert_eq!(Deg(179.0), Deg(-181.0).normalize_abs());
		assert_eq!(Deg(181.0), Deg(181.0).normalize_abs());
		assert_eq!(Deg(1.0), Deg(361.0).normalize_abs());
		assert_eq!(Deg(180.0), Deg(-180.0).normalize_abs());
		assert_eq!(Deg(359.0), Deg(359.0).normalize_abs());
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
