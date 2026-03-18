use super::*;

/// Polar coordinates.
#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Polar<T> {
	pub radius: T,
	pub theta: Angle<T>,
}

/// Polar constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Polar<T>(radius: T, theta: Angle<T>) -> Polar<T> {
	Polar { radius, theta }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Polar<T> {}

impl<T> Polar<T> {
	/// Constructs a new polar coordinate from components.
	#[inline]
	pub const fn new(radius: T, theta: Angle<T>) -> Polar<T> {
		Polar { radius, theta }
	}
}
impl<T: Zero> Polar<T> {
	/// Zero polar coordinate.
	pub const ZERO: Polar<T> = Polar {
		radius: Zero::ZERO,
		theta: Zero::ZERO,
	};
}
impl<T: Zero + One> Polar<T> {
	/// Unit polar coordinate.
	pub const UNIT: Polar<T> = Polar {
		radius: One::ONE,
		theta: Zero::ZERO,
	};
}

impl<T: Float> Polar<T> {
	/// Rotating polar coordinate.
	#[inline]
	pub fn rotation(angle: Angle<T>) -> Polar<T> {
		Polar {
			radius: T::ONE,
			theta: angle,
		}
	}
}

impl<T: Float> Polar<T> {
	/// Squares the polar coordinate.
	#[inline]
	pub fn sqr(self) -> Polar<T> {
		self * self
	}
	/// Calculates the squared absolute value.
	#[inline]
	pub fn abs_sqr(self) -> T {
		self.radius * self.radius
	}
	/// Calculates the absolute value.
	#[inline]
	pub fn abs(self) -> T {
		self.radius.abs()
	}
	/// Calculates the argument (angle).
	#[inline]
	pub fn arg(self) -> Angle<T> {
		if self.radius == T::ZERO {
			return Angle::ZERO;
		}

		(if self.radius < T::ZERO { self.theta + Angle::PI } else { self.theta }).norm()
	}
	/// Calculates the complex conjugate.
	#[inline]
	pub fn conj(self) -> Polar<T> {
		Polar {
			radius: self.radius,
			theta: -self.theta,
		}
	}
	/// Normalizes the represented complex number.
	///
	/// After normalization, the represented complex number has an absolute value of `1.0` except zero remains zero.
	#[inline]
	pub fn norm(self) -> Polar<T> {
		let radius = if self.radius == T::ZERO { T::ZERO } else { self.radius.signum() };
		Polar { radius, ..self }
	}
	/// Converts the polar coordinate to a complex number.
	#[inline]
	pub fn complex(self) -> Complex<T> {
		let (sin, cos) = self.theta.sin_cos();
		Complex {
			re: self.radius * cos,
			im: self.radius * sin,
		}
	}
	/// Converts the polar coordinate to a 2D vector.
	#[inline]
	pub fn vec2(self) -> Vec2<T> {
		let (sin, cos) = self.theta.sin_cos();
		Vec2 {
			x: self.radius * cos,
			y: self.radius * sin,
		}
	}
	/// Calculates the reciprocal.
	///
	/// Returns the reciprocal with radius `NaN` if the radius is zero.
	#[inline]
	pub fn recip(self) -> Polar<T> {
		Polar {
			radius: T::ONE / self.radius,
			theta: -self.theta,
		}
	}
	/// Raises the represented complex number to a power using the principal branch.
	#[inline]
	pub fn powf(self, exp: T) -> Polar<T> {
		Polar {
			radius: self.abs().powf(exp),
			theta: (self.arg() * exp).norm(),
		}
	}
	/// Raises the represented complex number to an integer power.
	#[inline]
	pub fn powi(self, exp: i32) -> Polar<T> {
		if exp == 0 {
			return Self::UNIT;
		}

		let mut base = self;
		let mut exp = if exp < 0 {
			base = base.recip();
			exp.unsigned_abs()
		}
		else {
			exp as u32
		};

		let mut result = base;
		exp -= 1;
		while exp > 0 {
			if exp & 1 == 1 {
				result = result * base;
			}
			base = base * base;
			exp >>= 1;
		}
		result
	}
	/// Casts the polar coordinate to another type.
	#[inline]
	pub fn cast<U>(self) -> Polar<U> where T: CastTo<U> {
		Polar {
			radius: self.radius.cast_to(),
			theta: self.theta.cast(),
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T: Float> From<Vec2<T>> for Polar<T> {
	#[inline]
	fn from(vec: Vec2<T>) -> Polar<T> {
		Polar {
			radius: vec.len(),
			theta: vec.polar_angle(),
		}
	}
}
impl<T: Float> From<Polar<T>> for Vec2<T> {
	#[inline]
	fn from(polar: Polar<T>) -> Vec2<T> {
		polar.vec2()
	}
}

//----------------------------------------------------------------
// Operators

impl<T: ops::Neg<Output = T>> ops::Neg for Polar<T> {
	type Output = Polar<T>;

	#[inline]
	fn neg(self) -> Polar<T> {
		Polar {
			radius: -self.radius,
			theta: self.theta,
		}
	}
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Polar<T> {
	type Output = Polar<T>;

	#[inline]
	fn mul(self, rhs: T) -> Polar<T> {
		Polar {
			radius: self.radius * rhs,
			theta: self.theta,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Polar<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.radius *= rhs;
	}
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Polar<T> {
	type Output = Polar<T>;

	#[inline]
	fn div(self, rhs: T) -> Polar<T> {
		Polar {
			radius: self.radius / rhs,
			theta: self.theta,
		}
	}
}
impl<T: Copy + ops::DivAssign> ops::DivAssign<T> for Polar<T> {
	#[inline]
	fn div_assign(&mut self, rhs: T) {
		self.radius /= rhs;
	}
}

impl<T: ops::Mul<Output = T> + ops::Add<Output = T>> ops::Mul<Polar<T>> for Polar<T> {
	type Output = Polar<T>;

	#[inline]
	fn mul(self, rhs: Polar<T>) -> Polar<T> {
		Polar {
			radius: self.radius * rhs.radius,
			theta: self.theta + rhs.theta,
		}
	}
}
impl<T: ops::MulAssign + ops::AddAssign> ops::MulAssign<Polar<T>> for Polar<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Polar<T>) {
		self.radius *= rhs.radius;
		self.theta += rhs.theta;
	}
}

impl<T: ops::Div<Output = T> + ops::Sub<Output = T>> ops::Div<Polar<T>> for Polar<T> {
	type Output = Polar<T>;

	#[inline]
	fn div(self, rhs: Polar<T>) -> Polar<T> {
		Polar {
			radius: self.radius / rhs.radius,
			theta: self.theta - rhs.theta,
		}
	}
}
impl<T: ops::DivAssign + ops::SubAssign> ops::DivAssign<Polar<T>> for Polar<T> {
	#[inline]
	fn div_assign(&mut self, rhs: Polar<T>) {
		self.radius /= rhs.radius;
		self.theta -= rhs.theta;
	}
}

macro_rules! impl_fmt {
	($fmt:path) => {
		impl<T: $fmt> $fmt for Polar<T> where Angle<T>: $fmt {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.radius.fmt(f)?;
				let symbol = if f.alternate() { " angle " } else { " ∠ " };
				f.write_str(symbol)?;
				<Angle<T> as $fmt>::fmt(&self.theta, f)
			}
		}
	};
}

impl_fmt!(fmt::Display);
impl_fmt!(fmt::Debug);
impl_fmt!(fmt::Binary);
impl_fmt!(fmt::Octal);
impl_fmt!(fmt::LowerHex);
impl_fmt!(fmt::UpperHex);
impl_fmt!(fmt::LowerExp);
impl_fmt!(fmt::UpperExp);

specialized_type!(Polar, Polarf, f32, radius: f32, theta: Anglef);
specialized_type!(Polar, Polard, f64, radius: f64, theta: Angled);

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Polar<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<T> + urandom::Distribution<Angle<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Polar<T> {
		let radius = rand.sample(self);
		let theta = rand.sample(self);
		Polar { radius, theta }
	}
}

//----------------------------------------------------------------
// Serialization

#[cfg(feature = "serde")]
impl<T: serde::Serialize + 'static> serde::Serialize for Polar<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeTupleStruct;
		let mut state = serializer.serialize_tuple_struct("Polar", 2)?;
		state.serialize_field(&self.radius)?;
		state.serialize_field(&self.theta)?;
		state.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de> + 'static> serde::Deserialize<'de> for Polar<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let (radius, theta) = {
			#[derive(serde::Deserialize)]
			struct Polar<T: 'static>(T, Angle<T>);
			let Polar(radius, theta) = Polar::<T>::deserialize(deserializer)?;
			(radius, theta)
		};
		Ok(Polar { radius, theta })
	}
}

#[test]
fn test_rotation_zero_angle() {
	assert_eq!(Polarf::rotation(Anglef::ZERO), Polarf::UNIT);
}

#[test]
fn test_powi_negative() {
	let polar = Polarf(2.0, Anglef::deg(45.0));
	let actual = polar.powi(-2).complex();
	let expected = polar.complex().powi(-2);
	assert!(actual.re.is_close(expected.re));
	assert!(actual.im.is_close(expected.im));
}

#[test]
fn test_powi_i32_min() {
	let polar = Polarf::UNIT;
	assert_eq!(polar.powi(i32::MIN), Polarf::UNIT);
}

#[test]
fn test_powf_negative_radius_uses_principal_branch() {
	let polar = Polarf(-1.0, Anglef::ZERO);
	let actual = polar.powf(0.5).complex();
	let expected = polar.complex().powf(0.5);
	assert!(actual.re.is_close(expected.re));
	assert!(actual.im.is_close(expected.im));
}
