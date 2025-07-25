/*!
Angles.
*/
use super::*;

/// Angle in radians.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Angle<T> {
	pub radians: T,
}

/// Angle constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Angle<T>(radians: T) -> Angle<T> {
	Angle { radians }
}

impl<T: Zero> Zero for Angle<T> {
	const ZERO: Angle<T> = Angle { radians: T::ZERO };
}

impl<T: Float> Angle<T> {
	/// Constructs a new angle from radians.
	#[inline]
	pub const fn new(radians: T) -> Angle<T> {
		Angle { radians }
	}

	/// Constructs a new angle from degrees.
	#[inline]
	pub fn deg(degrees: T) -> Angle<T> {
		let radians = degrees * (T::TAU / T::THREE_SIXTY);
		Angle { radians }
	}

	/// Constant 360° angle or 2π radians.
	pub const TURN: Angle<T> = Angle(T::TAU);
	/// Constant 180° angle or π radians.
	pub const HALF: Angle<T> = Angle(T::PI);
	/// Constant 90° angle or π/2 radians.
	pub const QUARTER: Angle<T> = Angle(T::FRAC_PI_2);

	/// Normalizes the angle to range -180°, 180° or -π rad, π rad.
	#[inline]
	pub fn norm(self) -> Self {
		let div = self.radians / T::TAU;
		Angle(self.radians - div.round() * T::TAU)
	}

	/// Normalizes the angle to range 0°, 360° or 0 rad, 2π rad.
	#[inline]
	pub fn norm_abs(self) -> Self {
		let div = self.radians / T::TAU;
		Angle(self.radians - div.floor() * T::TAU)
	}

	/// Sine.
	#[inline]
	pub fn sin(self) -> T {
		self.radians.sin()
	}
	/// Cosine.
	#[inline]
	pub fn cos(self) -> T {
		self.radians.cos()
	}
	/// Tangent.
	#[inline]
	pub fn tan(self) -> T {
		self.radians.tan()
	}
	/// Sine and cosine.
	#[inline]
	pub fn sin_cos(self) -> (T, T) {
		self.radians.sin_cos()
	}

	#[inline]
	pub fn asin(sin: T) -> Self {
		Angle(sin.asin())
	}
	#[inline]
	pub fn acos(cos: T) -> Self {
		Angle(cos.acos())
	}
	#[inline]
	pub fn atan(tan: T) -> Self {
		Angle(tan.atan())
	}
	#[inline]
	pub fn atan2(y: T, x: T) -> Self {
		Angle(y.atan2(x))
	}

	/// Converts to degrees.
	#[inline]
	pub fn to_deg(self) -> T {
		self.radians * (T::THREE_SIXTY / T::TAU)
	}
	/// Casts the angle to another type.
	#[inline]
	pub fn cast<U>(self) -> Angle<U> where T: CastTo<U> {
		Angle(self.radians.cast_to())
	}
}

impl<T: Extrema> Extrema for Angle<T> {
	#[inline]
	fn min(self, rhs: Angle<T>) -> Angle<T> {
		Angle(self.radians.min(rhs.radians))
	}
	#[inline]
	fn max(self, rhs: Angle<T>) -> Angle<T> {
		Angle(self.radians.max(rhs.radians))
	}
	#[inline]
	fn min_max(self, rhs: Angle<T>) -> (Angle<T>, Angle<T>) {
		let (min, max) = self.radians.min_max(rhs.radians);
		(Angle(min), Angle(max))
	}
}

impl<T: Extrema> Angle<T> {
	/// Returns the smaller of two angles.
	#[inline]
	pub fn min(self, rhs: Angle<T>) -> Angle<T> {
		Angle(self.radians.min(rhs.radians))
	}
	/// Returns the larger of two angles.
	#[inline]
	pub fn max(self, rhs: Angle<T>) -> Angle<T> {
		Angle(self.radians.max(rhs.radians))
	}
	/// Returns angles in order: (smaller, larger).
	#[inline]
	pub fn min_max(self, rhs: Angle<T>) -> (Angle<T>, Angle<T>) {
		let (min, max) = self.radians.min_max(rhs.radians);
		(Angle(min), Angle(max))
	}
	/// Clamps the angle to the [min, max] range.
	#[inline]
	pub fn clamp(self, min: Angle<T>, max: Angle<T>) -> Angle<T> {
		Angle(self.radians.clamp(min.radians, max.radians))
	}
}

specialized_type!(Angle, Anglef, f32, radians);
specialized_type!(Angle, Angled, f64, radians);

//----------------------------------------------------------------
// Conversions

impl<T> AsRef<T> for Angle<T> {
	#[inline]
	fn as_ref(&self) -> &T {
		&self.radians
	}
}
impl<T> AsMut<T> for Angle<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut T {
		&mut self.radians
	}
}

//----------------------------------------------------------------
// Operators

impl<T: ops::Add<Output = T>> ops::Add<Angle<T>> for Angle<T> {
	type Output = Angle<T>;
	#[inline]
	fn add(self, rhs: Angle<T>) -> Angle<T> {
		Angle(self.radians + rhs.radians)
	}
}
impl<T: ops::Sub<Output = T>> ops::Sub<Angle<T>> for Angle<T> {
	type Output = Angle<T>;
	#[inline]
	fn sub(self, rhs: Angle<T>) -> Angle<T> {
		Angle(self.radians - rhs.radians)
	}
}
impl<T: ops::Neg<Output = T>> ops::Neg for Angle<T> {
	type Output = Angle<T>;
	#[inline]
	fn neg(self) -> Angle<T> {
		Angle(-self.radians)
	}
}

impl<T: ops::Mul<Output = T>> ops::Mul<T> for Angle<T> {
	type Output = Angle<T>;
	#[inline]
	fn mul(self, rhs: T) -> Angle<T> {
		Angle(self.radians * rhs)
	}
}
impl<T: ops::Div<Output = T>> ops::Div<T> for Angle<T> {
	type Output = Angle<T>;
	#[inline]
	fn div(self, rhs: T) -> Angle<T> {
		Angle(self.radians / rhs)
	}
}
impl<T: ops::Div<Output = T>> ops::Div<Angle<T>> for Angle<T> {
	type Output = T;
	#[inline]
	fn div(self, rhs: Angle<T>) -> T {
		self.radians / rhs.radians
	}
}

impl<T: ops::AddAssign> ops::AddAssign for Angle<T> {
	#[inline]
	fn add_assign(&mut self, rhs: Angle<T>) {
		self.radians += rhs.radians;
	}
}
impl<T: ops::SubAssign> ops::SubAssign for Angle<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: Angle<T>) {
		self.radians -= rhs.radians;
	}
}

//----------------------------------------------------------------
// Formatting

macro_rules! fmt {
	($fmt:path) => {
		impl<T: $fmt + Float> $fmt for Angle<T> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				<T as $fmt>::fmt(&self.to_deg(), f)?;
				f.write_str("°")
			}
		}
	};
}

fmt!(fmt::Display);
fmt!(fmt::Debug);
fmt!(fmt::UpperExp);
fmt!(fmt::LowerExp);

//----------------------------------------------------------------
// Parsing

impl<T: Float + FromStr> FromStr for Angle<T> {
	type Err = T::Err;
	fn from_str(s: &str) -> Result<Angle<T>, T::Err> {
		let (s, degrees) = if let Some(s) = s.strip_suffix("°").or_else(|| s.strip_suffix("deg")) {
			(s.trim_ascii_end(), true)
		}
		else if let Some(s) = s.strip_suffix("rad") {
			(s.trim_ascii_end(), false)
		}
		else {
			(s, false)
		};
		let value = T::from_str(s)?;
		Ok(if degrees { Angle::deg(value) } else { Angle(value) })
	}
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Angle<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.radians.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Angle<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Angle<T>, D::Error> {
		T::deserialize(deserializer).map(Angle)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[allow(non_snake_case)]
	fn Deg<T: Float>(degrees: T) -> Angle<T> {
		Angle::deg(degrees)
	}

	#[track_caller]
	fn assert_eq<T: Float>(a: Angle<T>, b: Angle<T>) {
		assert_eq!(format!("{:.4}", a), format!("{:.4}", b), "angles not equal: {} != {}", a, b);
	}

	#[test]
	fn normalize() {
		assert_eq(Deg(179.0), Deg(-181.0).norm());
		assert_eq(Deg(-179.0), Deg(181.0).norm());
		assert_eq(Deg(0.125), Deg(360.125).norm());
		assert_eq(Deg(180.0), Deg(-180.0).norm());
		assert_eq(Deg(-180.0), Deg(180.0).norm());
	}

	#[test]
	fn normalize_abs() {
		assert_eq(Deg(179.0), Deg(-181.0).norm_abs());
		assert_eq(Deg(181.0), Deg(181.0).norm_abs());
		assert_eq(Deg(1.0), Deg(361.0).norm_abs());
		assert_eq(Deg(180.0), Deg(-180.0).norm_abs());
		assert_eq(Deg(359.0), Deg(359.0).norm_abs());
	}

	#[test]
	fn formatting() {
		assert_eq!("12°", format!("{:.0}", Deg(12.1f32)));
		assert_eq!(" 12.0°", format!("{:>5.1}", Deg(12.0)));
	}

	#[test]
	fn parse() {
		assert_eq!(Angle::<f32>::TURN, "360°".parse().unwrap());
		assert_eq!(Angle(1.5f32), "1.5 rad".parse().unwrap());
		assert_eq!(Angle(2.5f64), "2.5".parse().unwrap());
		assert_eq!(Deg(180f32), "180°".parse().unwrap());
		assert_eq!(Deg(90f32), "90deg".parse().unwrap());
	}
}
