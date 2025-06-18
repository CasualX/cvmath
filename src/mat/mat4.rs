/*!
Mat4 transformation matrix.
*/

use super::*;

/// 4D transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mat4<T> {
	pub a11: T, pub a12: T, pub a13: T, pub a14: T,
	pub a21: T, pub a22: T, pub a23: T, pub a24: T,
	pub a31: T, pub a32: T, pub a33: T, pub a34: T,
	pub a41: T, pub a42: T, pub a43: T, pub a44: T,
}

// pub struct Mat4<T> {
// 	pub a11: T, pub a21: T, pub a31: T, pub a41: T,
// 	pub a12: T, pub a22: T, pub a32: T, pub a42: T,
// 	pub a13: T, pub a23: T, pub a33: T, pub a43: T,
// 	pub a14: T, pub a24: T, pub a34: T, pub a44: T,
// }

/// Mat4 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Mat4<T>(
	a11: T, a12: T, a13: T, a14: T,
	a21: T, a22: T, a23: T, a24: T,
	a31: T, a32: T, a33: T, a34: T,
	a41: T, a42: T, a43: T, a44: T,
) -> Mat4<T> {
	Mat4 { a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34, a41, a42, a43, a44 }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Mat4<T> {}

//----------------------------------------------------------------
// Constructors

impl<T> Mat4<T> {
	/// Constructs a new matrix from components.
	#[inline]
	pub const fn new(
		a11: T, a12: T, a13: T, a14: T,
		a21: T, a22: T, a23: T, a24: T,
		a31: T, a32: T, a33: T, a34: T,
		a41: T, a42: T, a43: T, a44: T,
	) -> Mat4<T> {
		Mat4 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
			a41, a42, a43, a44,
		}
	}
}
impl<T: Zero> Mat4<T> {
	/// Zero matrix.
	pub const ZERO: Mat4<T> = Mat4 {
		a11: T::ZERO, a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
		a21: T::ZERO, a22: T::ZERO, a23: T::ZERO, a24: T::ZERO,
		a31: T::ZERO, a32: T::ZERO, a33: T::ZERO, a34: T::ZERO,
		a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ZERO,
	};
}
impl<T: Zero + One> Mat4<T> {
	/// Identity matrix.
	pub const IDENTITY: Mat4<T> = Mat4 {
		a11: T::ONE,  a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
		a21: T::ZERO, a22: T::ONE,  a23: T::ZERO, a24: T::ZERO,
		a31: T::ZERO, a32: T::ZERO, a33: T::ONE,  a34: T::ZERO,
		a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
	};
}
impl<T: Scalar> Mat4<T> {
	/// Translation matrix.
	#[inline]
	pub fn translate(trans: impl Into<Vec3<T>>) -> Mat4<T> {
		let trans = trans.into();
		Mat4 {
			a11: T::ONE, a12: T::ZERO, a13: T::ZERO, a14: trans.x,
			a21: T::ZERO, a22: T::ONE, a23: T::ZERO, a24: trans.y,
			a31: T::ZERO, a32: T::ZERO, a33: T::ONE, a34: trans.z,
			a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
		}
	}
	/// Scaling matrix.
	#[inline]
	pub fn scale(scale: impl Into<Vec3<T>>) -> Mat4<T> {
		let scale = scale.into();
		Mat4 {
			a11: scale.x, a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
			a21: T::ZERO, a22: scale.y, a23: T::ZERO, a24: T::ZERO,
			a31: T::ZERO, a32: T::ZERO, a33: scale.z, a34: T::ZERO,
			a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
		}
	}
	/// Rotation matrix around an axis.
	#[inline]
	pub fn rotate(angle: impl Angle<T = T>, axis: Vec3<T>) -> Mat4<T> where T: Float {
		let (s, c) = angle.sin_cos();
		let Vec3 { x, y, z } = axis;
		let t = T::ONE - c;
		Mat4 {
			a11: t * x * x + c, a12: t * x * y - s * z, a13: t * x * z + s * y, a14: T::ZERO,
			a21: t * x * y + s * z, a22: t * y * y + c, a23: t * y * z - s * x, a24: T::ZERO,
			a31: t * x * z - s * y, a32: t * y * z + s * x, a33: t * z * z + c, a34: T::ZERO,
			a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
		}
	}
}
impl<T: Zero + One> From<Transform3<T>> for Mat4<T> {
	#[inline]
	fn from(mat: Transform3<T>) -> Mat4<T> {
		Mat4 {
			a11: mat.a11, a12: mat.a12, a13: mat.a13, a14: mat.a14,
			a21: mat.a21, a22: mat.a22, a23: mat.a23, a24: mat.a24,
			a31: mat.a31, a32: mat.a32, a33: mat.a33, a34: mat.a34,
			a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
		}
	}
}

// https://miz-ar.info/glm-notes/gtc/matrix-transform.html

impl<T: Float> Mat4<T> {
	#[doc(hidden)]
	#[inline]
	pub fn no_to_zo(self) -> Mat4<T> {
		let conv = Mat4 {
			a11: T::ONE, a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
			a21: T::ZERO, a22: T::ONE, a23: T::ZERO, a24: T::ZERO,
			a31: T::ZERO, a32: T::ZERO, a33: T::ONE / (T::ONE + T::ONE), a34: T::ONE / (T::ONE + T::ONE),
			a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
		};
		conv * self
	}

	#[doc(hidden)]
	#[inline]
	pub fn zo_to_no(self) -> Mat4<T> {
		let half = T::ONE / (T::ONE + T::ONE);
		let conv = Mat4 {
			a11: T::ONE, a12: T::ZERO, a13: T::ZERO, a14: T::ZERO,
			a21: T::ZERO, a22: T::ONE, a23: T::ZERO, a24: T::ZERO,
			a31: T::ZERO, a32: T::ZERO, a33: half, a34: half,
			a41: T::ZERO, a42: T::ZERO, a43: T::ZERO, a44: T::ONE,
		}.inverse();
		conv * self
	}

	/// Look-at matrix.
	#[inline]
	pub fn look_at(eye: Vec3<T>, target: Vec3<T>, up: Vec3<T>, hand: Hand) -> Mat4<T> {
		let forward = (target - eye).normalize();
		let side = up.cross(forward).normalize();
		let up = forward.cross(side);

		let Vec3 { x: a11, y: a12, z: a13 } = side;
		let Vec3 { x: a21, y: a22, z: a23 } = up;
		let Vec3 { x: a31, y: a32, z: a33 } = match hand { Hand::LH => forward, Hand::RH => -forward };
		let (a14, a24, a34) = (-side.dot(eye), -up.dot(eye), forward.dot(eye));
		let a34 = match hand { Hand::LH => -a34, Hand::RH => a34 };
		Mat4 { a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34, a44: T::ONE, ..Mat4::ZERO }
	}

	/// Frustum matrix.
	#[inline]
	pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T, flags: (Hand, Clip)) -> Mat4<T> {
		debug_assert!(T::ZERO < near && near < far);

		let (hand, clip) = flags;

		let a11 = (near + near) / (right - left);
		let a13 = (right + left) / (right - left);
		let a22 = (near + near) / (top - bottom);
		let a23 = (top + bottom) / (top - bottom);

		let a33 = match clip { Clip::ZO => far, Clip::NO => far + near } / (far - near);
		let a33 = match hand { Hand::LH => a33, Hand::RH => -a33 };

		let a34 = -far * near / (far - near);
		let a34 = match clip { Clip::ZO => a34, Clip::NO => a34 + a34 };

		let a43 = match hand { Hand::LH => T::ONE, Hand::RH => -T::ONE };

		Mat4 { a11, a13, a22, a23, a33, a34, a43, ..Mat4::ZERO }
	}

	/// Orthographic 2D matrix.
	#[inline]
	pub fn ortho_2d(left: T, right: T, bottom: T, top: T) -> Mat4<T> {
		let two = T::ONE + T::ONE;

		let a11 = two / (right - left);
		let a14 = -(right + left) / (right - left);
		let a22 = two / (top - bottom);
		let a24 = -(top + bottom) / (top - bottom);

		Mat4 { a11, a14, a22, a24, a33: -T::ONE, a44: T::ONE, ..Mat4::ZERO }
	}

	/// Orthographic 3D matrix.
	#[inline]
	pub fn ortho_3d(left: T, right: T, bottom: T, top: T, near: T, far: T, (hand, clip): (Hand, Clip)) -> Mat4<T> {
		debug_assert!(T::ZERO < near && near < far);

		let two = T::ONE + T::ONE;
		let a11 = two / (right - left);
		let a14 = -(right + left) / (right - left);
		let a22 = two / (top - bottom);
		let a24 = -(top + bottom) / (top - bottom);
		let a33 = match clip { Clip::ZO => T::ONE, Clip::NO => two } / (far - near);
		let a33 = match hand { Hand::LH => a33, Hand::RH => -a33 };
		let a34 = -match clip { Clip::ZO => near, Clip::NO => far + near } / (far - near);

		Mat4 { a11, a14, a22, a24, a33, a34, a44: T::ONE, ..Mat4::ZERO }
	}

	/// Perspective matrix.
	#[inline]
	pub fn perspective(fovy: impl Angle<T = T>, aspect: T, near: T, far: T, flags: (Hand, Clip)) -> Mat4<T> {
		// debug_assert!(fovy > T::zero() && fovy < T::half());
		debug_assert!(aspect > T::ZERO);
		debug_assert!(T::ZERO < near && near < far);

		let two = T::ONE + T::ONE;
		let h = two * near * (fovy / (T::ONE + T::ONE)).tan();
		let w = aspect * h;

		let left = -w / two;
		let right = w / two;
		let bottom = -h / two;
		let top = h / two;
		Mat4::frustum(left, right, bottom, top, near, far, flags)
	}

	/// Perspective FOV matrix.
	#[inline]
	pub fn perspective_fov(fovy: impl Angle<T = T>, width: T, height: T, near: T, far: T, flags: (Hand, Clip)) -> Mat4<T> {
		debug_assert!(width > T::ZERO && height > T::ZERO);
		Mat4::perspective(fovy, width / height, near, far, flags)
	}

	/// Screen coordinate matrix.
	#[inline]
	pub fn screen(screen: Bounds2<T>) -> Mat4<T> {
		let half = T::ONE / (T::ONE + T::ONE);
		Mat4::translate((screen.left(), screen.top(), T::ZERO)) * Mat4::scale((screen.width(), screen.height(), T::ONE)) * Mat4::translate((half, half, T::ZERO)) * Mat4::scale((half, -half, T::ONE))
	}
}

//----------------------------------------------------------------
// Conversions

impl<T> Mat4<T> {
	/// Converts to a Mat3 matrix.
	#[inline]
	pub fn mat3(self) -> Mat3<T> {
		Mat3 {
			a11: self.a11, a12: self.a12, a13: self.a13,
			a21: self.a21, a22: self.a22, a23: self.a23,
			a31: self.a31, a32: self.a32, a33: self.a33,
		}
	}
}

impl<T> Mat4<T> {
	/// Imports the matrix from a row-major layout.
	#[inline]
	pub fn from_row_major(mat: [[T; 4]; 4]) -> Mat4<T> {
		let [[a11, a12, a13, a14], [a21, a22, a23, a24], [a31, a32, a33, a34], [a41, a42, a43, a44]] = mat;
		Mat4 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
			a41, a42, a43, a44,
		}
	}
	/// Imports the matrix from a column-major layout.
	#[inline]
	pub fn from_column_major(mat: [[T; 4]; 4]) -> Mat4<T> {
		let [[a11, a21, a31, a41], [a12, a22, a32, a42], [a13, a23, a33, a43], [a14, a24, a34, a44]] = mat;
		Mat4 {
			a11, a12, a13, a14,
			a21, a22, a23, a24,
			a31, a32, a33, a34,
			a41, a42, a43, a44,
		}
	}
	/// Exports the matrix as a row-major array.
	#[inline]
	pub fn into_row_major(self) -> [[T; 4]; 4] {
		[
			[self.a11, self.a12, self.a13, self.a14],
			[self.a21, self.a22, self.a23, self.a24],
			[self.a31, self.a32, self.a33, self.a34],
			[self.a41, self.a42, self.a43, self.a44],
		]
	}
	/// Exports the matrix as a column-major array.
	#[inline]
	pub fn into_column_major(self) -> [[T; 4]; 4] {
		[
			[self.a11, self.a21, self.a31, self.a41],
			[self.a12, self.a22, self.a32, self.a42],
			[self.a13, self.a23, self.a33, self.a43],
			[self.a14, self.a24, self.a34, self.a44],
		]
	}
}

//----------------------------------------------------------------
// Decomposition

impl<T> Mat4<T> {
	/// Composes the matrix from basis vectors.
	#[inline]
	pub fn compose(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Mat4<T> {
		Mat4 {
			a11: x.x, a12: y.x, a13: z.x, a14: w.x,
			a21: x.y, a22: y.y, a23: z.y, a24: w.y,
			a31: x.z, a32: y.z, a33: z.z, a34: w.z,
			a41: x.w, a42: y.w, a43: z.w, a44: w.w,
		}
	}
	/// Gets the transformed X basis vector.
	#[inline]
	pub fn x(self) -> Vec4<T> {
		Vec4 { x: self.a11, y: self.a21, z: self.a31, w: self.a41 }
	}
	/// Gets the transformed Y basis vector.
	#[inline]
	pub fn y(self) -> Vec4<T> {
		Vec4 { x: self.a12, y: self.a22, z: self.a32, w: self.a42 }
	}
	/// Gets the transformed Z basis vector.
	#[inline]
	pub fn z(self) -> Vec4<T> {
		Vec4 { x: self.a13, y: self.a23, z: self.a33, w: self.a43 }
	}
	/// Gets the transformed W basis vector.
	#[inline]
	pub fn w(self) -> Vec4<T> {
		Vec4 { x: self.a14, y: self.a24, z: self.a34, w: self.a44 }
	}
}

//----------------------------------------------------------------
// Operations

impl<T: Scalar> Mat4<T> {
	/// Computes the determinant.
	#[inline]
	pub fn determinant(self) -> T {
		self.a11 * (self.a22 * (self.a33 * self.a44 - self.a34 * self.a43) - self.a23 * (self.a32 * self.a44 - self.a34 * self.a42) + self.a24 * (self.a32 * self.a43 - self.a33 * self.a42)) -
		self.a12 * (self.a21 * (self.a33 * self.a44 - self.a34 * self.a43) - self.a23 * (self.a31 * self.a44 - self.a34 * self.a41) + self.a24 * (self.a31 * self.a43 - self.a33 * self.a41)) +
		self.a13 * (self.a21 * (self.a32 * self.a44 - self.a34 * self.a42) - self.a22 * (self.a31 * self.a44 - self.a34 * self.a41) + self.a24 * (self.a31 * self.a42 - self.a32 * self.a41)) -
		self.a14 * (self.a21 * (self.a32 * self.a43 - self.a33 * self.a42) - self.a22 * (self.a31 * self.a43 - self.a33 * self.a41) + self.a23 * (self.a31 * self.a42 - self.a32 * self.a41))
	}
	/// Computes the trace.
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22 + self.a33 + self.a44
	}
	/// Computes the inverse matrix.
	#[inline]
	pub fn inverse(self) -> Mat4<T> {
		let det = self.determinant();
		if det != T::ZERO {
			self.adjugate() * (T::ONE / det)
		}
		else {
			self
		}
	}
	/// Returns the transposed matrix.
	#[inline]
	pub fn transpose(self) -> Mat4<T> {
		Mat4 {
			a11: self.a11, a12: self.a21, a13: self.a31, a14: self.a41,
			a21: self.a12, a22: self.a22, a23: self.a32, a24: self.a42,
			a31: self.a13, a32: self.a23, a33: self.a33, a34: self.a43,
			a41: self.a14, a42: self.a24, a43: self.a34, a44: self.a44,
		}
	}
	/// Computes the adjugate matrix.
	#[inline]
	pub fn adjugate(self) -> Mat4<T> {
		Mat4 {
			a11: self.a22 * (self.a33 * self.a44 - self.a34 * self.a43) - self.a23 * (self.a32 * self.a44 - self.a34 * self.a42) + self.a24 * (self.a32 * self.a43 - self.a33 * self.a42),
			a12: -(self.a21 * (self.a33 * self.a44 - self.a34 * self.a43) - self.a23 * (self.a31 * self.a44 - self.a34 * self.a41) + self.a24 * (self.a31 * self.a43 - self.a33 * self.a41)),
			a13: self.a21 * (self.a32 * self.a44 - self.a34 * self.a42) - self.a22 * (self.a31 * self.a44 - self.a34 * self.a41) + self.a24 * (self.a31 * self.a42 - self.a32 * self.a41),
			a14: -(self.a21 * (self.a32 * self.a43 - self.a33 * self.a42) - self.a22 * (self.a31 * self.a43 - self.a33 * self.a41) + self.a23 * (self.a31 * self.a42 - self.a32 * self.a41)),
			a21: -(self.a12 * (self.a33 * self.a44 - self.a34 * self.a43) - self.a13 * (self.a32 * self.a44 - self.a34 * self.a42) + self.a14 * (self.a32 * self.a43 - self.a33 * self.a42)),
			a22: self.a11 * (self.a33 * self.a44 - self.a34 * self.a43) - self.a13 * (self.a31 * self.a44 - self.a34 * self.a41) + self.a14 * (self.a31 * self.a43 - self.a33 * self.a41),
			a23: -(self.a11 * (self.a32 * self.a44 - self.a34 * self.a42) - self.a12 * (self.a31 * self.a44 - self.a34 * self.a41) + self.a14 * (self.a31 * self.a42 - self.a32 * self.a41)),
			a24: self.a11 * (self.a32 * self.a43 - self.a33 * self.a42) - self.a12 * (self.a31 * self.a43 - self.a33 * self.a41) + self.a13 * (self.a31 * self.a42 - self.a32 * self.a41),
			a31: self.a12 * (self.a23 * self.a44 - self.a24 * self.a43) - self.a13 * (self.a22 * self.a44 - self.a24 * self.a42) + self.a14 * (self.a22 * self.a43 - self.a23 * self.a42),
			a32: -(self.a11 * (self.a23 * self.a44 - self.a24 * self.a43) - self.a13 * (self.a21 * self.a44 - self.a24 * self.a41) + self.a14 * (self.a21 * self.a43 - self.a23 * self.a41)),
			a33: self.a11 * (self.a22 * self.a44 - self.a24 * self.a42) - self.a12 * (self.a21 * self.a44 - self.a24 * self.a41) + self.a14 * (self.a21 * self.a42 - self.a22 * self.a41),
			a34: -(self.a11 * (self.a22 * self.a43 - self.a23 * self.a42) - self.a12 * (self.a21 * self.a43 - self.a23 * self.a41) + self.a13 * (self.a21 * self.a42 - self.a22 * self.a41)),
			a41: -(self.a12 * (self.a23 * self.a34 - self.a24 * self.a33) - self.a13 * (self.a22 * self.a34 - self.a24 * self.a32) + self.a14 * (self.a22 * self.a33 - self.a23 * self.a32)),
			a42: self.a11 * (self.a23 * self.a34 - self.a24 * self.a33) - self.a13 * (self.a21 * self.a34 - self.a24 * self.a31) + self.a14 * (self.a21 * self.a33 - self.a23 * self.a31),
			a43: -(self.a11 * (self.a22 * self.a34 - self.a24 * self.a32) - self.a12 * (self.a21 * self.a34 - self.a24 * self.a31) + self.a14 * (self.a21 * self.a32 - self.a22 * self.a31)),
			a44: self.a11 * (self.a22 * self.a33 - self.a23 * self.a32) - self.a12 * (self.a21 * self.a33 - self.a23 * self.a31) + self.a13 * (self.a21 * self.a32 - self.a22 * self.a31),
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for Mat4<T> {
	type Output = Mat4<T>;
	#[inline]
	fn mul(self, rhs: T) -> Mat4<T> {
		Mat4 {
			a11: self.a11 * rhs, a12: self.a12 * rhs, a13: self.a13 * rhs, a14: self.a14 * rhs,
			a21: self.a21 * rhs, a22: self.a22 * rhs, a23: self.a23 * rhs, a24: self.a24 * rhs,
			a31: self.a31 * rhs, a32: self.a32 * rhs, a33: self.a33 * rhs, a34: self.a34 * rhs,
			a41: self.a41 * rhs, a42: self.a42 * rhs, a43: self.a43 * rhs, a44: self.a44 * rhs,
		}
	}
}
impl<T: Copy + ops::MulAssign> ops::MulAssign<T> for Mat4<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.a11 *= rhs; self.a12 *= rhs; self.a13 *= rhs; self.a14 *= rhs;
		self.a21 *= rhs; self.a22 *= rhs; self.a23 *= rhs; self.a24 *= rhs;
		self.a31 *= rhs; self.a32 *= rhs; self.a33 *= rhs; self.a34 *= rhs;
		self.a41 *= rhs; self.a42 *= rhs; self.a43 *= rhs; self.a44 *= rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Vec4<T>> for Mat4<T> {
	type Output = Vec4<T>;
	#[inline]
	fn mul(self, rhs: Vec4<T>) -> Vec4<T> {
		Vec4 {
			x: self.a11 * rhs.x + self.a12 * rhs.y + self.a13 * rhs.z + self.a14 * rhs.w,
			y: self.a21 * rhs.x + self.a22 * rhs.y + self.a23 * rhs.z + self.a24 * rhs.w,
			z: self.a31 * rhs.x + self.a32 * rhs.y + self.a33 * rhs.z + self.a34 * rhs.w,
			w: self.a41 * rhs.x + self.a42 * rhs.y + self.a43 * rhs.z + self.a44 * rhs.w,
		}
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Mat4<T>> for Mat4<T> {
	type Output = Mat4<T>;
	#[inline]
	fn mul(self, rhs: Mat4<T>) -> Mat4<T> {
		Mat4 {
			a11: self.a11 * rhs.a11 + self.a12 * rhs.a21 + self.a13 * rhs.a31 + self.a14 * rhs.a41,
			a12: self.a11 * rhs.a12 + self.a12 * rhs.a22 + self.a13 * rhs.a32 + self.a14 * rhs.a42,
			a13: self.a11 * rhs.a13 + self.a12 * rhs.a23 + self.a13 * rhs.a33 + self.a14 * rhs.a43,
			a14: self.a11 * rhs.a14 + self.a12 * rhs.a24 + self.a13 * rhs.a34 + self.a14 * rhs.a44,

			a21: self.a21 * rhs.a11 + self.a22 * rhs.a21 + self.a23 * rhs.a31 + self.a24 * rhs.a41,
			a22: self.a21 * rhs.a12 + self.a22 * rhs.a22 + self.a23 * rhs.a32 + self.a24 * rhs.a42,
			a23: self.a21 * rhs.a13 + self.a22 * rhs.a23 + self.a23 * rhs.a33 + self.a24 * rhs.a43,
			a24: self.a21 * rhs.a14 + self.a22 * rhs.a24 + self.a23 * rhs.a34 + self.a24 * rhs.a44,

			a31: self.a31 * rhs.a11 + self.a32 * rhs.a21 + self.a33 * rhs.a31 + self.a34 * rhs.a41,
			a32: self.a31 * rhs.a12 + self.a32 * rhs.a22 + self.a33 * rhs.a32 + self.a34 * rhs.a42,
			a33: self.a31 * rhs.a13 + self.a32 * rhs.a23 + self.a33 * rhs.a33 + self.a34 * rhs.a43,
			a34: self.a31 * rhs.a14 + self.a32 * rhs.a24 + self.a33 * rhs.a34 + self.a34 * rhs.a44,

			a41: self.a41 * rhs.a11 + self.a42 * rhs.a21 + self.a43 * rhs.a31 + self.a44 * rhs.a41,
			a42: self.a41 * rhs.a12 + self.a42 * rhs.a22 + self.a43 * rhs.a32 + self.a44 * rhs.a42,
			a43: self.a41 * rhs.a13 + self.a42 * rhs.a23 + self.a43 * rhs.a33 + self.a44 * rhs.a43,
			a44: self.a41 * rhs.a14 + self.a42 * rhs.a24 + self.a43 * rhs.a34 + self.a44 * rhs.a44,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Mat4<T>> for Mat4<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Mat4<T>) {
		*self = *self * rhs;
	}
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul<Transform3<T>> for Mat4<T> {
	type Output = Mat4<T>;
	#[inline]
	fn mul(self, rhs: Transform3<T>) -> Mat4<T> {
		Mat4 {
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

			a41: self.a41 * rhs.a11 + self.a42 * rhs.a21 + self.a43 * rhs.a31,
			a42: self.a41 * rhs.a12 + self.a42 * rhs.a22 + self.a43 * rhs.a32,
			a43: self.a41 * rhs.a13 + self.a42 * rhs.a23 + self.a43 * rhs.a33,
			a44: self.a41 * rhs.a14 + self.a42 * rhs.a24 + self.a43 * rhs.a34 + self.a44,
		}
	}
}
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> ops::MulAssign<Transform3<T>> for Mat4<T> {
	#[inline]
	fn mul_assign(&mut self, rhs: Transform3<T>) {
		*self = *self * rhs;
	}
}
