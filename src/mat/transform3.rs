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
	///
	/// ```
	/// let mat = cvmath::Transform3::translation(cvmath::Vec3(5.0, 6.0, 7.0));
	/// let value = mat * cvmath::Vec3(1.0, 2.0, 3.0);
	/// let expected = cvmath::Vec3(6.0, 8.0, 10.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn translation(trans: Vec3<T>) -> Transform3<T> {
		let Vec3 { x: a14, y: a24, z: a34 } = trans;
		Transform3 { a14, a24, a34, ..Transform3::IDENTITY }
	}

	/// Scaling matrix.
	///
	/// Scales around the origin.
	///
	/// ```
	/// let mat = cvmath::Transform3::scaling(cvmath::Vec3(2.0, 3.0, 4.0));
	/// let value = mat * cvmath::Vec3(4.0, 5.0, 6.0);
	/// let expected = cvmath::Vec3(8.0, 15.0, 24.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn scaling(scale: Vec3<T>) -> Transform3<T> {
		let Vec3 { x: a11, y: a22, z: a33 } = scale;
		Transform3 { a11, a22, a33, ..Transform3::IDENTITY }
	}

	/// Rotation matrix around an axis. See [Mat3::rotation] for details.
	///
	/// ```
	/// let mat = cvmath::Transform3::rotation(cvmath::Vec3::Z, cvmath::Angle::deg(90.0));
	/// let value = (mat * cvmath::Vec3(1.0f64, 1.0, 1.0)).cast::<f32>();
	/// let expected = cvmath::Vec3(-1.0f32, 1.0, 1.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn rotation(axis: Vec3<T>, angle: Angle<T>) -> Transform3<T> {
		Mat3::rotation(axis, angle).transform3()
	}

	/// Rotation matrix between two vectors. See [Mat3::rotation_between] for details.
	///
	/// ```
	/// let from = cvmath::Vec3(1.0, 0.0, 0.0);
	/// let to = cvmath::Vec3(0.0, 1.0, 0.0);
	/// let mat = cvmath::Transform3::rotation_between(from, to);
	/// let value = (mat * from).cast::<f32>();
	/// let expected = to.cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn rotation_between(from: Vec3<T>, to: Vec3<T>) -> Transform3<T> {
		Mat3::rotation_between(from, to).transform3()
	}

	/// Projection matrix.
	///
	/// Projects onto the given plane, returning the zero matrix if the plane normal is zero.
	///
	/// ```
	/// let plane = cvmath::Plane3(cvmath::Vec3(0.0f64, 0.0, 2.0), -4.0);
	/// let value = cvmath::Transform3::projection(plane) * cvmath::Vec3(2.0, 3.0, 7.0);
	/// let expected = cvmath::Vec3(2.0, 3.0, 2.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn projection(plane: Plane3<T>) -> Transform3<T> {
		let Plane3 { normal, distance } = plane;
		let denom = Vec3::dot(normal, normal);
		if denom > T::EPSILON {
			let project3 = Mat3::IDENTITY - Mat3::outer_product(normal, normal) * (T::ONE / denom);
			project3.translate(normal * -(distance * (T::ONE / denom)))
		}
		else {
			Transform3::ZERO
		}
	}

	/// Reflection matrix.
	///
	/// Reflects across the given plane, returning a point reflection around the origin if the plane normal is zero.
	///
	/// ```
	/// let plane = cvmath::Plane3(cvmath::Vec3(0.0f64, 0.0, 2.0), -4.0);
	/// let value = cvmath::Transform3::reflection(plane) * cvmath::Vec3(2.0, 3.0, 7.0);
	/// let expected = cvmath::Vec3(2.0, 3.0, -3.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn reflection(plane: Plane3<T>) -> Transform3<T> {
		let Plane3 { normal, distance } = plane;
		let denom = Vec3::dot(normal, normal);
		if denom > T::EPSILON {
			let reflect3 = Mat3::IDENTITY - Mat3::outer_product(normal, normal) * (T::TWO / denom);
			reflect3.translate(normal * -(distance * (T::TWO / denom)))
		}
		else {
			Transform3::scaling(-Vec3::<T>::ONE)
		}
	}
}

impl<T: Float> Transform3<T> {
	/// Look-at matrix.
	///
	/// ```
	/// let mat = cvmath::Transform3::look_at(cvmath::Vec3(0.0, 0.0, 5.0), cvmath::Vec3(0.0, 0.0, 0.0), cvmath::Vec3::Y, cvmath::Hand::RH);
	/// let value = mat * cvmath::Vec3(0.0, 0.0, 5.0);
	/// let expected = cvmath::Vec3(0.0, 0.0, 0.0);
	/// assert_eq!(expected, value);
	/// ```
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
	/// Axis conventions relative to the viewer's screen:
	///
	/// Right-handed mode ([Hand::RH]):
	/// * +X → right side of the screen
	/// * +Y → top of the screen
	/// * +Z → increases into the scene
	///
	/// Left-handed mode ([Hand::LH]):
	/// * +X → right side of the screen
	/// * +Y → top of the screen
	/// * +Z → decreases into the scene
	///
	/// The `clip` parameter controls the mapping of the Z range into clip space.
	///
	/// ```
	/// let bounds = cvmath::Bounds3(cvmath::Vec3(-1.0, -1.0, 0.0), cvmath::Vec3(1.0, 1.0, 1.0));
	/// let mat = cvmath::Transform3::ortho(bounds, (cvmath::Hand::LH, cvmath::Clip::ZO));
	/// let value = mat * cvmath::Vec3(0.5, -0.5, 0.25);
	/// let expected = cvmath::Vec3(0.5, -0.5, 0.25);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn ortho(bounds: Bounds3<T>, (hand, clip): (Hand, Clip)) -> Transform3<T> {
		let Bounds3 {
			mins: Vec3 { x: left, y: bottom, z: near },
			maxs: Vec3 { x: right, y: top, z: far },
		} = bounds;

		debug_assert!(near != far);
		debug_assert!(left != right);
		debug_assert!(bottom != top);

		let a11 = T::TWO / (right - left);
		let a14 = -(right + left) / (right - left);
		let a22 = T::TWO / (top - bottom);
		let a24 = -(top + bottom) / (top - bottom);
		let a33 = match clip { Clip::ZO => T::ONE, Clip::NO => T::TWO } / (far - near);
		let a33 = match hand { Hand::LH => a33, Hand::RH => -a33 };
		let a34 = -match clip { Clip::ZO => near, Clip::NO => far + near } / (far - near);

		Transform3 { a11, a14, a22, a24, a33, a34, ..Self::IDENTITY }
	}

	/// Maps NDC coordinates to screen space.
	///
	/// ```
	/// let mat = cvmath::Transform3::screen(640.0, 480.0);
	/// let value = mat * cvmath::Vec3(-1.0, -1.0, 0.0);
	/// let expected = cvmath::Vec3(0.0, 480.0, 1.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn screen(width: T, height: T) -> Transform3<T> {
		let half_width = width / T::TWO;
		let half_height = height / T::TWO;
		let x = Vec3 { x: half_width, y: T::ZERO, z: T::ZERO };
		let y = Vec3 { x: T::ZERO, y: -half_height, z: T::ZERO };
		let z = Vec3::Z;
		let t = Vec3 { x: half_width, y: half_height, z: T::ONE };
		Transform3::compose(x, y, z, t)
	}

	/// Maps NDC coordinates to viewport space.
	///
	/// ```
	/// let bounds = cvmath::Bounds2::c(10.0, 20.0, 110.0, 220.0);
	/// let mat = cvmath::Transform3::viewport(bounds);
	/// let value = mat * cvmath::Vec3(-1.0, -1.0, 0.0);
	/// let expected = cvmath::Vec3(10.0, 220.0, 1.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn viewport(bounds: Bounds2<T>) -> Transform3<T> {
		let Bounds2 {
			mins: Vec2 { x: left, y: top },
			maxs: Vec2 { x: right, y: bottom },
		} = bounds;

		let half_width = (right - left) / T::TWO;
		let half_height = (bottom - top) / T::TWO;
		let x = Vec3 { x: half_width, y: T::ZERO, z: T::ZERO };
		let y = Vec3 { x: T::ZERO, y: -half_height, z: T::ZERO };
		let z = Vec3::Z;
		let t = Vec3 { x: left + half_width, y: top + half_height, z: T::ONE };
		Transform3::compose(x, y, z, t)
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Transform3<T> {
	/// Casts to a matrix of different type with the same dimensions.
	#[inline]
	pub fn cast<U>(self) -> Transform3<U> where T: CastTo<U> {
		Transform3 {
			a11: self.a11.cast_to(), a12: self.a12.cast_to(), a13: self.a13.cast_to(), a14: self.a14.cast_to(),
			a21: self.a21.cast_to(), a22: self.a22.cast_to(), a23: self.a23.cast_to(), a24: self.a24.cast_to(),
			a31: self.a31.cast_to(), a32: self.a32.cast_to(), a33: self.a33.cast_to(), a34: self.a34.cast_to(),
		}
	}
}

impl<T: Zero + One> Transform3<T> {
	/// Converts to a 4x4 matrix.
	///
	/// ```
	/// let mat = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).mat4();
	/// let value = mat.into_row_major();
	/// let expected = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [0, 0, 0, 1]];
	/// assert_eq!(expected, value);
	/// ```
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
	///
	/// ```
	/// let mat = cvmath::Transform3::from_row_major([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
	/// let expected = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
	/// assert_eq!(expected, mat);
	/// ```
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
	///
	/// ```
	/// let mat = cvmath::Transform3::from_column_major([[1, 5, 9], [2, 6, 10], [3, 7, 11], [4, 8, 12]]);
	/// let expected = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
	/// assert_eq!(expected, mat);
	/// ```
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
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).into_row_major();
	/// let expected = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn into_row_major(self) -> [[T; 4]; 3] {
		[
			[self.a11, self.a12, self.a13, self.a14],
			[self.a21, self.a22, self.a23, self.a24],
			[self.a31, self.a32, self.a33, self.a34],
		]
	}
	/// Exports the matrix as a column-major array.
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).into_column_major();
	/// let expected = [[1, 5, 9], [2, 6, 10], [3, 7, 11], [4, 8, 12]];
	/// assert_eq!(expected, value);
	/// ```
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
	///
	/// ```
	/// let mat = cvmath::Transform3::compose(cvmath::Vec3(1, 2, 3), cvmath::Vec3(4, 5, 6), cvmath::Vec3(7, 8, 9), cvmath::Vec3(10, 11, 12));
	/// let value = mat.into_row_major();
	/// let expected = [[1, 4, 7, 10], [2, 5, 8, 11], [3, 6, 9, 12]];
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn compose(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>, t: Vec3<T>) -> Transform3<T> {
		Transform3 {
			a11: x.x, a12: y.x, a13: z.x, a14: t.x,
			a21: x.y, a22: y.y, a23: z.y, a24: t.y,
			a31: x.z, a32: y.z, a33: z.z, a34: t.z,
		}
	}
	/// Gets the transformed X basis vector.
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).x();
	/// let expected = cvmath::Vec3(1, 5, 9);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn x(self) -> Vec3<T> {
		Vec3 { x: self.a11, y: self.a21, z: self.a31 }
	}
	/// Gets the transformed Y basis vector.
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).y();
	/// let expected = cvmath::Vec3(2, 6, 10);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn y(self) -> Vec3<T> {
		Vec3 { x: self.a12, y: self.a22, z: self.a32 }
	}
	/// Gets the transformed Z basis vector.
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).z();
	/// let expected = cvmath::Vec3(3, 7, 11);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn z(self) -> Vec3<T> {
		Vec3 { x: self.a13, y: self.a23, z: self.a33 }
	}
	/// Gets the translation vector.
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).t();
	/// let expected = cvmath::Vec3(4, 8, 12);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn t(self) -> Vec3<T> {
		Vec3 { x: self.a14, y: self.a24, z: self.a34 }
	}
	/// Gets the rotation matrix.
	///
	/// ```
	/// let value = cvmath::Transform3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).mat3();
	/// let expected = cvmath::Mat3(1, 2, 3, 5, 6, 7, 9, 10, 11);
	/// assert_eq!(expected, value);
	/// ```
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
	///
	/// ```
	/// let value = cvmath::Transform3::scaling(cvmath::Vec3(2.0, 3.0, 4.0)).det();
	/// assert_eq!(24.0, value);
	/// ```
	#[inline]
	pub fn det(self) -> T {
		self.a11 * (self.a22 * self.a33 - self.a23 * self.a32) +
		self.a12 * (self.a23 * self.a31 - self.a21 * self.a33) +
		self.a13 * (self.a21 * self.a32 - self.a22 * self.a31)
	}
	/// Computes the trace.
	///
	/// ```
	/// let value = cvmath::Transform3::scaling(cvmath::Vec3(2.0, 3.0, 4.0)).trace();
	/// assert_eq!(10.0, value);
	/// ```
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22 + self.a33 + T::ONE
	}
	/// Computes the squared Frobenius norm (sum of squares of all matrix elements).
	///
	/// This measure is useful for quickly checking matrix magnitude or comparing matrices without the cost of a square root operation.
	///
	/// To check if a matrix is effectively zero, test if `flat_norm_sqr()` is below a small epsilon threshold.
	///
	/// ```
	/// let value = cvmath::Transform3(2, 0, 0, 1, 0, 3, 0, 2, 0, 0, 4, 3).flat_norm_sqr();
	/// assert_eq!(43, value);
	/// ```
	#[inline]
	pub fn flat_norm_sqr(self) -> T {
		self.a11 * self.a11 + self.a12 * self.a12 + self.a13 * self.a13 + self.a14 * self.a14 +
		self.a21 * self.a21 + self.a22 * self.a22 + self.a23 * self.a23 + self.a24 * self.a24 +
		self.a31 * self.a31 + self.a32 * self.a32 + self.a33 * self.a33 + self.a34 * self.a34
	}
	/// Attempts to invert the transform.
	///
	/// ```
	/// let mat = cvmath::Transform3::translation(cvmath::Vec3(8.0, 10.0, 12.0)) * cvmath::Transform3::scaling(cvmath::Vec3(2.0, 4.0, 8.0));
	/// let point = cvmath::Vec3(7.0f64, 8.0, 9.0);
	/// let projected = mat * point;
	/// let value = (mat.try_invert().unwrap() * projected).cast::<f32>();
	/// let expected = point.cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
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
	/// Returns the zero matrix if the determinant is exactly zero.
	///
	/// ```
	/// let mat = cvmath::Transform3::translation(cvmath::Vec3(8.0, 10.0, 12.0)) * cvmath::Transform3::scaling(cvmath::Vec3(2.0, 4.0, 8.0));
	/// let point = cvmath::Vec3(7.0f64, 8.0, 9.0);
	/// let projected = mat * point;
	/// let value = (mat.inverse() * projected).cast::<f32>();
	/// let expected = point.cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn inverse(self) -> Transform3<T> where T: Float {
		self.try_invert().unwrap_or(Transform3::ZERO)
	}
	/// Linear interpolation between the matrix elements.
	///
	/// ```
	/// let source = cvmath::Transform3::IDENTITY;
	/// let target = cvmath::Transform3::translation(cvmath::Vec3(8.0, 10.0, 12.0));
	/// let value = source.lerp(target, 0.5);
	/// let expected = cvmath::Transform3(1.0, 0.0, 0.0, 4.0, 0.0, 1.0, 0.0, 5.0, 0.0, 0.0, 1.0, 6.0);
	/// assert_eq!(expected, value);
	/// ```
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
	/// Solves a linear system of equations represented by the matrix.
	///
	/// Interprets the affine transform rows as the system:
	///
	/// - _a_<sub>11</sub> **x** + _a_<sub>12</sub> **y** + _a_<sub>13</sub> **z** + _a_<sub>14</sub> = 0
	/// - _a_<sub>21</sub> **x** + _a_<sub>22</sub> **y** + _a_<sub>23</sub> **z** + _a_<sub>24</sub> = 0
	/// - _a_<sub>31</sub> **x** + _a_<sub>32</sub> **y** + _a_<sub>33</sub> **z** + _a_<sub>34</sub> = 0
	///
	/// Equivalently, this finds the vector `v` such that `self * v == Vec3::ZERO`.
	///
	/// Returns `None` if the determinant is zero (no unique solution).
	///
	/// ```
	/// let mat = cvmath::Transform3(
	/// 	1.0,  2.0, 1.0, -9.0,
	/// 	2.0, -1.0, 3.0, -4.0,
	/// 	1.0,  1.0, 1.0, -6.0,
	/// );
	/// let value = mat.solve();
	/// let expected = Some(cvmath::Vec3(2.0, 3.0, 1.0));
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn solve(self) -> Option<Vec3<T>> {
		let det = self.det();
		if det == T::ZERO {
			return None;
		}

		let inv_det = T::ONE / det;
		Some(Vec3 {
			x: (self.a12 * (self.a24 * self.a33 - self.a23 * self.a34) +
				self.a13 * (self.a22 * self.a34 - self.a24 * self.a32) +
				self.a14 * (self.a23 * self.a32 - self.a22 * self.a33)) * inv_det,
			y: (self.a11 * (self.a23 * self.a34 - self.a24 * self.a33) +
				self.a13 * (self.a24 * self.a31 - self.a21 * self.a34) +
				self.a14 * (self.a21 * self.a33 - self.a23 * self.a31)) * inv_det,
			z: (self.a11 * (self.a24 * self.a32 - self.a22 * self.a34) +
				self.a12 * (self.a21 * self.a34 - self.a24 * self.a31) +
				self.a14 * (self.a22 * self.a31 - self.a21 * self.a32)) * inv_det,
		})
	}
	/// Applies the transformation around a given origin.
	///
	/// ```
	/// let rotation = cvmath::Transform3::rotation(cvmath::Vec3::Z, cvmath::Angle::deg(90.0));
	/// let mat = rotation.around(cvmath::Vec3(2.0f64, 3.0, 4.0));
	/// let value = (mat * cvmath::Vec3(3.0, 3.0, 4.0)).cast::<f32>();
	/// let expected = cvmath::Vec3(2.0f32, 4.0, 4.0);
	/// assert_eq!(expected, value);
	/// ```
	#[inline]
	pub fn around(self, origin: Vec3<T>) -> Transform3<T> where T: Float {
		let to_origin = Transform3::translation(-origin);
		let from_origin = Transform3::translation(origin);
		from_origin * self * to_origin
	}
}

//----------------------------------------------------------------
// Exponentiation

impl<T: Float> Transform3<T> {
	/// Raises the matrix to an integer power.
	///
	/// ```
	/// let axis = cvmath::Vec3(1.0, 2.0, 3.0).norm();
	/// let mat = cvmath::Transform3::rotation(axis, cvmath::Angle::deg(60.0));
	/// let value = mat.powi(-2).cast::<f32>();
	/// let expected = cvmath::Transform3::rotation(axis, cvmath::Angle::deg(60.0) * -2.0).cast::<f32>();
	/// assert_eq!(expected, value);
	/// ```
	pub fn powi(self, exp: i32) -> Transform3<T> {
		if exp == 0 {
			return Transform3::IDENTITY;
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
		print::print(&move |i| &self.as_array()[i], 0x34, f)?;
		f.write_str(")")
	}
}
impl<T: fmt::Debug> fmt::Debug for Transform3<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Transform3(")?;
		print::print(&move |i| print::Debug(&self.as_array()[i]), 0x34, f)?;
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

#[test]
fn test_ortho() {
	// aspect ratio 2:1, z range -2..2
	let bounds = Bounds3(
		Vec3(-10.0, -5.0, -2.0),
		Vec3(10.0, 5.0, 2.0),
	);

	let ortho = Transform3::ortho(bounds, (Hand::LH, Clip::ZO));
	dbg!(ortho);

	let p1 = Vec3(-10.0, -5.0, -2.0);
	let p2 = Vec3(10.0, 5.0, 2.0);
	let c = Vec3(0.0, 0.0, 0.0);

	#[track_caller]
	fn assert_approx_eq(point: Vec3f, transform: Transform3f, expected: Vec3f) {
		let value = transform * point;
		assert!((value.x - expected.x).abs() < 1e-6, "X component differs: got {}, expected {}", value.x, expected.x);
		assert!((value.y - expected.y).abs() < 1e-6, "Y component differs: got {}, expected {}", value.y, expected.y);
		assert!((value.z - expected.z).abs() < 1e-6, "Z component differs: got {}, expected {}", value.z, expected.z);
	}

	assert_approx_eq(p1, ortho, Vec3f(-1.0, -1.0, 0.0));
	assert_approx_eq(p2, ortho, Vec3f(1.0, 1.0, 1.0));
	assert_approx_eq(c, ortho, Vec3f(0.0, 0.0, 0.5));
}

#[test]
fn test_projection_reflection() {
	let plane = Plane3(Vec3(0.0f64, 0.0, 2.0), -4.0);
	let projected = Transform3::projection(plane) * Vec3(2.0, 3.0, 7.0);
	let reflected = Transform3::reflection(plane) * Vec3(2.0, 3.0, 7.0);
	assert_eq!(Vec3(2.0, 3.0, 2.0), projected);
	assert_eq!(Vec3(2.0, 3.0, -3.0), reflected);
}
