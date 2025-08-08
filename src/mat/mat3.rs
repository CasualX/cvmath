/*!
Mat3 transformation matrix.
*/

use super::*;

/// 3D transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
///
/// Row-major storage with column-major semantics.
///
/// Stored in row-major order (fields appear in reading order),
/// but interpreted as column-major: each column is a transformed basis vector,
/// and matrices are applied to column vectors via `mat * vec`.
#[derive(Copy, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Mat3<T> {
	pub a11: T, pub a12: T, pub a13: T,
	pub a21: T, pub a22: T, pub a23: T,
	pub a31: T, pub a32: T, pub a33: T,
}

/// Mat3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Mat3<T>(
	a11: T, a12: T, a13: T,
	a21: T, a22: T, a23: T,
	a31: T, a32: T, a33: T,
) -> Mat3<T> {
	Mat3 { a11, a12, a13, a21, a22, a23, a31, a32, a33 }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Mat3<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Mat3<T> {
	/// Constructs a new matrix from components.
	#[inline]
	pub const fn new(
		a11: T, a12: T, a13: T,
		a21: T, a22: T, a23: T,
		a31: T, a32: T, a33: T,
	) -> Mat3<T> {
		Mat3 {
			a11, a12, a13,
			a21, a22, a23,
			a31, a32, a33,
		}
	}
}

impl<T: Zero> Mat3<T> {
	/// Zero matrix.
	pub const ZERO: Mat3<T> = Mat3 {
		a11: T::ZERO, a12: T::ZERO, a13: T::ZERO,
		a21: T::ZERO, a22: T::ZERO, a23: T::ZERO,
		a31: T::ZERO, a32: T::ZERO, a33: T::ZERO,
	};
}

impl<T: Zero + One> Mat3<T> {
	/// Identity matrix.
	pub const IDENTITY: Mat3<T> = Mat3 {
		a11: T::ONE,  a12: T::ZERO, a13: T::ZERO,
		a21: T::ZERO, a22: T::ONE,  a23: T::ZERO,
		a31: T::ZERO, a32: T::ZERO, a33: T::ONE,
	};
}

impl<T: Float> Mat3<T> {
	/// Scaling matrix.
	#[inline]
	pub fn scale(scale: Vec3<T>) -> Mat3<T> {
		let Vec3 { x: a11, y: a22, z: a33 } = scale;
		Mat3 { a11, a22, a33, ..Mat3::IDENTITY }
	}

	/// Rotation matrix around an axis.
	#[inline]
	pub fn rotate(axis: Vec3<T>, angle: Angle<T>) -> Mat3<T> {
		let (s, c) = angle.sin_cos();
		let Vec3 { x, y, z } = axis;
		let t = T::ONE - c;
		Mat3 {
			a11: t * x * x + c,     a12: t * x * y - s * z, a13: t * x * z + s * y,
			a21: t * x * y + s * z, a22: t * y * y + c,     a23: t * y * z - s * x,
			a31: t * x * z - s * y, a32: t * y * z + s * x, a33: t * z * z + c,
		}
	}
}

impl<T: Zero + One> From<Transform2<T>> for Mat3<T> {
	#[inline]
	fn from(mat: Transform2<T>) -> Mat3<T> {
		Mat3 {
			a11: mat.a11, a12: mat.a12, a13: mat.a13,
			a21: mat.a21, a22: mat.a22, a23: mat.a23,
			..Mat3::IDENTITY
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Mat3<T> {
	/// Converts to a Mat2 matrix.
	#[inline]
	pub fn mat2(self) -> Mat2<T> {
		Mat2 {
			a11: self.a11, a12: self.a12,
			a21: self.a21, a22: self.a22,
		}
	}
	/// Converts to a Transform3 matrix.
	#[inline]
	pub fn transform3(self) -> Transform3<T> where T: Zero {
		Transform3 {
			a11: self.a11, a12: self.a12, a13: self.a13, a14: T::ZERO,
			a21: self.a21, a22: self.a22, a23: self.a23, a24: T::ZERO,
			a31: self.a31, a32: self.a32, a33: self.a33, a34: T::ZERO,
		}
	}
	/// Adds a translation to the matrix.
	#[inline]
	pub fn translate(self, trans: Vec3<T>) -> Transform3<T> {
		let Vec3 { x: a14, y: a24, z: a34 } = trans;
		Transform3 {
			a11: self.a11, a12: self.a12, a13: self.a13, a14,
			a21: self.a21, a22: self.a22, a23: self.a23, a24,
			a31: self.a31, a32: self.a32, a33: self.a33, a34,
		}
	}
}

impl<T> Mat3<T> {
	#[inline]
	fn as_array(&self) -> &[T; 9] {
		unsafe { mem::transmute(self) }
	}
	/// Imports the matrix from a row-major layout.
	#[inline]
	pub fn from_row_major(mat: [[T; 3]; 3]) -> Mat3<T> {
		let [[a11, a12, a13], [a21, a22, a23], [a31, a32, a33]] = mat;
		Mat3 {
			a11, a12, a13,
			a21, a22, a23,
			a31, a32, a33,
		}
	}
	/// Imports the matrix from a column-major layout.
	#[inline]
	pub fn from_column_major(mat: [[T; 3]; 3]) -> Mat3<T> {
		let [[a11, a21, a31], [a12, a22, a32], [a13, a23, a33]] = mat;
		Mat3 {
			a11, a12, a13,
			a21, a22, a23,
			a31, a32, a33,
		}
	}
	/// Exports the matrix as a row-major array.
	#[inline]
	pub fn into_row_major(self) -> [[T; 3]; 3] {
		[
			[self.a11, self.a12, self.a13],
			[self.a21, self.a22, self.a23],
			[self.a31, self.a32, self.a33],
		]
	}
	/// Exports the matrix as a column-major array.
	#[inline]
	pub fn into_column_major(self) -> [[T; 3]; 3] {
		[
			[self.a11, self.a21, self.a31],
			[self.a12, self.a22, self.a32],
			[self.a13, self.a23, self.a33],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Mat3<T> {
	/// Composes the matrix from basis vectors.
	#[inline]
	pub fn compose(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
		Mat3 {
			a11: x.x, a12: y.x, a13: z.x,
			a21: x.y, a22: y.y, a23: z.y,
			a31: x.z, a32: y.z, a33: z.z,
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
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Mat3<T> {
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
		self.a11 + self.a22 + self.a33
	}
	/// Computes the squared Frobenius norm (sum of squares of all matrix elements).
	///
	/// This measure is useful for quickly checking matrix magnitude or comparing matrices without the cost of a square root operation.
	///
	/// To check if a matrix is effectively zero, test if `flat_norm_sqr()` is below a small epsilon threshold.
	#[inline]
	pub fn flat_norm_sqr(self) -> T {
		self.a11 * self.a11 + self.a12 * self.a12 + self.a13 * self.a13 +
		self.a21 * self.a21 + self.a22 * self.a22 + self.a23 * self.a23 +
		self.a31 * self.a31 + self.a32 * self.a32 + self.a33 * self.a33
	}
	#[inline]
	pub fn try_invert(self) -> Option<Mat3<T>> where T: Float {
		let det = self.det();
		if det == T::ZERO {
			return None;
		}
		Some(self.adjugate() * (T::ONE / det))
	}
	/// Computes the inverse matrix.
	///
	/// Returns the zero matrix if the determinant is near zero.
	#[inline]
	pub fn inverse(self) -> Mat3<T> where T: Float {
		self.try_invert().unwrap_or(Mat3::ZERO)
	}
	/// Returns the transposed matrix.
	#[inline]
	pub fn transpose(self) -> Mat3<T> {
		Mat3 {
			a11: self.a11, a12: self.a21, a13: self.a31,
			a21: self.a12, a22: self.a22, a23: self.a32,
			a31: self.a13, a32: self.a23, a33: self.a33,
		}
	}
	/// Computes the adjugate matrix.
	#[inline]
	pub fn adjugate(self) -> Mat3<T> {
		Mat3 {
			a11: self.a22 * self.a33 - self.a23 * self.a32,
			a12: self.a13 * self.a32 - self.a12 * self.a33,
			a13: self.a12 * self.a23 - self.a13 * self.a22,

			a21: self.a23 * self.a31 - self.a21 * self.a33,
			a22: self.a11 * self.a33 - self.a13 * self.a31,
			a23: self.a13 * self.a21 - self.a11 * self.a23,

			a31: self.a21 * self.a32 - self.a22 * self.a31,
			a32: self.a12 * self.a31 - self.a11 * self.a32,
			a33: self.a11 * self.a22 - self.a12 * self.a21,
		}
	}
	/// Linear interpolation between the matrix elements.
	#[inline]
	pub fn lerp(self, rhs: Mat3<T>, t: T) -> Mat3<T> where T: Float {
		Mat3 {
			a11: self.a11 + (rhs.a11 - self.a11) * t,
			a12: self.a12 + (rhs.a12 - self.a12) * t,
			a13: self.a13 + (rhs.a13 - self.a13) * t,
			a21: self.a21 + (rhs.a21 - self.a21) * t,
			a22: self.a22 + (rhs.a22 - self.a22) * t,
			a23: self.a23 + (rhs.a23 - self.a23) * t,
			a31: self.a31 + (rhs.a31 - self.a31) * t,
			a32: self.a32 + (rhs.a32 - self.a32) * t,
			a33: self.a33 + (rhs.a33 - self.a33) * t,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for Mat3<T> {
	type Output = Mat3<T>;
	#[inline]
	fn mul(self, rhs: T) -> Mat3<T> {
		Mat3 {
			a11: self.a11 * rhs, a12: self.a12 * rhs, a13: self.a13 * rhs,
			a21: self.a21 * rhs, a22: self.a22 * rhs, a23: self.a23 * rhs,
			a31: self.a31 * rhs, a32: self.a32 * rhs, a33: self.a33 * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Mat3<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.a11 *= rhs; self.a12 *= rhs; self.a13 *= rhs;
		self.a21 *= rhs; self.a22 *= rhs; self.a23 *= rhs;
		self.a31 *= rhs; self.a32 *= rhs; self.a33 *= rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Mat3<T> {
	type Output = Vec3<T>;
	#[inline]
	fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
		Vec3 {
			x: self.a11 * rhs.x + self.a12 * rhs.y + self.a13 * rhs.z,
			y: self.a21 * rhs.x + self.a22 * rhs.y + self.a23 * rhs.z,
			z: self.a31 * rhs.x + self.a32 * rhs.y + self.a33 * rhs.z,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat3<T>> for Mat3<T> {
	type Output = Mat3<T>;
	#[inline]
	fn mul(self, rhs: Mat3<T>) -> Mat3<T> {
		Mat3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat3<T>> for Mat3<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Mat3<T>) {
		*self = *self * rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Transform2<T>> for Mat3<T> {
	type Output = Mat3<T>;
	#[inline]
	fn mul(self, rhs: Transform2<T>) -> Mat3<T> {
		Mat3 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Transform2<T>> for Mat3<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Transform2<T>) {
		*self = *self * rhs;
	}
}

impl_mat_mul_scalar!(Mat3);
impl_mat_mul_vec!(Mat3, Vec3);
impl_mat_mul_mat!(Mat3);

//----------------------------------------------------------------
// Formatting

impl<T: fmt::Display> fmt::Display for Mat3<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Mat3(")?;
		print::print(&move |i| &self.as_array()[i], 0x33, f)?;
		f.write_str(")")
	}
}
impl<T: fmt::Debug> fmt::Debug for Mat3<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Mat3(")?;
		print::print(&move |i| print::Debug(&self.as_array()[i]), 0x33, f)?;
		f.write_str(")")
	}
}

//----------------------------------------------------------------
// Tests

#[test]
fn test_inverse() {
	let mut rng = urandom::seeded(42);

	for _ in 0..1000 {
		let a11 = rng.range(-10.0..10.0);
		let a12 = rng.range(-10.0..10.0);
		let a13 = rng.range(-10.0..10.0);
		let a21 = rng.range(-10.0..10.0);
		let a22 = rng.range(-10.0..10.0);
		let a23 = rng.range(-10.0..10.0);
		let a31 = rng.range(-10.0..10.0);
		let a32 = rng.range(-10.0..10.0);
		let a33 = rng.range(-10.0..10.0);

		let mat = Mat3(a11, a12, a13, a21, a22, a23, a31, a32, a33);
		let inv = mat.inverse();
		let _identity = mat * inv;

		let p = Vec3(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);

		let projected = mat * p;
		let unprojected = inv * projected;

		let error = (unprojected - p).len();
		assert!(error < 1e-6, "Failed for mat: {mat:?}, p: {p:?}, error: {error}");
	}
}
