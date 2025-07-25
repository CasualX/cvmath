use super::*;

/// Quaternion number.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Quat<T> {
	pub a: T,
	pub b: T,
	pub c: T,
	pub d: T,
}

/// Quaternion constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Quat<T>(a: T, b: T, c: T, d: T) -> Quat<T> {
	Quat { a, b, c, d }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Quat<T> {}

impl<T> Quat<T> {
	/// Constructs a new quaternion from components.
	#[inline]
	pub const fn new(a: T, b: T, c: T, d: T) -> Quat<T> {
		Quat { a, b, c, d }
	}
	/// Returns the imaginary part of the quaternion.
	#[inline]
	pub fn imag(self) -> Vec3<T> {
		Vec3 { x: self.b, y: self.c, z: self.d }
	}
}
impl<T: Zero> Quat<T> {
	pub const ZERO: Quat<T> = Quat { a: T::ZERO, b: T::ZERO, c: T::ZERO, d: T::ZERO };
}
impl<T: Zero + One> Quat<T> {
	pub const UNIT: Quat<T> = Quat { a: T::ONE, b: T::ZERO, c: T::ZERO, d: T::ZERO };
}

impl<T: Float> From<Quat<T>> for Mat4<T> {
	#[inline]
	fn from(q: Quat<T>) -> Mat4<T> {
		Mat4 {
			a11:  q.a, a12:  q.b, a13:  q.c, a14:  q.d,
			a21: -q.b, a22:  q.a, a23: -q.d, a24:  q.c,
			a31: -q.c, a32:  q.d, a33:  q.a, a34: -q.b,
			a41: -q.d, a42: -q.c, a43:  q.b, a44:  q.a,
		}
	}
}
impl<T> From<[T; 4]> for Quat<T> {
	#[inline]
	fn from(val: [T; 4]) -> Quat<T> {
		let [a, b, c, d] = val;
		Quat { a, b, c, d }
	}
}
impl<T> Into<[T; 4]> for Quat<T> {
	#[inline]
	fn into(self) -> [T; 4] {
		[self.a, self.b, self.c, self.d]
	}
}
impl<T> AsRef<[T; 4]> for Quat<T> {
	#[inline]
	fn as_ref(&self) -> &[T; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl<T> AsRef<[T]> for Quat<T> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		<Self as AsRef<[T; 4]>>::as_ref(self)
	}
}
impl<T> AsMut<[T; 4]> for Quat<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl<T> AsMut<[T]> for Quat<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T] {
		<Self as AsMut<[T; 4]>>::as_mut(self)
	}
}

impl<T: Float> Quat<T> {
	/// Computes the dot product.
	#[inline]
	pub fn dot(self, other: Quat<T>) -> T {
		self.a * other.a + self.b * other.b + self.c * other.c + self.d * other.d
	}
	/// Computes the length of the quaternion.
	#[inline]
	pub fn len(self) -> T {
		self.dot(self).sqrt()
	}
	/// Computes the normalized quaternion.
	#[inline]
	pub fn norm(self) -> Quat<T> {
		let len = self.len();
		if len == T::ZERO {
			return self;
		}
		self * (T::ONE / len)
	}
	/// Computes the squared length (determinant) of the quaternion.
	#[inline]
	pub fn det(self) -> T {
		let x = self.len();
		x * x
	}
	/// Computes the conjugate of the quaternion.
	#[inline]
	pub fn conj(self) -> Quat<T> {
		Quat { a: self.a, b: -self.b, c: -self.c, d: -self.d }
	}
	/// Computes the inverse of the quaternion.
	#[inline]
	pub fn inverse(self) -> Quat<T> {
		self.conj().norm()
	}
}

impl<T: Float> Quat<T> {
	/// Constructs a quaternion from an axis and angle.
	///
	/// The axis should be a normalized vector, and the angle is in radians.
	#[inline]
	pub fn from_axis_angle(axis: Vec3<T>, angle: Angle<T>) -> Quat<T> {
		let half_angle = angle * (T::ONE / T::TWO);
		let (sin, cos) = half_angle.sin_cos();
		Quat { a: cos, b: axis.x * sin, c: axis.y * sin, d: axis.z * sin }
	}
	#[inline]
	pub fn to_axis_angle(self) -> (Vec3<T>, Angle<T>) {
		let q = self.norm();

		let half_angle = Angle::acos(q.a);
		let angle = half_angle + half_angle;

		let s = (T::ONE - q.a * q.a).sqrt();
		let axis = if s < T::EPSILON {
			Vec3::ZERO
		}
		else {
			q.imag() * (T::ONE / s)
		};

		(axis, angle)
	}
	/// Constructs a quaternion that represents the rotation from one vector to another.
	pub fn rotation(from: Vec3<T>, to: Vec3<T>) -> Quat<T> {
		let from = from.norm();
		let to = to.norm();
		let dot = from.dot(to);
		if dot >= T::ONE - T::EPSILON {
			Quat::UNIT
		}
		else if dot <= -(T::ONE - T::EPSILON) {
			let axis = from.any_perp();
			Quat::from_axis_angle(axis, Angle::HALF)
		}
		else {
			let axis = from.cross(to).norm();
			let angle = Angle::acos(dot);
			Quat::from_axis_angle(axis, angle)
		}
	}
	/// Raises this quaternion to the power of `v`, representing a fractional rotation.
	pub fn powf(self, exp: T) -> Quat<T> {
		let (axis, angle) = self.to_axis_angle();
		Quat::from_axis_angle(axis, angle * exp)
	}
	/// Performs spherical linear interpolation (slerp) between two quaternions.
	pub fn slerp(self, mut end: Quat<T>, t: T) -> Quat<T> {
		let mut cos_theta = self.dot(end);

		if cos_theta < T::ZERO {
			cos_theta = -cos_theta;
			end = -end;
		}

		// If the quaternions are very close, use linear interpolation
		if cos_theta >= T::ONE - T::EPSILON {
			return self + (end - self) * t;
		}

		// Nearly opposite quaternions: choose an orthogonal quaternion for interpolation
		// Find an orthogonal axis to self's vector part
		if cos_theta <= -(T::ONE - T::EPSILON) {
			let axis = self.imag().any_perp();
			let orthogonal = Quat::from_axis_angle(axis, Angle::HALF);
			return (self * (Quat::UNIT + (orthogonal - Quat::UNIT) * t)).norm();
		}

		let theta = cos_theta.acos();
		let sin_theta = theta.sin();

		let w1 = ((T::ONE - t) * theta).sin() / sin_theta;
		let w2 = (t * theta).sin() / sin_theta;

		(self * w1 + end * w2).norm()
	}
	/// Performs normalized linear interpolation (nlerp) between two quaternions.
	pub fn nlerp(self, mut end: Quat<T>, t: T) -> Quat<T> {
		let cos_theta = self.dot(end);
		if cos_theta < T::ZERO {
			end = -end;
		}

		(self + (end - self) * t).norm()
	}
}

// Addition
impl<T: ops::Add<Output = T>> ops::Add for Quat<T> {
	type Output = Quat<T>;
	#[inline]
	fn add(self, rhs: Quat<T>) -> Quat<T> {
		Quat {
			a: self.a + rhs.a,
			b: self.b + rhs.b,
			c: self.c + rhs.c,
			d: self.d + rhs.d,
		}
	}
}
impl<T: ops::AddAssign> ops::AddAssign for Quat<T> {
	#[inline]
	fn add_assign(&mut self, rhs: Quat<T>) {
		self.a += rhs.a;
		self.b += rhs.b;
		self.c += rhs.c;
		self.d += rhs.d;
	}
}

// Subtraction
impl<T: ops::Sub<Output = T>> ops::Sub for Quat<T> {
	type Output = Quat<T>;
	#[inline]
	fn sub(self, rhs: Quat<T>) -> Quat<T> {
		Quat {
			a: self.a - rhs.a,
			b: self.b - rhs.b,
			c: self.c - rhs.c,
			d: self.d - rhs.d,
		}
	}
}
impl<T: ops::SubAssign> ops::SubAssign for Quat<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: Quat<T>) {
		self.a -= rhs.a;
		self.b -= rhs.b;
		self.c -= rhs.c;
		self.d -= rhs.d;
	}
}

// Scalar multiplication
impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for Quat<T> {
	type Output = Quat<T>;
	#[inline]
	fn mul(self, rhs: T) -> Quat<T> {
		Quat {
			a: self.a * rhs,
			b: self.b * rhs,
			c: self.c * rhs,
			d: self.d * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Quat<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.a *= rhs;
		self.b *= rhs;
		self.c *= rhs;
		self.d *= rhs;
	}
}

// Scalar division
impl<T: Copy + ops::Div<Output = T>> ops::Div<T> for Quat<T> {
	type Output = Quat<T>;
	#[inline]
	fn div(self, rhs: T) -> Quat<T> {
		Quat {
			a: self.a / rhs,
			b: self.b / rhs,
			c: self.c / rhs,
			d: self.d / rhs,
		}
	}
}
impl<T: Copy + ops::DivAssign> ops::DivAssign<T> for Quat<T> {
	#[inline]
	fn div_assign(&mut self, rhs: T) {
		self.a /= rhs;
		self.b /= rhs;
		self.c /= rhs;
		self.d /= rhs;
	}
}

impl<T: ops::Neg<Output = T>> ops::Neg for Quat<T> {
	type Output = Quat<T>;
	#[inline]
	fn neg(self) -> Quat<T> {
		Quat { a: -self.a, b: -self.b, c: -self.c, d: -self.d }
	}
}

// Hamilton product
impl<T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T>> ops::Mul<Quat<T>> for Quat<T> {
	type Output = Quat<T>;
	#[inline]
	fn mul(self, rhs: Quat<T>) -> Quat<T> {
		Quat {
			a: self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
			b: self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
			c: self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
			d: self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T>> ops::MulAssign for Quat<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Quat<T>) {
		*self = *self * rhs;
	}
}

impl<T: Float> ops::Mul<Vec3<T>> for Quat<T> {
	type Output = Vec3<T>;
	#[inline]
	fn mul(self, v: Vec3<T>) -> Vec3<T> {
		let s = self.a;
		let u = self.imag();
		u * (u.dot(v) + u.dot(v)) + v * (s * s - u.dot(u)) + u.cross(v) * (s + s)
	}
}

specialized_type!(Quat, Quatf, f32, a, b, c, d);
specialized_type!(Quat, Quatd, f64, a, b, c, d);

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Quat<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let slice = <Quat<T> as AsRef<[T; 4]>>::as_ref(self).as_slice();
		serializer.collect_seq(slice)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Quat<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let [a, b, c, d] = <[T; 4]>::deserialize(deserializer)?;
		Ok(Quat { a, b, c, d })
	}
}
