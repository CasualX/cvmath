
use super::*;

/// Complex number.
#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Complex<T> {
	pub re: T,
	pub im: T,
}

/// Complex constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Complex<T>(re: T, im: T) -> Complex<T> {
	Complex { re, im }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Complex<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Complex<T> {
	/// Constructs a new complex number from components.
	#[inline]
	pub const fn new(re: T, im: T) -> Complex<T> {
		Complex { re, im }
	}
}
impl<T: Zero> Complex<T> {
	/// Zero complex number.
	pub const ZERO: Complex<T> = Complex {
		re: T::ZERO,
		im: T::ZERO,
	};
}
impl<T: Zero + One> Complex<T> {
	/// Unit complex number.
	pub const UNIT: Complex<T> = Complex {
		re: T::ONE,
		im: T::ZERO,
	};
}
impl<T: Float> Complex<T> {
	/// Rotating complex number.
	#[inline]
	pub fn rotation(angle: Angle<T>) -> Complex<T> {
		let (sin, cos) = angle.sin_cos();
		Complex { re: cos, im: sin }
	}

	/// Returns the shortest rotation that aligns vector `from` with vector `to`.
	///
	/// The resulting complex number `R` satisfies:
	///
	/// ```text
	/// R * from = to
	/// ```
	///
	/// Both vectors are expected to be normalized.
	/// The implementation avoids trigonometric functions.
	///
	/// The 2D rotation is uniquely defined even when the vectors are opposite, so this returns a 180° rotation in that case.
	///
	/// This is useful for constructing a 2D orientation that points one direction vector toward another.
	///
	/// ```
	/// let from = cvmath::Vec2(1.0, 1.0).norm();
	/// let to = cvmath::Vec2(-1.0, 1.0).norm();
	/// let rotation = cvmath::Complex::rotation_between(from, to);
	///
	/// let expected = to.cast::<f32>();
	/// let actual = (rotation * from).cast::<f32>();
	/// assert_eq!(expected, actual);
	/// ```
	#[inline]
	pub fn rotation_between(from: Vec2<T>, to: Vec2<T>) -> Complex<T> {
		let im = from.cross(to);
		let re = from.dot(to);
		Complex { re, im }
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Float> Complex<T> {
	/// Squares the complex number.
	#[inline]
	pub fn sqr(self) -> Complex<T> {
		self * self
	}
	/// Calculates the squared absolute value.
	#[inline]
	pub fn abs_sqr(self) -> T {
		self.re * self.re + self.im * self.im
	}
	/// Calculates the absolute value.
	#[inline]
	pub fn abs(self) -> T {
		(self.re * self.re + self.im * self.im).sqrt()
	}
	/// Calculates the argument (angle).
	#[inline]
	pub fn arg(self) -> Angle<T> {
		Angle(self.im.atan2(self.re))
	}
	/// Calculates the complex conjugate.
	#[inline]
	pub fn conj(self) -> Complex<T> {
		Complex { re: self.re, im: -self.im }
	}
	/// Normalizes the complex number.
	///
	/// After normalization, the complex number has an absolute value of `1.0` except zero remains zero.
	#[inline]
	pub fn norm(self) -> Complex<T> {
		let len = self.abs();
		if len == T::ZERO {
			return self;
		}
		self * (T::ONE / len)
	}
	/// Converts to polar coordinates.
	#[inline]
	pub fn polar(self) -> Polar<T> {
		Polar {
			radius: self.abs(),
			theta: self.arg(),
		}
	}
	/// Calculates the reciprocal.
	#[inline]
	pub fn recip(self) -> Complex<T> {
		let denom = self.re * self.re + self.im * self.im;
		if denom == T::ZERO {
			return self;
		}
		let denom_recip = T::ONE / denom;
		Complex {
			re: self.re * denom_recip,
			im: -self.im * denom_recip,
		}
	}
	/// Raises the complex number to a power.
	#[inline]
	pub fn powf(self, exp: T) -> Complex<T> {
		let polar = self.polar();
		let radius = polar.radius.powf(exp);
		let theta = polar.theta * exp;
		let (sin, cos) = theta.sin_cos();
		Complex {
			re: radius * cos,
			im: radius * sin,
		}
	}
	/// Raises the complex number to an integer power.
	#[inline]
	pub fn powi(self, mut n: i32) -> Complex<T> {
		if n == 0 {
			return Self::UNIT;
		}

		let mut base = self;
		let mut result = Self::UNIT;

		let negative = n < 0;
		if negative {
			n = -n;
		}

		while n > 0 {
			if n % 2 == 1 {
				result = result * base;
			}
			base = base * base;
			n /= 2;
		}

		if negative {
			result.recip()
		}
		else {
			result
		}
	}
	/// Calculates the natural logarithm.
	#[inline]
	pub fn ln(self) -> Complex<T> {
		let radius = self.abs();
		if radius == T::ZERO {
			return self;
		}
		let theta = self.arg();
		Complex {
			re: radius.ln(),
			im: theta.radians,
		}
	}
	/// Calculates the exponential.
	#[inline]
	pub fn exp(self) -> Complex<T> {
		let exp_re = self.re.exp();
		let (sin, cos) = self.im.sin_cos();
		Complex {
			re: exp_re * cos,
			im: exp_re * sin,
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> From<Vec2<T>> for Complex<T> {
	#[inline]
	fn from(vec: Vec2<T>) -> Complex<T> {
		Complex { re: vec.x, im: vec.y }
	}
}
impl<T> From<Complex<T>> for Vec2<T> {
	#[inline]
	fn from(complex: Complex<T>) -> Vec2<T> {
		Vec2 { x: complex.re, y: complex.im }
	}
}
impl<T: Float> From<Polar<T>> for Complex<T> {
	#[inline]
	fn from(polar: Polar<T>) -> Complex<T> {
		polar.complex()
	}
}
impl<T: Float> From<Complex<T>> for Polar<T> {
	#[inline]
	fn from(complex: Complex<T>) -> Polar<T> {
		complex.polar()
	}
}

impl<T: Copy + ops::Neg<Output = T>> Complex<T> {
	#[inline]
	pub fn mat2(self) -> Mat2<T> {
		Mat2 {
			a11: self.re,
			a12: -self.im,
			a21: self.im,
			a22: self.re,
		}
	}
	#[inline]
	pub fn vec2(self) -> Vec2<T> {
		Vec2 { x: self.re, y: self.im }
	}
}

//----------------------------------------------------------------
// As references

impl<T> AsRef<[T; 2]> for Complex<T> {
	#[inline]
	fn as_ref(&self) -> &[T; 2] {
		unsafe { mem::transmute(self) }
	}
}
impl<T> AsMut<[T; 2]> for Complex<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T; 2] {
		unsafe { mem::transmute(self) }
	}
}

//----------------------------------------------------------------
// Operators

impl<T: ops::Add<Output = T>> ops::Add for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn add(self, other: Complex<T>) -> Complex<T> {
		Complex {
			re: self.re + other.re,
			im: self.im + other.im,
		}
	}
}
impl<T: ops::AddAssign> ops::AddAssign for Complex<T> {
	#[inline]
	fn add_assign(&mut self, other: Complex<T>) {
		self.re += other.re;
		self.im += other.im;
	}
}

impl<T: ops::Sub<Output = T>> ops::Sub for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn sub(self, other: Complex<T>) -> Complex<T> {
		Complex {
			re: self.re - other.re,
			im: self.im - other.im,
		}
	}
}
impl<T: ops::SubAssign> ops::SubAssign for Complex<T> {
	#[inline]
	fn sub_assign(&mut self, other: Complex<T>) {
		self.re -= other.re;
		self.im -= other.im;
	}
}

impl<T: ops::Neg<Output = T>> ops::Neg for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn neg(self) -> Complex<T> {
		Complex {
			re: -self.re,
			im: -self.im,
		}
	}
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn mul(self, rhs: T) -> Complex<T> {
		Complex {
			re: self.re * rhs,
			im: self.im * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Complex<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.re *= rhs;
		self.im *= rhs;
	}
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn div(self, rhs: T) -> Complex<T> {
		Complex {
			re: self.re / rhs,
			im: self.im / rhs,
		}
	}
}
impl<T: Copy + ops::DivAssign> ops::DivAssign<T> for Complex<T> {
	#[inline]
	fn div_assign(&mut self, rhs: T) {
		self.re /= rhs;
		self.im /= rhs;
	}
}

impl<T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>> ops::Mul for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn mul(self, other: Complex<T>) -> Complex<T> {
		Complex {
			re: self.re * other.re - self.im * other.im,
			im: self.re * other.im + self.im * other.re,
		}
	}
}
impl<T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>> ops::MulAssign for Complex<T> {
	#[inline]
	fn mul_assign(&mut self, other: Complex<T>) {
		let re = self.re * other.re - self.im * other.im;
		self.im = self.re * other.im + self.im * other.re;
		self.re = re;
	}
}


impl<T: Copy + ops::Mul<Output = T> + ops::Div<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>> ops::Div for Complex<T> {
	type Output = Complex<T>;

	#[inline]
	fn div(self, other: Complex<T>) -> Complex<T> {
		let denom = other.re * other.re + other.im * other.im;
		Complex {
			re: (self.re * other.re + self.im * other.im) / denom,
			im: (self.im * other.re - self.re * other.im) / denom,
		}
	}
}
impl<T: Copy + ops::Mul<Output = T> + ops::Div<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>> ops::DivAssign for Complex<T> {
	#[inline]
	fn div_assign(&mut self, other: Complex<T>) {
		*self = *self / other;
	}
}

impl<T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>> ops::Mul<Vec2<T>> for Complex<T> {
	type Output = Vec2<T>;

	#[inline]
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: self.re * rhs.x - self.im * rhs.y,
			y: self.re * rhs.y + self.im * rhs.x,
		}
	}
}

//----------------------------------------------------------------
// Random

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Complex<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<T>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Complex<T> {
		let re = rand.sample(self);
		let im = rand.sample(self);
		Complex { re, im }
	}
}

//----------------------------------------------------------------
// Formatting

macro_rules! impl_fmt {
	($fmt:path) => {
		impl<T: $fmt> $fmt for Complex<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.re.fmt(f)?;
				f.write_str(if f.alternate() { " + " } else { "+" })?;
				self.im.fmt(f)?;
				f.write_str("i")
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

//----------------------------------------------------------------
// Parsing

impl<T: FromStr + Default> FromStr for Complex<T> {
	type Err = ParseComplexError<T::Err>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim_ascii();
		let (re, im) = if let Some(imag) = s.strip_suffix("i") {
			let imag = imag.trim_ascii_end();
			if let Some(index) = split_complex_components(imag) {
				let (lhs, rhs) = imag.split_at(index);
				(lhs.trim_ascii_end().parse::<T>()?, parse_signed_component(rhs)?)
			}
			else {
				(T::default(), parse_signed_component(imag)?)
			}
		}
		else {
			(s.parse::<T>()?, T::default())
		};
		Ok(Complex { re, im })
	}
}

#[inline]
fn split_complex_components(s: &str) -> Option<usize> {
	let mut prev = None;
	for (index, ch) in s.char_indices().skip(1) {
		if matches!(ch, '+' | '-') && !matches!(prev, Some('e' | 'E')) {
			return Some(index);
		}
		prev = Some(ch);
	}
	None
}

#[inline]
fn parse_signed_component<T: FromStr>(s: &str) -> Result<T, T::Err> {
	let s = s.trim_ascii_start();
	let s = s.strip_prefix('+').map(str::trim_ascii_start).unwrap_or(s);
	s.parse()
}

/// An error which can be returned when parsing a Complex<T>.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseComplexError<E> {
	InvalidFormat,
	ParseValue(E),
}

impl<E> From<E> for ParseComplexError<E> {
	#[inline]
	fn from(err: E) -> ParseComplexError<E> {
		ParseComplexError::ParseValue(err)
	}
}

impl<E: Error + 'static> fmt::Display for ParseComplexError<E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#[allow(deprecated)]
		self.description().fmt(f)
	}
}

impl<E: Error + 'static> Error for ParseComplexError<E> {
	fn description(&self) -> &str {
		#[allow(deprecated)]
		match *self {
			ParseComplexError::InvalidFormat => "invalid format",
			ParseComplexError::ParseValue(ref inner) => inner.description(),
		}
	}
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			ParseComplexError::InvalidFormat => None,
			ParseComplexError::ParseValue(ref inner) => Some(inner),
		}
	}
}

specialized_type!(Complex, Complexf, f32, re, im);
specialized_type!(Complex, Complexd, f64, re, im);

//----------------------------------------------------------------
// Serialization

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Complex<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeTupleStruct;
		let mut state = serializer.serialize_tuple_struct("Complex", 2)?;
		state.serialize_field(&self.re)?;
		state.serialize_field(&self.im)?;
		state.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Complex<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let (re, im) = {
			#[derive(serde::Deserialize)]
			struct Complex<T>(T, T);
			let Complex(re, im) = Complex::<T>::deserialize(deserializer)?;
			(re, im)
		};
		Ok(Complex { re, im })
	}
}

#[test]
fn test_fmt() {
	let mut rng = urandom::new();
	for _ in 0..100 {
		let complex = Complexf::new(rng.next_f32() * 2.0 - 1.0, rng.next_f32() * 2.0 - 1.0);
		let parsed: Complex<f32> = format!("{complex:E}").parse().unwrap();
		assert_eq!(complex, parsed);
	}
}

#[test]
fn test_parse_negative_imaginary() {
	// Negative imaginary
	assert_eq!("1-2i".parse::<Complexf>().unwrap(), Complexf::new(1.0, -2.0));
	assert_eq!("1 -2i".parse::<Complexf>().unwrap(), Complexf::new(1.0, -2.0));
	assert_eq!("1+-2i".parse::<Complexf>().unwrap(), Complexf::new(1.0, -2.0));
	assert!("1 - 2i".parse::<Complexf>().is_err());

	// Pure imaginary
	assert_eq!("2i".parse::<Complexf>().unwrap(), Complexf::new(0.0, 2.0));
	assert_eq!("+2i".parse::<Complexf>().unwrap(), Complexf::new(0.0, 2.0));
	assert_eq!("-2i".parse::<Complexf>().unwrap(), Complexf::new(0.0, -2.0));
	assert_eq!("2.5E-1i".parse::<Complexf>().unwrap(), Complexf::new(0.0, 0.25));

	// Positive imaginary
	assert_eq!("1+2i".parse::<Complexf>().unwrap(), Complexf::new(1.0, 2.0));
	assert_eq!("1 +2i".parse::<Complexf>().unwrap(), Complexf::new(1.0, 2.0));
	assert_eq!("1 + 2i".parse::<Complexf>().unwrap(), Complexf::new(1.0, 2.0));
	assert_eq!("1+2.5E-1i".parse::<Complexf>().unwrap(), Complexf::new(1.0, 0.25));
}

#[test]
fn test_rotate_zero_angle() {
	assert_eq!(Complexf::rotation(Anglef::ZERO), Complexf::UNIT);
}

#[test]
fn test_rotation_between() {
	let from = Vec2(1.0, 1.0).norm();
	let to = Vec2(-1.0, 1.0).norm();
	let matrix = Complexd::rotation_between(from, to);
	let actual = (matrix * from).cast::<f32>();
	let expected = to.cast::<f32>();
	assert_eq!(expected, actual);
}

#[test]
fn test_rotation_between_opposite() {
	let from = Vec2d::X;
	let to = -Vec2d::X;
	let matrix = Complexd::rotation_between(from, to);
	let actual = (matrix * from).cast::<f32>();
	let expected = to.cast::<f32>();
	assert_eq!(expected, actual);
}
