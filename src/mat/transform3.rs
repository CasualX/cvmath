/*!
3D affine transformation matrix.
*/

use super::*;

/// 3D affine transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
///
/// The third row is implied to be `[0, 0, 0, 1]` and is omitted.
///
/// Row-major storage with column-major semantics.
///
/// Stored in row-major order (fields appear in reading order),
/// but interpreted as column-major: each column is a transformed basis vector,
/// and matrices are applied to column vectors via `mat * vec`.
#[derive(Copy, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Transform3<T> {
	pub a11: T, pub a12: T, pub a13: T, pub a14: T,
	pub a21: T, pub a22: T, pub a23: T, pub a24: T,
	pub a31: T, pub a32: T, pub a33: T, pub a34: T,
}

/// Transform3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Transform3<T>(
	a11: T, a12: T, a13: T, a14: T,
	a21: T, a22: T, a23: T, a24: T,
	a31: T, a32: T, a33: T, a34: T,
) -> Transform3<T> {
	Transform3 { a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34 }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Transform3<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Transform3<T> {
	/// Constructs a new matrix from components.
	#[inline]
	pub const fn new(
		a11: T, a12: T, a13: T, a14: T,
		a21: T, a22: T, a23: T, a24: T,
		a31: T, a32: T, a33: T, a34: T,
	) -> Transform3<T> {
		Transform3 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
		}
	}
}

impl<T: Zero> Transform3<T> {
	/// Zero matrix.
	pub const ZERO: Transform3<T> = Transform3 {
		a11: T::ZERO, a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
		a21: T::ZERO, a22: T::ZERO, a23: T::ZERO, a24: T::ZERO,
		a31: T::ZERO, a32: T::ZERO, a33: T::ZERO, a34: T::ZERO,
	};
}

impl<T: Zero + One> Transform3<T> {
	/// Identity matrix.
	pub const IDENTITY: Transform3<T> = Transform3 {
		a11: T::ONE,  a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
		a21: T::ZERO, a22: T::ONE,  a23: T::ZERO, a24: T::ZERO,
		a31: T::ZERO, a32: T::ZERO, a33: T::ONE,  a34: T::ZERO,
	};
}

impl<T: Float> Transform3<T> {
	/// Translation matrix.
	#[inline]
	pub fn translate(trans: Vec3<T>) -> Transform3<T> {
		let Vec3 { x: a14, y: a24, z: a34 } = trans;
		Transform3 { a14, a24, a34, ..Transform3::IDENTITY }
	}

	/// Scaling matrix.
	///
	/// Scales around the origin.
	#[inline]
	pub fn scale(scale: Vec3<T>) -> Transform3<T> {
		let Vec3 { x: a11, y: a22, z: a33 } = scale;
		Transform3 { a11, a22, a33, ..Transform3::IDENTITY }
	}

	/// Rotation matrix around an axis.
	#[inline]
	pub fn rotate(axis: Vec3<T>, angle: Angle<T>) -> Transform3<T> {
		Mat3::rotate(axis, angle).transform3()
	}
}

impl<T: Float> Transform3<T> {
	/// Look-at matrix.
	#[inline]
	pub fn look_at(position: Vec3<T>, target: Vec3<T>, ref_up: Vec3<T>, hand: Hand) -> Transform3<T> {
		let forward = match hand {
			Hand::LH => target - position,
			Hand::RH => position - target,
		}.norm();

		let side = ref_up.cross(forward).norm();
		let up = forward.cross(side);

		let Vec3 { x: a11, y: a12, z: a13 } = side;
		let a14 = -side.dot(position);

		let Vec3 { x: a21, y: a22, z: a23 } = up;
		let a24 = -up.dot(position);

		let Vec3 { x: a31, y: a32, z: a33 } = forward;
		let a34 = -forward.dot(position);

		Transform3 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
		}
	}

	/// Orthographic projection matrix.
	///
	/// Clip and hand parameters only affect the Z coordinate.
	#[inline]
	pub fn ortho(bounds: Bounds3<T>, (hand, clip): (Hand, Clip)) -> Transform3<T> {
		let Bounds3 {
			mins: Vec3 { x: left, y: bottom, z: near },
			maxs: Vec3 { x: right, y: top, z: far },
		} = bounds;

		debug_assert!(T::ZERO < near && near < far);

		let a11 = T::TWO / (right - left);
		let a14 = -(right + left) / (right - left);
		let a22 = T::TWO / (top - bottom);
		let a24 = -(top + bottom) / (top - bottom);
		let a33 = match clip { Clip::ZO => T::ONE, Clip::NO => T::TWO } / (far - near);
		let a33 = match hand { Hand::LH => a33, Hand::RH => -a33 };
		let a34 = -match clip { Clip::ZO => near, Clip::NO => far + near } / (far - near);

		Transform3 { a11, a14, a22, a24, a33, a34, ..Self::IDENTITY }
	}
}

//----------------------------------------------------------------
// Conversions

impl<T: Zero + One> Transform3<T> {
	/// Converts to a 4x4 matrix.
	#[inline]
	pub fn mat4(self) -> Mat4<T> {
		self.into()
	}
}

impl<T> Transform3<T> {
	#[inline]
	fn as_array(&self) -> &[T; 12] {
		unsafe { mem::transmute(self)}
	}
	/// Imports the matrix from a row-major layout.
	#[inline]
	pub fn from_row_major(mat: [[T; 4]; 3]) -> Transform3<T> {
		let [[a11, a12, a13, a14], [a21, a22, a23, a24], [a31, a32, a33, a34]] = mat;
		Transform3 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
		}
	}
	/// Imports the matrix from a column-major layout.
	#[inline]
	pub fn from_column_major(mat: [[T; 3]; 4]) -> Transform3<T> {
		let [[a11, a21, a31], [a12, a22, a32], [a13, a23, a33], [a14, a24, a34]] = mat;
		Transform3 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
		}
	}
	/// Exports the matrix as a row-major array.
	#[inline]
	pub fn into_row_major(self) -> [[T; 4]; 3] {
		[
			[self.a11, self.a12, self.a13, self.a14],
			[self.a21, self.a22, self.a23, self.a24],
			[self.a31, self.a32, self.a33, self.a34],
		]
	}
	/// Exports the matrix as a column-major array.
	#[inline]
	pub fn into_column_major(self) -> [[T; 3]; 4] {
		[
			[self.a11, self.a21, self.a31],
			[self.a12, self.a22, self.a32],
			[self.a13, self.a23, self.a33],
			[self.a14, self.a24, self.a34],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Transform3<T> {
	/// Composes the matrix from basis vectors.
	#[inline]
	pub fn compose(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>, t: Vec3<T>) -> Transform3<T> {
		Transform3 {
			a11: x.x, a12: y.x, a13: z.x, a14: t.x,
			a21: x.y, a22: y.y, a23: z.y, a24: t.y,
			a31: x.z, a32: y.z, a33: z.z, a34: t.z,
		}
	}
	/// Gets the transformed X basis vector.
	#[inline]
	pub fn x(self) -> Vec3<T> {
		Vec3 { x: self.a11, y: self.a21, z: self.a31 }
	}
	/// Gets the transformed Y basis vector.
	#[inline]
	pub fn y(self) -> Vec3<T> {
		Vec3 { x: self.a12, y: self.a22, z: self.a32 }
	}
	/// Gets the transformed Z basis vector.
	#[inline]
	pub fn z(self) -> Vec3<T> {
		Vec3 { x: self.a13, y: self.a23, z: self.a33 }
	}
	/// Gets the translation vector.
	#[inline]
	pub fn t(self) -> Vec3<T> {
		Vec3 { x: self.a14, y: self.a24, z: self.a34 }
	}
	/// Gets the rotation matrix.
	#[inline]
	pub fn mat3(self) -> Mat3<T> {
		Mat3 {
			a11: self.a11, a12: self.a12, a13: self.a13,
			a21: self.a21, a22: self.a22, a23: self.a23,
			a31: self.a31, a32: self.a32, a33: self.a33,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Transform3<T> {
	/// Computes the determinant.
	#[inline]
	pub fn det(self) -> T {
		self.a11 * (self.a22 * self.a33 - self.a23 * self.a32) +
		self.a12 * (self.a23 * self.a31 - self.a21 * self.a33) +
		self.a13 * (self.a21 * self.a32 - self.a22 * self.a31)
	}
	/// Computes the trace.
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22 + self.a33 + T::ONE
	}
	/// Computes the squared Frobenius norm (sum of squares of all matrix elements).
	///
	/// This measure is useful for quickly checking matrix magnitude or comparing matrices without the cost of a square root operation.
	///
	/// To check if a matrix is effectively zero, test if `flat_norm_sqr()` is below a small epsilon threshold.
	#[inline]
	pub fn flat_norm_sqr(self) -> T {
		self.a11 * self.a11 + self.a12 * self.a12 + self.a13 * self.a13 + self.a14 * self.a14 +
		self.a21 * self.a21 + self.a22 * self.a22 + self.a23 * self.a23 + self.a24 * self.a24 +
		self.a31 * self.a31 + self.a32 * self.a32 + self.a33 * self.a33 + self.a34 * self.a34
	}
	#[inline]
	pub fn try_invert(self) -> Option<Transform3<T>> where T: Float {
		let det = self.det();
		if det == T::ZERO {
			return None;
		}

		let inv_det = T::ONE / det;
		Some(Transform3 {
			a11: (self.a22 * self.a33 - self.a23 * self.a32) * inv_det,
			a12: (self.a13 * self.a32 - self.a12 * self.a33) * inv_det,
			a13: (self.a12 * self.a23 - self.a13 * self.a22) * inv_det,
			a14: (self.a12 * (self.a24 * self.a33 - self.a23 * self.a34) +
				self.a13 * (self.a22 * self.a34 - self.a24 * self.a32) +
				self.a14 * (self.a23 * self.a32 - self.a22 * self.a33)) * inv_det,
			a21: (self.a23 * self.a31 - self.a21 * self.a33) * inv_det,
			a22: (self.a11 * self.a33 - self.a13 * self.a31) * inv_det,
			a23: (self.a13 * self.a21 - self.a11 * self.a23) * inv_det,
			a24: (self.a11 * (self.a23 * self.a34 - self.a24 * self.a33) +
				self.a13 * (self.a24 * self.a31 - self.a21 * self.a34) +
				self.a14 * (self.a21 * self.a33 - self.a23 * self.a31)) * inv_det,
			a31: (self.a21 * self.a32 - self.a22 * self.a31) * inv_det,
			a32: (self.a12 * self.a31 - self.a11 * self.a32) * inv_det,
			a33: (self.a11 * self.a22 - self.a12 * self.a21) * inv_det,
			a34: (self.a11 * (self.a24 * self.a32 - self.a22 * self.a34) +
				self.a12 * (self.a21 * self.a34 - self.a24 * self.a31) +
				self.a14 * (self.a22 * self.a31 - self.a21 * self.a32)) * inv_det,
		})
	}
	/// Computes the inverse matrix.
	///
	/// Returns the zero matrix if the determinant is near zero.
	#[inline]
	pub fn inverse(self) -> Transform3<T> where T: Float {
		self.try_invert().unwrap_or(Transform3::ZERO)
	}
	/// Linear interpolation between the matrix elements.
	#[inline]
	pub fn lerp(self, rhs: Transform3<T>, t: T) -> Transform3<T> where T: Float {
		Transform3 {
			a11: self.a11 + (rhs.a11 - self.a11) * t,
			a12: self.a12 + (rhs.a12 - self.a12) * t,
			a13: self.a13 + (rhs.a13 - self.a13) * t,
			a14: self.a14 + (rhs.a14 - self.a14) * t,

			a21: self.a21 + (rhs.a21 - self.a21) * t,
			a22: self.a22 + (rhs.a22 - self.a22) * t,
			a23: self.a23 + (rhs.a23 - self.a23) * t,
			a24: self.a24 + (rhs.a24 - self.a24) * t,

			a31: self.a31 + (rhs.a31 - self.a31) * t,
			a32: self.a32 + (rhs.a32 - self.a32) * t,
			a33: self.a33 + (rhs.a33 - self.a33) * t,
			a34: self.a34 + (rhs.a34 - self.a34) * t,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Transform3<T> {
	type Output = Vec3<T>;
	#[inline]
	fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
		Vec3 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13 + self.a14,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23 + self.a24,
			z: rhs.x * self.a31 + rhs.y * self.a32 + rhs.z * self.a33 + self.a34,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec4<T>> for Transform3<T> {
	type Output = Vec3<T>;
	#[inline]
	fn mul(self, rhs: Vec4<T>) -> Vec3<T> {
		Vec3 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13 + rhs.w * self.a14,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23 + rhs.w * self.a24,
			z: rhs.x * self.a31 + rhs.y * self.a32 + rhs.z * self.a33 + rhs.w * self.a34,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Transform3<T>> for Transform3<T> {
	type Output = Transform3<T>;
	#[inline]
	fn mul(self, rhs: Transform3<T>) -> Transform3<T> {
		Transform3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33,
			a14: self.a11 * rhs.a14 + self.a12 * rhs.a24 + self.a13 * rhs.a34 + self.a14,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33,
			a24: self.a21 * rhs.a14 + self.a22 * rhs.a24 + self.a23 * rhs.a34 + self.a24,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33,
			a34: self.a31 * rhs.a14 + self.a32 * rhs.a24 + self.a33 * rhs.a34 + self.a34,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat3<T>> for Transform3<T> {
	type Output = Transform3<T>;
	#[inline]
	fn mul(self, rhs: Mat3<T>) -> Transform3<T> {
		Transform3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33,
			a14: self.a14,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33,
			a24: self.a24,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33,
			a34: self.a34,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat3<T>> for Transform3<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Mat3<T>) {
		*self = *self * rhs;
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Transform3<T>> for Transform3<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Transform3<T>) {
		*self = *self * rhs;
	}
}

impl_mat_mul_scalar!(Transform3);
impl_mat_mul_vec!(Transform3, Vec3);
impl_mat_mul_vec!(Transform3, Vec4);
impl_mat_mul_mat!(Transform3);

//----------------------------------------------------------------
// Formatting

impl<T: fmt::Display> fmt::Display for Transform3<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Transform3(")?;
		print::print(&move |i| &self.as_array()[i], 0x23, f)?;
		f.write_str(")")
	}
}
impl<T: fmt::Debug> fmt::Debug for Transform3<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Transform3(")?;
		print::print(&move |i| print::Debug(&self.as_array()[i]), 0x23, f)?;
		f.write_str(")")
	}
}

//----------------------------------------------------------------
// Tests

#[test]
fn test_inverse() {
	let mut rng = urandom::seeded(42);

	for _ in 0..1000 {
		let x = Vec3(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);
		let y = Vec3(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);
		let z = Vec3(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);
		let t = Vec3(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);

		let mat = Transform3::compose(x, y, z, t);
		let inv = mat.inverse();
		let _identity = mat * inv;

		let p = Vec3(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(-1.0..1.0),
		);
		let projected = mat * p;
		let unprojected = inv * projected;
		let error = (unprojected - p).len();
		assert!(error < 1e-6, "Failed for mat: {mat:?}, p: {p:?}, error: {error}");
	}
}
