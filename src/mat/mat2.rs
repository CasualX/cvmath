/*!
Mat2 transformation matrix.
*/

use super::*;

/// 2D transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
///
/// Row-major storage with column-major semantics.
///
/// Stored in row-major order (fields appear in reading order),
/// but interpreted as column-major: each column is a transformed basis vector,
/// and matrices are applied to column vectors via `mat * vec`.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat2<T> {
	pub a11: T, pub a12: T,
	pub a21: T, pub a22: T,
}

/// Mat2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Mat2<T>(
	a11: T, a12: T,
	a21: T, a22: T,
) -> Mat2<T> {
	Mat2 { a11, a12, a21, a22 }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Mat2<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Mat2<T> {
	/// Constructs a new matrix from components.
	#[inline]
	pub const fn new(
		a11: T, a12: T,
		a21: T, a22: T,
	) -> Mat2<T> {
		Mat2 {
			a11, a12,
			a21, a22,
		}
	}
}

impl<T: Zero> Mat2<T> {
	/// Zero matrix.
	pub const ZERO: Mat2<T> = Mat2 {
		a11: T::ZERO, a12: T::ZERO,
		a21: T::ZERO, a22: T::ZERO,
	};
}

impl<T: Zero + One> Mat2<T> {
	/// Identity matrix.
	pub const IDENTITY: Mat2<T> = Mat2 {
		a11: T::ONE,  a12: T::ZERO,
		a21: T::ZERO, a22: T::ONE,
	};
}

impl<T: Float> Mat2<T> {
	/// Scaling matrix.
	///
	/// Scales around the origin.
	#[inline]
	pub fn scale(scale: impl Into<Vec2<T>>) -> Mat2<T> {
		let scale = scale.into();
		Mat2 {
			a11: scale.x, a12: T::ZERO,
			a21: T::ZERO, a22: scale.y,
		}
	}

	/// Rotation matrix.
	///
	/// Rotates around the origin.
	#[inline]
	pub fn rotate(angle: impl Angle<T = T>) -> Mat2<T> {
		let (cy, cx) = angle.sin_cos();
		Mat2 {
			a11: cx, a12: -cy,
			a21: cy, a22:  cx,
		}
	}

	/// Skewing matrix.
	#[inline]
	pub fn skew(skew: impl Into<Vec2<T>>) -> Mat2<T> {
		let skew = skew.into();
		Mat2 {
			a11: T::ONE, a12: skew.x,
			a21: skew.y, a22: T::ONE,
		}
	}

	/// Reflection matrix.
	///
	/// Reflects around the given axis.
	/// If axis is the zero vector, returns a point reflection around the origin.
	#[inline]
	pub fn reflect(axis: impl Into<Vec2<T>>) -> Mat2<T> {
		let axis = axis.into();
		let ls = axis.dot(axis);
		if ls > T::EPSILON {
			let Vec2 { x, y } = axis;
			let ls = T::ONE / ls;
			Mat2 {
				a11: ls * (x * x - y * y), a12: ls * (x * y + x * y),
				a21: ls * (x * y + x * y), a22: ls * (y * y - x * x),
			}
		}
		else {
			// Do something like point reflection instead of NaN
			Mat2::scale(-T::ONE)
		}
	}

	/// Projection matrix.
	///
	/// Projects onto the given axis.
	/// If axis is the zero vector, returns the zero matrix.
	#[inline]
	pub fn project(axis: impl Into<Vec2<T>>) -> Mat2<T> {
		let axis = axis.into();
		let ls = axis.dot(axis);
		if ls > T::EPSILON {
			let Vec2 { x, y } = axis;
			let ls = T::ONE / ls;
			Mat2 {
				a11: ls * x * x, a12: ls * x * y,
				a21: ls * x * y, a22: ls * y * y,
			}
		}
		else {
			// Do something like absorb all
			Mat2::ZERO
		}
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Mat2<T> {
	/// Converts to a Transform2 matrix.
	#[inline]
	pub fn transform2(self) -> Transform2<T> where T: Zero {
		Transform2 {
			a11: self.a11, a12: self.a12, a13: T::ZERO,
			a21: self.a21, a22: self.a22, a23: T::ZERO,
		}
	}
	/// Adds a translation to the matrix.
	#[inline]
	pub fn translate(self, trans: impl Into<Vec2<T>>) -> Transform2<T> {
		let Vec2 { x: a13, y: a23 } = trans.into();
		Transform2 {
			a11: self.a11, a12: self.a12, a13,
			a21: self.a21, a22: self.a22, a23,
		}
	}
}

impl<T> Mat2<T> {
	/// Imports the matrix from a row-major layout.
	#[inline]
	pub fn from_row_major(mat: [[T; 2]; 2]) -> Mat2<T> {
		let [[a11, a12], [a21, a22]] = mat;
		Mat2 {
			a11, a12,
			a21, a22,
		}
	}
	/// Imports the matrix from a column-major layout.
	#[inline]
	pub fn from_column_major(mat: [[T; 2]; 2]) -> Mat2<T> {
		let [[a11, a21], [a12, a22]] = mat;
		Mat2 {
			a11, a12,
			a21, a22,
		}
	}
	/// Exports the matrix as a row-major array.
	#[inline]
	pub fn into_row_major(self) -> [[T; 2]; 2] {
		[
			[self.a11, self.a12],
			[self.a21, self.a22],
		]
	}
	/// Exports the matrix as a column-major array.
	#[inline]
	pub fn into_column_major(self) -> [[T; 2]; 2] {
		[
			[self.a11, self.a21],
			[self.a12, self.a22],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Mat2<T> {
	/// Composes the matrix from basis vectors.
	#[inline]
	pub fn compose(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
		Mat2 {
			a11: x.x, a12: y.x,
			a21: x.y, a22: y.y,
		}
	}
	/// Gets the transformed X basis vector.
	#[inline]
	pub fn x(self) -> Vec2<T> {
		Vec2 { x: self.a11, y: self.a21 }
	}
	/// Gets the transformed Y basis vector.
	#[inline]
	pub fn y(self) -> Vec2<T> {
		Vec2 { x: self.a12, y: self.a22 }
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Mat2<T> {
	/// Computes the determinant.
	#[inline]
	pub fn determinant(self) -> T {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	/// Computes the trace.
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22
	}
	/// Computes the squared Frobenius norm (sum of squares of all matrix elements).
	///
	/// This measure is useful for quickly checking matrix magnitude or comparing matrices without the cost of a square root operation.
	///
	/// To check if a matrix is effectively zero, test if `flat_norm_sqr()` is below a small epsilon threshold.
	#[inline]
	pub fn flat_norm_sqr(self) -> T {
		self.a11 * self.a11 + self.a12 * self.a12 +
		self.a21 * self.a21 + self.a22 * self.a22
	}
	#[inline]
	pub fn try_invert(self) -> Option<Mat2<T>> where T: Float {
		let det = self.determinant();
		if det.abs() < T::EPSILON {
			return None;
		}
		Some(self.adjugate() * (T::ONE / det))
	}
	/// Computes the inverse matrix.
	///
	/// Returns the zero matrix if the determinant is near zero.
	#[inline]
	pub fn inverse(self) -> Mat2<T> where T: Float {
		self.try_invert().unwrap_or(Mat2::ZERO)
	}
	/// Returns the transposed matrix.
	#[inline]
	pub fn transpose(self) -> Mat2<T> {
		Mat2 {
			a11: self.a22, a12: self.a21,
			a21: self.a12, a22: self.a11,
		}
	}
	/// Computes the adjugate matrix.
	#[inline]
	pub fn adjugate(self) -> Mat2<T> {
		Mat2 {
			a11:  self.a22, a12: -self.a12,
			a21: -self.a21, a22:  self.a11,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for Mat2<T> {
	type Output = Mat2<T>;
	#[inline]
	fn mul(self, rhs: T) -> Mat2<T> {
		Mat2 {
			a11: self.a11 * rhs,
			a12: self.a12 * rhs,
			a21: self.a21 * rhs,
			a22: self.a22 * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Mat2<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.a11 *= rhs;
		self.a12 *= rhs;
		self.a21 *= rhs;
		self.a22 *= rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Mat2<T> {
	type Output = Vec2<T>;
	#[inline]
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12,
			y: rhs.x * self.a21 + rhs.y * self.a22,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat2<T>> for Mat2<T> {
	type Output = Mat2<T>;
	#[inline]
	fn mul(self, rhs: Mat2<T>) -> Mat2<T> {
		Mat2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Transform2<T>> for Mat2<T> {
	type Output = Transform2<T>;
	#[inline]
	fn mul(self, rhs: Transform2<T>) -> Transform2<T> {
		Transform2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat2<T>> for Mat2<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Mat2<T>) {
		*self = *self * rhs;
	}
}

impl_mat_mul_scalar!(Mat2);
impl_mat_mul_vec!(Mat2, Vec2);
impl_mat_mul_mat!(Mat2);

#[test]
fn test_inverse() {
	let mut rng = urandom::seeded(42);

	for _ in 0..1000 {
		let a11 = rng.range(-10.0..10.0);
		let a12 = rng.range(-10.0..10.0);
		let a21 = rng.range(-10.0..10.0);
		let a22 = rng.range(-10.0..10.0);

		let mat = Mat2(a11, a12, a21, a22);
		let inv = mat.inverse();
		let _identity = mat * inv;

		let p = Vec2(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);
		let projected = mat * p;
		let unprojected = inv * projected;
		let error = (unprojected - p).len();
		assert!(error < 1e-6, "Failed for mat: {mat:?}, p: {p:?}, error: {error}");
	}
}
