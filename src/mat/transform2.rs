/*!
2D affine transformation matrix.
*/

use super::*;

/// 2D affine transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
///
/// The third row is implied to be `[0, 0, 1]` and is omitted.
///
/// Row-major storage with column-major semantics.
///
/// Stored in row-major order (fields appear in reading order),
/// but interpreted as column-major: each column is a transformed basis vector,
/// and matrices are applied to column vectors via `mat * vec`.
#[derive(Copy, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Transform2<T> {
	pub a11: T, pub a12: T, pub a13: T,
	pub a21: T, pub a22: T, pub a23: T,
}

/// Transform2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Transform2<T>(
	a11: T, a12: T, a13: T,
	a21: T, a22: T, a23: T,
) -> Transform2<T> {
	Transform2 { a11, a12, a13, a21, a22, a23 }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Transform2<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Transform2<T> {
	/// Constructs a new matrix from components.
	#[inline]
	pub const fn new(
		a11: T, a12: T, a13: T,
		a21: T, a22: T, a23: T,
	) -> Transform2<T> {
		Transform2 {
			a11, a12, a13,
			a21, a22, a23,
		}
	}
}

impl<T: Zero> Transform2<T> {
	/// Zero matrix.
	pub const ZERO: Transform2<T> = Transform2 {
		a11: T::ZERO, a12: T::ZERO, a13: T::ZERO,
		a21: T::ZERO, a22: T::ZERO, a23: T::ZERO,
	};
}

impl<T: Zero + One> Transform2<T> {
	/// Identity matrix.
	pub const IDENTITY: Transform2<T> = Transform2 {
		a11: T::ONE,  a12: T::ZERO, a13: T::ZERO,
		a21: T::ZERO, a22: T::ONE,  a23: T::ZERO,
	};
}

impl<T: Float> Transform2<T> {
	/// Translation matrix.
	///
	/// ```
	/// let mat = cvmath::Transform2::translation(cvmath::Vec2(5.0, 6.0));
	/// let value = mat * cvmath::Vec2(1.0, 2.0);
	/// let expected = cvmath::Vec2(6.0, 8.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn translation(trans: Vec2<T>) -> Transform2<T> {
		Transform2 {
			a11: T::ONE,  a12: T::ZERO, a13: trans.x,
			a21: T::ZERO, a22: T::ONE,  a23: trans.y,
		}
	}

	/// Scaling matrix.
	///
	/// Scales around the origin.
	///
	/// ```
	/// let mat = cvmath::Transform2::scaling(cvmath::Vec2(2.0, 3.0));
	/// let value = mat * cvmath::Vec2(4.0, 5.0);
	/// let expected = cvmath::Vec2(8.0, 15.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn scaling(scale: Vec2<T>) -> Transform2<T> {
		Transform2 {
			a11: scale.x, a12: T::ZERO, a13: T::ZERO,
			a21: T::ZERO, a22: scale.y, a23: T::ZERO,
		}
	}

	/// Rotation matrix.
	///
	/// Rotates around the origin.
	///
	/// ```
	/// let mat = cvmath::Transform2::rotation(cvmath::Angle::deg(90.0));
	/// let value = (mat * cvmath::Vec2(1.0f64, 1.0)).cast::<f32>();
	/// let expected = cvmath::Vec2(-1.0f32, 1.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn rotation(angle: Angle<T>) -> Transform2<T> {
		Mat2::rotation(angle).transform2()
	}

	/// Rotation matrix between two vectors. See [Mat2::rotation_between].
	///
	/// ```
	/// let from = cvmath::Vec2(1.0, 1.0).norm();
	/// let to = cvmath::Vec2(-1.0, 1.0).norm();
	/// let mat = cvmath::Transform2::rotation_between(from, to);
	/// let value = (mat * from).cast::<f32>();
	/// let expected = to.cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn rotation_between(from: Vec2<T>, to: Vec2<T>) -> Transform2<T> {
		Mat2::rotation_between(from, to).transform2()
	}

	/// Skewing matrix.
	///
	/// ```
	/// let mat = cvmath::Transform2::skewing(cvmath::Vec2(2.0, 3.0));
	/// let value = mat * cvmath::Vec2(4.0, 5.0);
	/// let expected = cvmath::Vec2(14.0, 17.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn skewing(skew: Vec2<T>) -> Transform2<T> {
		Transform2 {
			a11: T::ONE, a12: skew.x, a13: T::ZERO,
			a21: skew.y, a22: T::ONE, a23: T::ZERO,
		}
	}

	/// Reflection matrix.
	///
	/// Reflects around the given axis.
	/// If axis is the zero vector, returns a point reflection around the origin.
	///
	/// ```
	/// let mat = cvmath::Transform2::reflection(cvmath::Vec2::<f64>::Y);
	/// let value = mat * cvmath::Vec2(2.0, 3.0);
	/// let expected = cvmath::Vec2(-2.0, 3.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn reflection(line: Vec2<T>) -> Transform2<T> {
		Mat2::reflection(line).transform2()
	}

	/// Projection matrix.
	///
	/// Projects onto the given axis.
	/// If axis is the zero vector, returns the zero matrix.
	///
	/// ```
	/// let mat = cvmath::Transform2::projection(cvmath::Vec2::<f64>::X);
	/// let value = mat * cvmath::Vec2(2.0, 3.0);
	/// let expected = cvmath::Vec2(2.0, 0.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn projection(line: Vec2<T>) -> Transform2<T> {
		Mat2::projection(line).transform2()
	}

	/// Fit matrix.
	///
	/// Fits coordinates from a source rect into a target rect.
	///
	/// ```
	/// let source = cvmath::Bounds2::c(10.0, 20.0, 30.0, 40.0);
	/// let target = cvmath::Bounds2::c(100.0, 200.0, 200.0, 260.0);
	/// let mat = cvmath::Transform2::fit(source, target);
	/// let value = mat * cvmath::Point2(10.0, 20.0);
	/// let expected = cvmath::Point2(100.0, 200.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn fit(source: Bounds2<T>, target: Bounds2<T>) -> Transform2<T> {
		let scale = target.size() / source.size();
		let Vec2 { x: a13, y: a23 } = target.mins - source.mins * scale;
		Transform2 {
			a11: scale.x, a12: T::ZERO, a13,
			a21: T::ZERO, a22: scale.y, a23,
		}
	}

	/// Orthographic matrix.
	///
	/// Fits the coordinates from a rectangle to `x = [-1, 1]` and `y = [1, -1]`.
	///
	/// ```
	/// let rect = cvmath::Bounds2::c(10.0, 20.0, 30.0, 40.0);
	/// let mat = cvmath::Transform2::ortho(rect);
	/// let value = mat * cvmath::Point2(10.0, 20.0);
	/// let expected = cvmath::Point2(-1.0, 1.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn ortho(rect: Bounds2<T>) -> Transform2<T> {
		let Bounds2 {
			mins: Vec2 { x: left, y: top },
			maxs: Vec2 { x: right, y: bottom },
		} = rect;

		let inv_width = T::ONE / (right - left);
		let inv_height = T::ONE / (bottom - top);

		let a11 = T::TWO * inv_width;
		let a22 = -T::TWO * inv_height; // flip Y

		let a13 = -(right + left) * inv_width;
		let a23 = (bottom + top) * inv_height; // flip Y

		Transform2 {
			a11, a12: T::ZERO, a13,
			a21: T::ZERO, a22, a23,
		}
	}

	/// Maps NDC coordinates to screen space.
	///
	/// ```
	/// let mat = cvmath::Transform2::screen(640.0, 480.0);
	/// let value = mat * cvmath::Vec2(-1.0, -1.0);
	/// let expected = cvmath::Vec2(0.0, 480.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn screen(width: T, height: T) -> Transform2<T> {
		let half_width = width / T::TWO;
		let half_height = height / T::TWO;
		let x = Vec2 { x: half_width, y: T::ZERO };
		let y = Vec2 { x: T::ZERO, y: -half_height };
		let t = Vec2 { x: half_width, y: half_height };
		Transform2::compose(x, y, t)
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Transform2<T> {
	/// Casts to a matrix of different type with the same dimensions.
	#[inline]
	pub fn cast<U>(self) -> Transform2<U> where T: CastTo<U> {
		Transform2 {
			a11: self.a11.cast_to(), a12: self.a12.cast_to(), a13: self.a13.cast_to(),
			a21: self.a21.cast_to(), a22: self.a22.cast_to(), a23: self.a23.cast_to(),
		}
	}
}

impl<T: Zero + One> Transform2<T> {
	/// Converts to a 3x3 matrix.
	///
	/// ```
	/// let mat = cvmath::Transform2(1, 2, 3, 4, 5, 6).mat3();
	/// let value = mat.into_row_major();
	/// let expected = [[1, 2, 3], [4, 5, 6], [0, 0, 1]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn mat3(self) -> Mat3<T> {
		self.into()
	}
}

impl<T> Transform2<T> {
	#[inline]
	fn as_array(&self) -> &[T; 6] {
		unsafe { mem::transmute(self)}
	}
	/// Imports the matrix from a row-major layout.
	///
	/// ```
	/// let mat = cvmath::Transform2::from_row_major([[1, 2, 3], [4, 5, 6]]);
	/// let expected = cvmath::Transform2(1, 2, 3, 4, 5, 6);
	/// assert_eq!(expected, mat);
	/// ```
	#[inline]
	pub fn from_row_major(mat: [[T; 3]; 2]) -> Transform2<T> {
		let [[a11, a12, a13], [a21, a22, a23]] = mat;
		Transform2 {
			a11, a12, a13,
			a21, a22, a23,
		}
	}
	/// Imports the matrix from a column-major layout.
	///
	/// ```
	/// let mat = cvmath::Transform2::from_column_major([[1, 4], [2, 5], [3, 6]]);
	/// let expected = cvmath::Transform2(1, 2, 3, 4, 5, 6);
	/// assert_eq!(expected, mat);
	/// ```
	#[inline]
	pub fn from_column_major(mat: [[T; 2]; 3]) -> Transform2<T> {
		let [[a11, a21], [a12, a22], [a13, a23]] = mat;
		Transform2 {
			a11, a12, a13,
			a21, a22, a23,
		}
	}
	/// Exports the matrix as a row-major array.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).into_row_major();
	/// let expected = [[1, 2, 3], [4, 5, 6]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn into_row_major(self) -> [[T; 3]; 2] {
		[
			[self.a11, self.a12, self.a13],
			[self.a21, self.a22, self.a23],
		]
	}
	/// Exports the matrix as a column-major array.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).into_column_major();
	/// let expected = [[1, 4], [2, 5], [3, 6]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn into_column_major(self) -> [[T; 2]; 3] {
		[
			[self.a11, self.a21],
			[self.a12, self.a22],
			[self.a13, self.a23],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Transform2<T> {
	/// Composes the matrix from basis vectors.
	///
	/// ```
	/// let mat = cvmath::Transform2::compose(cvmath::Vec2(1, 2), cvmath::Vec2(3, 4), cvmath::Vec2(5, 6));
	/// let value = mat.into_row_major();
	/// let expected = [[1, 3, 5], [2, 4, 6]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn compose(x: Vec2<T>, y: Vec2<T>, t: Vec2<T>) -> Transform2<T> {
		Transform2 {
			a11: x.x, a12: y.x, a13: t.x,
			a21: x.y, a22: y.y, a23: t.y,
		}
	}
	/// Gets the transformed X basis vector.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).x();
	/// let expected = cvmath::Vec2(1, 4);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn x(self) -> Vec2<T> {
		Vec2 { x: self.a11, y: self.a21 }
	}
	/// Gets the transformed Y basis vector.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).y();
	/// let expected = cvmath::Vec2(2, 5);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn y(self) -> Vec2<T> {
		Vec2 { x: self.a12, y: self.a22 }
	}
	/// Gets the translation vector.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).t();
	/// let expected = cvmath::Vec2(3, 6);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn t(self) -> Vec2<T> {
		Vec2 { x: self.a13, y: self.a23 }
	}
	/// Gets the rotation matrix.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).mat2();
	/// let expected = cvmath::Mat2(1, 2, 4, 5);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn mat2(self) -> Mat2<T> {
		Mat2 {
			a11: self.a11, a12: self.a12,
			a21: self.a21, a22: self.a22,
		}
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Transform2<T> {
	/// Computes the determinant.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).det();
	/// assert_eq!(-3, value);
	/// ```
	#[inline]
	pub fn det(self) -> T {
		self.a11 * self.a22 - self.a21 * self.a12
	}
	/// Computes the trace.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).trace();
	/// assert_eq!(7, value);
	/// ```
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22 + T::ONE
	}
	/// Computes the squared Frobenius norm (sum of squares of all matrix elements).
	///
	/// This measure is useful for quickly checking matrix magnitude or comparing matrices without the cost of a square root operation.
	///
	/// To check if a matrix is effectively zero, test if `flat_norm_sqr()` is below a small epsilon threshold.
	///
	/// ```
	/// let value = cvmath::Transform2(1, 2, 3, 4, 5, 6).flat_norm_sqr();
	/// assert_eq!(91, value);
	/// ```
	#[inline]
	pub fn flat_norm_sqr(self) -> T {
		self.a11 * self.a11 + self.a12 * self.a12 + self.a13 * self.a13 +
		self.a21 * self.a21 + self.a22 * self.a22 + self.a23 * self.a23
	}
	/// Attempts to invert the transform.
	///
	/// ```
	/// let mat = cvmath::Transform2(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
	/// let point = cvmath::Vec2(7.0f64, 8.0);
	/// let projected = mat * point;
	/// let value = (mat.try_invert().unwrap() * projected).cast::<f32>();
	/// let expected = point.cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn try_invert(self) -> Option<Transform2<T>> where T: Float {
		let det = self.det();
		if det == T::ZERO {
			return None;
		}

		let inv_det = T::ONE / det;
		Some(Transform2 {
			a11: self.a22 * inv_det,
			a12: -self.a12 * inv_det,
			a13: (self.a12 * self.a23 - self.a13 * self.a22) * inv_det,
			a21: -self.a21 * inv_det,
			a22: self.a11 * inv_det,
			a23: (self.a13 * self.a21 - self.a11 * self.a23) * inv_det,
		})
	}
	/// Computes the inverse matrix.
	///
	/// Returns the zero matrix if the determinant is exactly zero.
	///
	/// ```
	/// let mat = cvmath::Transform2(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
	/// let point = cvmath::Vec2(7.0f64, 8.0);
	/// let projected = mat * point;
	/// let value = (mat.inverse() * projected).cast::<f32>();
	/// let expected = point.cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn inverse(self) -> Transform2<T> where T: Float {
		self.try_invert().unwrap_or(Transform2::ZERO)
	}
	/// Linear interpolation between the matrix elements.
	///
	/// ```
	/// let source = cvmath::Transform2::IDENTITY;
	/// let target = cvmath::Transform2::translation(cvmath::Vec2(8.0, 10.0));
	/// let value = source.lerp(target, 0.5);
	/// let expected = cvmath::Transform2(1.0, 0.0, 4.0, 0.0, 1.0, 5.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn lerp(self, other: Transform2<T>, t: T) -> Transform2<T> where T: Float {
		Transform2 {
			a11: self.a11 + (other.a11 - self.a11) * t,
			a12: self.a12 + (other.a12 - self.a12) * t,
			a13: self.a13 + (other.a13 - self.a13) * t,
			a21: self.a21 + (other.a21 - self.a21) * t,
			a22: self.a22 + (other.a22 - self.a22) * t,
			a23: self.a23 + (other.a23 - self.a23) * t,
		}
	}
	/// Solves a linear system of equations represented by the matrix.
	///
	/// Interprets the affine transform rows as the system:
	///
	/// - _a_<sub>11</sub> **x** + _a_<sub>12</sub> **y** + _a_<sub>13</sub> = 0
	/// - _a_<sub>21</sub> **x** + _a_<sub>22</sub> **y** + _a_<sub>23</sub> = 0
	///
	/// Equivalently, this finds the vector `v` such that `self * v == Vec2::ZERO`.
	///
	/// Returns `None` if the determinant is zero (no unique solution).
	///
	/// ```
	/// let mat = cvmath::Transform2(
	/// 	1.0,  1.0, -5.0,
	/// 	1.0, -1.0, -1.0,
	/// );
	/// let value = mat.solve();
	/// let expected = Some(cvmath::Vec2(3.0, 2.0));
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn solve(self) -> Option<Vec2<T>> {
		let det = self.det();
		if det == T::ZERO {
			return None;
		}

		let inv_det = T::ONE / det;
		let x = (self.a12 * self.a23 - self.a13 * self.a22) * inv_det;
		let y = (self.a13 * self.a21 - self.a11 * self.a23) * inv_det;
		Some(Vec2 { x, y })
	}
	/// Applies the transformation around a given origin.
	///
	/// ```
	/// let rotation = cvmath::Transform2::rotation(cvmath::Angle::deg(90.0));
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
// Exponentiation

impl<T: Float> Transform2<T> {
	/// Raises the matrix to an integer power.
	///
	/// ```
	/// let mat = cvmath::Transform2::rotation(cvmath::Angle::deg(45.0));
	/// let value = mat.powi(3).cast::<f32>();
	/// let expected = cvmath::Transform2::rotation(cvmath::Angle::deg(45.0) * 3.0).cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	pub fn powi(self, exp: i32) -> Transform2<T> {
		if exp == 0 {
			return Transform2::IDENTITY;
		}

		let mut base = self;
		let mut exp = if exp < 0 {
			base = base.inverse();
			exp.unsigned_abs()
		} else {
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
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Transform2<T> {
	type Output = Vec2<T>;
	#[inline]
	fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + self.a23,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Transform2<T> {
	type Output = Vec2<T>;
	#[inline]
	fn mul(self, rhs: Vec3<T>) -> Vec2<T> {
		Vec2 {
			x: rhs.x * self.a11 + rhs.y * self.a12 + rhs.z * self.a13,
			y: rhs.x * self.a21 + rhs.y * self.a22 + rhs.z * self.a23,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Transform2<T>> for Transform2<T> {
	type Output = Transform2<T>;
	#[inline]
	fn mul(self, rhs: Transform2<T>) -> Transform2<T> {
		Transform2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat2<T>> for Transform2<T> {
	type Output = Transform2<T>;
	#[inline]
	fn mul(self, rhs: Mat2<T>) -> Transform2<T> {
		Transform2 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22,
			a13: self.a13,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22,
			a23: self.a23,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Transform2<T>> for Transform2<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Transform2<T>) {
		*self = *self * rhs;
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat2<T>> for Transform2<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Mat2<T>) {
		*self = *self * rhs;
	}
}

impl_mat_mul_scalar!(Transform2);
impl_mat_mul_vec!(Transform2, Vec2);
impl_mat_mul_vec!(Transform2, Vec3);
impl_mat_mul_mat!(Transform2);

//----------------------------------------------------------------
// Formatting

impl<T: fmt::Display> fmt::Display for Transform2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Transform2(")?;
		print::print(&move |i| &self.as_array()[i], 0x23, f)?;
		f.write_str(")")
	}
}
impl<T: fmt::Debug> fmt::Debug for Transform2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Transform2(")?;
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
		let x = Vec2(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);
		let y = Vec2(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);
		let t = Vec2(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
		);

		let mat = Transform2::compose(x, y, t);
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
