
use super::*;

/// Complex number.
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
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
	pub fn rotate(angle: impl Angle<T = T>) -> Complex<T> {
		let (re, im) = angle.sin_cos();
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
	pub fn arg(self) -> Rad<T> {
		Rad(self.im.atan2(self.re))
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
		if len < T::EPSILON {
			Complex::ZERO
		}
		else {
			self * (T::ONE / len)
		}
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
		if denom < T::EPSILON {
			return Complex::ZERO;
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
		if radius < T::EPSILON {
			return Complex::ZERO;
		}
		let theta = self.arg();
		Complex {
			re: radius.ln(),
			im: theta.value,
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

macro_rules! impl_fmt {
	($fmt:path) => {
		impl<T: $fmt> $fmt for Complex<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.re.fmt(f)?;
				f.write_str(" + ")?;
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

specialized_type!(Complex, Complexf, f32, re, im);
specialized_type!(Complex, Complexd, f64, re, im);

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Complex<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let slice = <Complex<T> as AsRef<[T; 2]>>::as_ref(self).as_slice();
		serializer.collect_seq(slice)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Complex<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let [re, im]: [T; 2] = serde::Deserialize::deserialize(deserializer)?;
		Ok(Complex { re, im })
	}
}
