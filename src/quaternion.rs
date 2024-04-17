use super::*;

/// Quaternion structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Quaternion<T> {
	pub a: T,
	pub b: T,
	pub c: T,
	pub d: T,
}

/// Quaternion constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Quaternion<T>(a: T, b: T, c: T, d: T) -> Quaternion<T> {
	Quaternion { a, b, c, d }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Quaternion<T> {}

impl<T> Quaternion<T> {
	/// Constructs a new quaternion from components.
	#[inline]
	pub const fn new(a: T, b: T, c: T, d: T) -> Quaternion<T> {
		Quaternion { a, b, c, d }
	}
}
impl<T: Zero> Quaternion<T> {
	pub const ZERO: Quaternion<T> = Quaternion { a: T::ZERO, b: T::ZERO, c: T::ZERO, d: T::ZERO };
}
impl<T: Zero + One> Quaternion<T> {
	pub const IDENTITY: Quaternion<T> = Quaternion { a: T::ONE, b: T::ZERO, c: T::ZERO, d: T::ZERO };
}

impl<T: Float> From<Quaternion<T>> for Mat4<T> {
	#[inline]
	fn from(q: Quaternion<T>) -> Mat4<T> {
		Mat4 {
			a11:  q.a, a12:  q.b, a13:  q.c, a14:  q.d,
			a21: -q.b, a22:  q.a, a23: -q.d, a24:  q.c,
			a31: -q.c, a32:  q.d, a33:  q.a, a34: -q.b,
			a41: -q.d, a42: -q.c, a43:  q.b, a44:  q.a,
		}
	}
}
impl<T> From<[T; 4]> for Quaternion<T> {
	#[inline]
	fn from(val: [T; 4]) -> Quaternion<T> {
		let [a, b, c, d] = val;
		Quaternion { a, b, c, d }
	}
}
impl<T> Into<[T; 4]> for Quaternion<T> {
	#[inline]
	fn into(self) -> [T; 4] {
		[self.a, self.b, self.c, self.d]
	}
}
impl<T> AsRef<[T; 4]> for Quaternion<T> {
	#[inline]
	fn as_ref(&self) -> &[T; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl<T> AsRef<[T]> for Quaternion<T> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		<Self as AsRef<[T; 4]>>::as_ref(self)
	}
}
impl<T> AsMut<[T; 4]> for Quaternion<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl<T> AsMut<[T]> for Quaternion<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T] {
		<Self as AsMut<[T; 4]>>::as_mut(self)
	}
}

impl<T: Float> Quaternion<T> {
	#[inline]
	pub fn len(self) -> T {
		(self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d).sqrt()
	}
	#[inline]
	pub fn normalize(self) -> Quaternion<T> {
		let len = self.len();
		if len == T::ZERO {
			Quaternion::ZERO
		} else {
			self * (T::ONE / len)
		}
	}
	#[inline]
	pub fn determinant(self) -> T {
		let x = self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d;
		x * x
	}
	#[inline]
	pub fn conjugate(self) -> Quaternion<T> {
		Quaternion { a: self.a, b: -self.b, c: -self.c, d: -self.d }
	}
	#[inline]
	pub fn inverse(self) -> Quaternion<T> {
		self.conjugate().normalize()
	}
}

// Addition
impl<T: ops::Add<Output = T>> ops::Add for Quaternion<T> {
	type Output = Quaternion<T>;
	#[inline]
	fn add(self, rhs: Quaternion<T>) -> Quaternion<T> {
		Quaternion {
			a: self.a + rhs.a,
			b: self.b + rhs.b,
			c: self.c + rhs.c,
			d: self.d + rhs.d,
		}
	}
}
impl<T: ops::AddAssign> ops::AddAssign for Quaternion<T> {
	#[inline]
	fn add_assign(&mut self, rhs: Quaternion<T>) {
		self.a += rhs.a;
		self.b += rhs.b;
		self.c += rhs.c;
		self.d += rhs.d;
	}
}

// Subtraction
impl<T: ops::Sub<Output = T>> ops::Sub for Quaternion<T> {
	type Output = Quaternion<T>;
	#[inline]
	fn sub(self, rhs: Quaternion<T>) -> Quaternion<T> {
		Quaternion {
			a: self.a - rhs.a,
			b: self.b - rhs.b,
			c: self.c - rhs.c,
			d: self.d - rhs.d,
		}
	}
}
impl<T: ops::SubAssign> ops::SubAssign for Quaternion<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: Quaternion<T>) {
		self.a -= rhs.a;
		self.b -= rhs.b;
		self.c -= rhs.c;
		self.d -= rhs.d;
	}
}

// Scalar multiplication
impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for Quaternion<T> {
	type Output = Quaternion<T>;
	#[inline]
	fn mul(self, rhs: T) -> Quaternion<T> {
		Quaternion {
			a: self.a * rhs,
			b: self.b * rhs,
			c: self.c * rhs,
			d: self.d * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Quaternion<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.a *= rhs;
		self.b *= rhs;
		self.c *= rhs;
		self.d *= rhs;
	}
}

// Scalar division
impl<T: Copy + ops::Div<Output = T>> ops::Div<T> for Quaternion<T> {
	type Output = Quaternion<T>;
	#[inline]
	fn div(self, rhs: T) -> Quaternion<T> {
		Quaternion {
			a: self.a / rhs,
			b: self.b / rhs,
			c: self.c / rhs,
			d: self.d / rhs,
		}
	}
}
impl<T: Copy + ops::DivAssign> ops::DivAssign<T> for Quaternion<T> {
	#[inline]
	fn div_assign(&mut self, rhs: T) {
		self.a /= rhs;
		self.b /= rhs;
		self.c /= rhs;
		self.d /= rhs;
	}
}

impl<T: ops::Neg<Output = T>> ops::Neg for Quaternion<T> {
	type Output = Quaternion<T>;
	#[inline]
	fn neg(self) -> Quaternion<T> {
		Quaternion { a: -self.a, b: -self.b, c: -self.c, d: -self.d }
	}
}

// Hamilton product
impl<T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T>> ops::Mul<Quaternion<T>> for Quaternion<T> {
	type Output = Quaternion<T>;
	#[inline]
	fn mul(self, rhs: Quaternion<T>) -> Quaternion<T> {
		Quaternion {
			a: self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
			b: self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
			c: self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
			d: self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T>> ops::MulAssign for Quaternion<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Quaternion<T>) {
		*self = *self * rhs;
	}
}

impl<T: Float> ops::Mul<Vec3<T>> for Quaternion<T> {
	type Output = Vec3<T>;
	#[inline]
	fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
		// let q = Quaternion { a: T::ZERO, b: rhs.x, c: rhs.y, d: rhs.z };
		// let r = self * q * self.inverse();
		// Vec3::new(r.b, r.c, r.d)

		let s = self.a;
		let u = Vec3::new(self.b, self.c, self.d);
		let v = rhs;
		let two = T::ONE + T::ONE;
		u * (u.dot(v) * two) + v * (s * s - u.dot(u)) + u.cross(v) * (s + s)
	}
}

specialized_type!(Quaternion, Quaternionf, f32, a, b, c, d);
specialized_type!(Quaternion, Quaterniond, f64, a, b, c, d);

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Quaternion<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let slice = <Quaternion<T> as AsRef<[T; 4]>>::as_ref(self).as_slice();
		serializer.collect_seq(slice)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Quaternion<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let [a, b, c, d] = <[T; 4]>::deserialize(deserializer)?;
		Ok(Quaternion { a, b, c, d })
	}
}
