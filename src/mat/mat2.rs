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
#[derive(Copy, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
	///
	/// ```
	/// let mat = cvmath::Mat2::scaling(cvmath::Vec2(2.0, 3.0));
	/// let value = mat * cvmath::Vec2(4.0, 5.0);
	/// let expected = cvmath::Vec2(8.0, 15.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn scaling(scale: Vec2<T>) -> Mat2<T> {
		Mat2 {
			a11: scale.x, a12: T::ZERO,
			a21: T::ZERO, a22: scale.y,
		}
	}

	/// Rotation matrix.
	///
	/// Rotates around the origin.
	///
	/// ```
	/// let mat = cvmath::Mat2::rotation(cvmath::Angle::deg(90.0));
	/// let value = (mat * cvmath::Vec2(1.0f64, 1.0)).cast::<f32>();
	/// let expected = cvmath::Vec2(-1.0f32, 1.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn rotation(angle: Angle<T>) -> Mat2<T> {
		let (cy, cx) = angle.sin_cos();
		Mat2 {
			a11: cx, a12: -cy,
			a21: cy, a22:  cx,
		}
	}

	/// Returns the shortest rotation that aligns vector `from` with vector `to`.
	///
	/// The resulting matrix `R` satisfies:
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
	/// This is useful for constructing an orientation matrix that points one direction vector toward another.
	///
	/// ```
	/// let from = cvmath::Vec2(1.0, 1.0).norm();
	/// let to = cvmath::Vec2(-1.0, 1.0).norm();
	/// let mat = cvmath::Mat2::rotation_between(from, to);
	///
	/// let expected = to.cast::<f32>();
	/// let value = (mat * from).cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn rotation_between(from: Vec2<T>, to: Vec2<T>) -> Mat2<T> {
		let z = from.cross(to);
		let c = from.dot(to);
		Mat2 {
			a11: c, a12: -z,
			a21: z, a22: c,
		}
	}

	/// Skewing matrix.
	#[inline]
	pub fn skewing(skew: Vec2<T>) -> Mat2<T> {
		Mat2 {
			a11: T::ONE, a12: skew.x,
			a21: skew.y, a22: T::ONE,
		}
	}

	/// Reflection matrix.
	///
	/// Reflects around the given axis.
	/// If axis is the zero vector, returns a point reflection around the origin.
	///
	/// ```
	/// let mat = cvmath::Mat2::reflection(cvmath::Vec2d::Y);
	/// let value = mat * cvmath::Vec2(2.0, 3.0);
	/// let expected = cvmath::Vec2(-2.0, 3.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn reflection(axis: Vec2<T>) -> Mat2<T> {
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
			Mat2::scaling(Vec2::dup(-T::ONE))
		}
	}

	/// Projection matrix.
	///
	/// Projects onto the given axis.
	/// If axis is the zero vector, returns the zero matrix.
	///
	/// ```
	/// let mat = cvmath::Mat2::projection(cvmath::Vec2d::X);
	/// let value = mat * cvmath::Vec2(2.0, 3.0);
	/// let expected = cvmath::Vec2(2.0, 0.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn projection(axis: Vec2<T>) -> Mat2<T> {
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
	///
	/// ```
	/// let mat = cvmath::Mat2(1, 2, 3, 4).transform2();
	/// let value = mat.into_row_major();
	/// let expected = [[1, 2, 0], [3, 4, 0]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn transform2(self) -> Transform2<T> where T: Zero {
		Transform2 {
			a11: self.a11, a12: self.a12, a13: T::ZERO,
			a21: self.a21, a22: self.a22, a23: T::ZERO,
		}
	}
	/// Adds a translation to the matrix.
	///
	/// ```
	/// let mat = cvmath::Mat2::IDENTITY.translate(cvmath::Vec2(5, 6));
	/// let value = mat.into_row_major();
	/// let expected = [[1, 0, 5], [0, 1, 6]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn translate(self, trans: Vec2<T>) -> Transform2<T> {
		let Vec2 { x: a13, y: a23 } = trans;
		Transform2 {
			a11: self.a11, a12: self.a12, a13,
			a21: self.a21, a22: self.a22, a23,
		}
	}
}

impl<T> Mat2<T> {
	#[inline]
	fn as_array(&self) -> &[T; 4] {
		unsafe { mem::transmute(self)}
	}
	/// Imports the matrix from a row-major layout.
	///
	/// ```
	/// let mat = cvmath::Mat2::from_row_major([[1, 2], [3, 4]]);
	/// let expected = cvmath::Mat2(1, 2, 3, 4);
	/// assert_eq!(expected, mat);
	/// ```
	#[inline]
	pub fn from_row_major(mat: [[T; 2]; 2]) -> Mat2<T> {
		let [[a11, a12], [a21, a22]] = mat;
		Mat2 {
			a11, a12,
			a21, a22,
		}
	}
	/// Imports the matrix from a column-major layout.
	///
	/// ```
	/// let mat = cvmath::Mat2::from_column_major([[1, 3], [2, 4]]);
	/// let expected = cvmath::Mat2(1, 2, 3, 4);
	/// assert_eq!(expected, mat);
	/// ```
	#[inline]
	pub fn from_column_major(mat: [[T; 2]; 2]) -> Mat2<T> {
		let [[a11, a21], [a12, a22]] = mat;
		Mat2 {
			a11, a12,
			a21, a22,
		}
	}
	/// Exports the matrix as a row-major array.
	///
	/// ```
	/// let mat = cvmath::Mat2(1, 2, 3, 4).into_row_major();
	/// let expected = [[1, 2], [3, 4]];
	/// assert_eq!(expected, mat);
	/// ```
	#[inline]
	pub fn into_row_major(self) -> [[T; 2]; 2] {
		[
			[self.a11, self.a12],
			[self.a21, self.a22],
		]
	}
	/// Exports the matrix as a column-major array.
	///
	/// ```
	/// let mat = cvmath::Mat2(1, 2, 3, 4).into_column_major();
	/// let expected = [[1, 3], [2, 4]];
	/// assert_eq!(expected, mat);
	/// ```
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
	///
	/// ```
	/// let mat = cvmath::Mat2::compose(cvmath::Vec2(1, 2), cvmath::Vec2(3, 4));
	/// let value = mat.into_row_major();
	/// let expected = [[1, 3], [2, 4]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn compose(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
		Mat2 {
			a11: x.x, a12: y.x,
			a21: x.y, a22: y.y,
		}
	}
	/// Gets the transformed X basis vector.
	///
	/// ```
	/// let value = cvmath::Mat2(1, 2, 3, 4).x();
	/// let expected = cvmath::Vec2(1, 3);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn x(self) -> Vec2<T> {
		Vec2 { x: self.a11, y: self.a21 }
	}
	/// Gets the transformed Y basis vector.
	///
	/// ```
	/// let value = cvmath::Mat2(1, 2, 3, 4).y();
	/// let expected = cvmath::Vec2(2, 4);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn y(self) -> Vec2<T> {
		Vec2 { x: self.a12, y: self.a22 }
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Mat2<T> {
	/// Computes the determinant.
	///
	/// ```
	/// let value = cvmath::Mat2(1, 2, 3, 4).det();
	/// assert_eq!(-2, value);
	/// ```
	#[inline]
	pub fn det(self) -> T {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	/// Computes the trace.
	///
	/// ```
	/// let value = cvmath::Mat2(1, 2, 3, 4).trace();
	/// assert_eq!(5, value);
	/// ```
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22
	}
	/// Computes the squared Frobenius norm (sum of squares of all matrix elements).
	///
	/// This measure is useful for quickly checking matrix magnitude or comparing matrices without the cost of a square root operation.
	///
	/// To check if a matrix is effectively zero, test if `flat_norm_sqr()` is below a small epsilon threshold.
	///
	/// ```
	/// let value = cvmath::Mat2(1, 2, 3, 4).flat_norm_sqr();
	/// assert_eq!(30, value);
	/// ```
	#[inline]
	pub fn flat_norm_sqr(self) -> T {
		self.a11 * self.a11 + self.a12 * self.a12 +
		self.a21 * self.a21 + self.a22 * self.a22
	}
	/// Attempts to invert the matrix.
	///
	/// ```
	/// let mat = cvmath::Mat2(1.0, 2.0, 3.0, 4.0);
	/// let value = mat.try_invert();
	/// let expected = Some(cvmath::Mat2(-2.0, 1.0, 1.5, -0.5));
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn try_invert(self) -> Option<Mat2<T>> where T: Float {
		let det = self.det();
		if det == T::ZERO {
			return None;
		}
		Some(self.adjugate() * (T::ONE / det))
	}
	/// Computes the inverse matrix.
	///
	/// Returns the zero matrix if the determinant is exactly zero.
	///
	/// ```
	/// let mat = cvmath::Mat2(1.0, 2.0, 3.0, 4.0);
	/// let value = mat.inverse();
	/// let expected = cvmath::Mat2(-2.0, 1.0, 1.5, -0.5);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn inverse(self) -> Mat2<T> where T: Float {
		self.try_invert().unwrap_or(Mat2::ZERO)
	}
	/// Returns the transposed matrix.
	///
	/// ```
	/// let mat = cvmath::Mat2(1, 2, 3, 4);
	/// assert_eq!(cvmath::Mat2(1, 3, 2, 4), mat.transpose());
	/// ```
	#[inline]
	pub fn transpose(self) -> Mat2<T> {
		Mat2 {
			a11: self.a11, a12: self.a21,
			a21: self.a12, a22: self.a22,
		}
	}
	/// Computes the adjugate matrix.
	///
	/// ```
	/// let value = cvmath::Mat2(1, 2, 3, 4).adjugate();
	/// let expected = cvmath::Mat2(4, -2, -3, 1);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn adjugate(self) -> Mat2<T> {
		Mat2 {
			a11:  self.a22, a12: -self.a12,
			a21: -self.a21, a22:  self.a11,
		}
	}
	/// Linear interpolation between the matrix elements.
	///
	/// ```
	/// let source = cvmath::Mat2::IDENTITY;
	/// let target = cvmath::Mat2::scaling(cvmath::Vec2(3.0, 5.0));
	/// let value = source.lerp(target, 0.5);
	/// let expected = cvmath::Mat2(2.0, 0.0, 0.0, 3.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn lerp(self, rhs: Mat2<T>, t: T) -> Mat2<T> where T: Float {
		Mat2 {
			a11: self.a11 + (rhs.a11 - self.a11) * t,
			a12: self.a12 + (rhs.a12 - self.a12) * t,
			a21: self.a21 + (rhs.a21 - self.a21) * t,
			a22: self.a22 + (rhs.a22 - self.a22) * t,
		}
	}
	/// Applies the transformation around a given origin.
	///
	/// ```
	/// let rotation = cvmath::Mat2::rotation(cvmath::Angle::deg(90.0));
	/// let mat = rotation.around(cvmath::Vec2(2.0f64, 3.0));
	/// let value = (mat * cvmath::Vec2(3.0, 3.0)).cast::<f32>();
	/// let expected = cvmath::Vec2(2.0f32, 4.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn around(self, origin: Vec2<T>) -> Transform2<T> where T: Float {
		let to_origin = Transform2::translation(-origin);
		let from_origin = Transform2::translation(origin);
		from_origin * self * to_origin
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

//----------------------------------------------------------------
// Formatting

impl<T: fmt::Display> fmt::Display for Mat2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Mat2(")?;
		print::print(&move |i| &self.as_array()[i], 0x22, f)?;
		f.write_str(")")
	}
}
impl<T: fmt::Debug> fmt::Debug for Mat2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Mat2(")?;
		print::print(&move |i| print::Debug(&self.as_array()[i]), 0x22, f)?;
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

#[test]
fn test_fmt() {
	let mat = Mat2(1.15, 2.0, 3.3, 4.4);
	assert_eq!(format!("{mat}"), "Mat2([1.15, 2], [3.3, 4.4])");
	assert_eq!(format!("{mat:#}"), "Mat2(\n [1.15,    2],\n [ 3.3,  4.4])");

	let mat = Mat2(1.23456789, 2.3456789, 3.45678901, 4.56789012);
	assert_eq!(format!("{mat}"), "Mat2([1.23456789, 2.3456789], [3.45678901, 4.56789012])");
}

#[test]
fn test_fmt_precision_behavior() {
	let mat = Mat2(1.15, 2.0, 3.3, 4.4);
	assert_eq!(format!("{mat:.2}"), "Mat2([1.15, 2.00], [3.30, 4.40])");
	assert_eq!(format!("{mat:#.2}"), "Mat2(\n [1.15,    2],\n [ 3.3,  4.4])");
}

#[test]
fn test_fmt_width_behavior() {
	let mat = Mat2(1.15, 2.0, 3.3, 4.4);
	assert_eq!(format!("{mat:8.2}"), "Mat2([    1.15,     2.00], [    3.30,     4.40])");
	assert_eq!(format!("{mat:#8.2}"), "Mat2(\n [1.15,    2],\n [ 3.3,  4.4])");
}

#[test]
fn test_rotation() {
	let angle = Angle::deg(45.0);
	let rot = Mat2::rotation(angle);
	let v = Vec2(1.0, 0.0);
	let value = (rot * v).angle(v).cast::<f32>();
	let expected = angle.cast::<f32>();
	assert_eq!(value, expected);
}

#[test]
fn test_rotation_between() {
	let from = Vec2(1.0, 1.0).norm();
	let to = Vec2(-1.0, 1.0).norm();
	let matrix = Mat2::rotation_between(from, to);
	let value = (matrix * from).cast::<f32>();
	let expected = to.cast::<f32>();
	assert_eq!(expected, value);
}

#[test]
fn test_rotation_between_opposite() {
	let from = Vec2f::X;
	let to = -Vec2f::X;
	let rot = Mat2f::rotation_between(from, to);
	let value = (rot * from).cast::<f32>();
	let expected = to.cast::<f32>();
	assert_eq!(expected, value);
}
