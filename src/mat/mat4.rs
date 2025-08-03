/*!
Mat4 transformation matrix.
*/

use super::*;

/// 4D transformation matrix.
///
/// Each field _a_<sub>i</sub><sub>j</sub> represents the _i_-th row and _j_-th column of the matrix.
///
/// Row-major storage with column-major semantics.
///
/// Stored in row-major order (fields appear in reading order),
/// but interpreted as column-major: each column is a transformed basis vector,
/// and matrices are applied to column vectors via `mat * vec`.
#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Mat4<T> {
	pub a11: T, pub a12: T, pub a13: T, pub a14: T,
	pub a21: T, pub a22: T, pub a23: T, pub a24: T,
	pub a31: T, pub a32: T, pub a33: T, pub a34: T,
	pub a41: T, pub a42: T, pub a43: T, pub a44: T,
}

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

impl<T: Zero + One> From<Mat3<T>> for Mat4<T> {
	#[inline]
	fn from(mat: Mat3<T>) -> Mat4<T> {
		Mat4 {
			a11: mat.a11, a12: mat.a12, a13: mat.a13,
			a21: mat.a21, a22: mat.a22, a23: mat.a23,
			a31: mat.a31, a32: mat.a32, a33: mat.a33,
			..Mat4::IDENTITY
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
			..Mat4::IDENTITY
		}
	}
}

// https://miz-ar.info/glm-notes/gtc/matrix-transform.html

impl<T: Float> Mat4<T> {
	/// Look-at matrix.
	#[inline]
	pub fn look_at(position: Vec3<T>, target: Vec3<T>, ref_up: Vec3<T>, hand: Hand) -> Mat4<T> {
		Transform3::look_at(position, target, ref_up, hand).into()
	}

	/// Orthographic projection matrix.
	///
	/// Clip and hand parameters only affect the Z coordinate.
	#[inline]
	pub fn ortho(left: T, right: T, bottom: T, top: T, near: T, far: T, (hand, clip): (Hand, Clip)) -> Mat4<T> {
		let mins = Vec3(left, bottom, near);
		let maxs = Vec3(right, top, far);
		Transform3::ortho(Bounds3 { mins, maxs }, (hand, clip)).into()
	}

	/// Orthographic projection matrix matching the framing of a perspective camera.
	///
	/// Produces an orthographic projection that preserves the screen-space framing
	/// of a perspective projection at a given `focus_depth`.
	///
	/// Useful for matching zoom level or composition when switching from
	/// perspective to orthographic rendering modes.
	///
	/// # Parameters
	/// - `focus_depth`: Distance from the camera to the subject being framed.
	/// - `fov_y`: Vertical field of view in radians (for the perspective camera).
	/// - `aspect_ratio`: Width over height of the viewport.
	/// - `near`, `far`: Depth clipping planes.
	/// - `flags`: Projection handedness and clip space settings.
	#[inline]
	pub fn ortho_perspective(focus_depth: T, fov_y: Angle<T>, aspect_ratio: T, near: T, far: T, flags: (Hand, Clip)) -> Mat4<T> {
		debug_assert!(fov_y > Angle::ZERO && fov_y < Angle::HALF, "fov_y must be in (0, 180)");
		debug_assert!(aspect_ratio > T::ZERO, "aspect_ratio must be strictly positive");
		debug_assert!(T::ZERO < near && near < far);

		let half_height = (fov_y / T::TWO).tan() * focus_depth;
		let half_width = half_height * aspect_ratio;
		Mat4::ortho(-half_width, half_width, -half_height, half_height, near, far, flags)
	}

	/// Frustum matrix.
	#[inline]
	pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T, (hand, clip): (Hand, Clip)) -> Mat4<T> {
		debug_assert!(T::ZERO < near && near < far);

		let nf = far - near;
		let np = near + near;

		let a11 = np / (right - left);
		let a13 = (right + left) / (right - left);
		let a22 = np / (top - bottom);
		let a23 = (top + bottom) / (top - bottom);

		let a33 = match clip {
			Clip::ZO => far / nf,
			Clip::NO => (far + near) / nf,
		};
		let a33 = match hand {
			Hand::LH => a33,
			Hand::RH => -a33,
		};

		let a34 = -far * near / nf;
		let a34 = match clip {
			Clip::ZO => a34,
			Clip::NO => a34 + a34,
		};

		let a43 = match hand {
			Hand::LH => T::ONE,
			Hand::RH => -T::ONE,
		};

		Mat4 { a11, a13, a22, a23, a33, a34, a43, ..Mat4::ZERO }
	}

	/// Perspective projection matrix.
	///
	/// # Parameters
	/// - `fov_y`: Vertical field of view in radians (for the perspective camera).
	/// - `aspect_ratio`: Width over height of the viewport.
	/// - `near`, `far`: Depth clipping planes.
	/// - `flags`: Projection handedness and clip space settings.
	#[inline]
	pub fn perspective(fov_y: Angle<T>, aspect_ratio: T, near: T, far: T, flags: (Hand, Clip)) -> Mat4<T> {
		debug_assert!(fov_y > Angle::ZERO && fov_y < Angle::HALF, "fov_y must be in (0, 180)");
		debug_assert!(aspect_ratio > T::ZERO, "aspect_ratio must be strictly positive");
		debug_assert!(T::ZERO < near && near < far);

		let half_fov_y = fov_y / T::TWO;
		let half_height = near * half_fov_y.tan();
		let half_width = aspect_ratio * half_height;

		let left = -half_width;
		let right = half_width;
		let bottom = -half_height;
		let top = half_height;

		Mat4::frustum(left, right, bottom, top, near, far, flags)
	}

	/// Projection matrix blending orthographic and perspective.
	///
	/// # Parameters
	/// * `blend` controls the mix between ortho (0.0) and perspective (1.0).
	/// * `focus_depth` is the distance to the subject kept stable in the transition.
	/// - `fov_y`: Vertical field of view in radians (for the perspective camera).
	/// - `aspect_ratio`: Width over height of the viewport.
	/// - `near`, `far`: Depth clipping planes.
	/// - `flags`: Projection handedness and clip space settings.
	pub fn blend_ortho_perspective(blend: T, focus_depth: T, fov_y: Angle<T>, aspect_ratio: T, near: T, far: T, flags: (Hand, Clip)) -> Mat4<T> {
		debug_assert!(fov_y > Angle::ZERO && fov_y < Angle::HALF, "fov_y must be in (0, 180)");
		debug_assert!(aspect_ratio > T::ZERO, "aspect_ratio must be strictly positive");
		debug_assert!(T::ZERO < near && near < far);
		debug_assert!(blend >= T::ZERO && blend <= T::ONE, "fraction must be in [0, 1]");

		let blend = blend.clamp(T::ZERO, T::ONE);
		let blend = blend * blend;
		let blend = blend * blend;

		let half_height = (fov_y / T::TWO).tan() * focus_depth;
		if blend <= T::EPSILON {
			let half_width = half_height * aspect_ratio;
			return Mat4::ortho(-half_width, half_width, -half_height, half_height, near, far, flags);
		}

		let dz = (T::ONE - blend) / blend;
		let adjusted_fov_y = Angle::atan(half_height / (focus_depth + dz)) * T::TWO;

		let projection = Mat4::perspective(adjusted_fov_y, aspect_ratio, near + dz, far + dz, flags);
		let trans = Vec3(T::ZERO, T::ZERO, match flags.0 { Hand::LH => dz, Hand::RH => -dz });
		let view_shift = Transform3::translate(trans);

		projection * view_shift
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
	#[inline]
	fn as_array(&self) -> &[T; 16] {
		unsafe { mem::transmute(self)}
	}
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
	pub fn det(self) -> T {
		// 2×2 subfactors
		let s0 = self.a33 * self.a44 - self.a34 * self.a43;
		let s1 = self.a32 * self.a44 - self.a34 * self.a42;
		let s2 = self.a32 * self.a43 - self.a33 * self.a42;
		let s3 = self.a31 * self.a44 - self.a34 * self.a41;
		let s4 = self.a31 * self.a43 - self.a33 * self.a41;
		let s5 = self.a31 * self.a42 - self.a32 * self.a41;

		// Cofactors for first row
		let c0 = self.a22 * s0 - self.a23 * s1 + self.a24 * s2;
		let c1 = self.a21 * s0 - self.a23 * s3 + self.a24 * s4;
		let c2 = self.a21 * s1 - self.a22 * s3 + self.a24 * s5;
		let c3 = self.a21 * s2 - self.a22 * s4 + self.a23 * s5;

		// Final determinant using expansion by first row
		self.a11 * c0 - self.a12 * c1 + self.a13 * c2 - self.a14 * c3
	}
	/// Computes the trace.
	#[inline]
	pub fn trace(self) -> T {
		self.a11 + self.a22 + self.a33 + self.a44
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
		self.a31 * self.a31 + self.a32 * self.a32 + self.a33 * self.a33 + self.a34 * self.a34 +
		self.a41 * self.a41 + self.a42 * self.a42 + self.a43 * self.a43 + self.a44 * self.a44
	}
	#[inline]
	pub fn try_invert(self) -> Option<Mat4<T>> where T: Float {
		glu_invert(&self)
	}
	/// Computes the inverse matrix.
	///
	/// Returns the zero matrix if the determinant is near zero.
	#[inline]
	pub fn inverse(self) -> Mat4<T> where T: Float {
		glu_invert(&self).unwrap_or(Mat4::ZERO)
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
	/// Linear interpolation between the matrix elements.
	#[inline]
	pub fn lerp(self, rhs: Mat4<T>, t: T) -> Mat4<T> where T: Float {
		Mat4 {
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

			a41: self.a41 + (rhs.a41 - self.a41) * t,
			a42: self.a42 + (rhs.a42 - self.a42) * t,
			a43: self.a43 + (rhs.a43 - self.a43) * t,
			a44: self.a44 + (rhs.a44 - self.a44) * t,
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

/// Inverts the matrix using the gluInvertMatrix algorithm.
/// Returns Some(inverse) or None if non-invertible.
#[inline(always)]
fn glu_invert<T: Float>(this: &Mat4<T>) -> Option<Mat4<T>> {
	let m = [
		this.a11, this.a12, this.a13, this.a14,
		this.a21, this.a22, this.a23, this.a24,
		this.a31, this.a32, this.a33, this.a34,
		this.a41, this.a42, this.a43, this.a44,
	];

	let mut inv = [T::ZERO; 16];

	inv[0] =  m[5]  * m[10] * m[15] - m[5]  * m[11] * m[14] - m[9]  * m[6]  * m[15]
	        + m[9]  * m[7]  * m[14] + m[13] * m[6]  * m[11] - m[13] * m[7]  * m[10];

	inv[4] = -m[4]  * m[10] * m[15] + m[4]  * m[11] * m[14] + m[8]  * m[6]  * m[15]
	        - m[8]  * m[7]  * m[14] - m[12] * m[6]  * m[11] + m[12] * m[7]  * m[10];

	inv[8] =  m[4]  * m[9]  * m[15] - m[4]  * m[11] * m[13] - m[8]  * m[5]  * m[15]
	        + m[8]  * m[7]  * m[13] + m[12] * m[5]  * m[11] - m[12] * m[7]  * m[9];

	inv[12] = -m[4]  * m[9]  * m[14] + m[4]  * m[10] * m[13] + m[8]  * m[5]  * m[14]
	         - m[8]  * m[6]  * m[13] - m[12] * m[5]  * m[10] + m[12] * m[6]  * m[9];

	inv[1] = -m[1]  * m[10] * m[15] + m[1]  * m[11] * m[14] + m[9]  * m[2]  * m[15]
	        - m[9]  * m[3]  * m[14] - m[13] * m[2]  * m[11] + m[13] * m[3]  * m[10];

	inv[5] =  m[0]  * m[10] * m[15] - m[0]  * m[11] * m[14] - m[8]  * m[2]  * m[15]
	        + m[8]  * m[3]  * m[14] + m[12] * m[2]  * m[11] - m[12] * m[3]  * m[10];

	inv[9] = -m[0]  * m[9]  * m[15] + m[0]  * m[11] * m[13] + m[8]  * m[1]  * m[15]
	        - m[8]  * m[3]  * m[13] - m[12] * m[1]  * m[11] + m[12] * m[3]  * m[9];

	inv[13] = m[0]  * m[9]  * m[14] - m[0]  * m[10] * m[13] - m[8]  * m[1]  * m[14]
	        + m[8]  * m[2]  * m[13] + m[12] * m[1]  * m[10] - m[12] * m[2]  * m[9];

	inv[2] =  m[1]  * m[6]  * m[15] - m[1]  * m[7]  * m[14] - m[5]  * m[2]  * m[15]
	        + m[5]  * m[3]  * m[14] + m[13] * m[2]  * m[7]  - m[13] * m[3]  * m[6];

	inv[6] = -m[0]  * m[6]  * m[15] + m[0]  * m[7]  * m[14] + m[4]  * m[2]  * m[15]
	        - m[4]  * m[3]  * m[14] - m[12] * m[2]  * m[7]  + m[12] * m[3]  * m[6];

	inv[10] = m[0]  * m[5]  * m[15] - m[0]  * m[7]  * m[13] - m[4]  * m[1]  * m[15]
	        + m[4]  * m[3]  * m[13] + m[12] * m[1]  * m[7]  - m[12] * m[3]  * m[5];

	inv[14] = -m[0]  * m[5]  * m[14] + m[0]  * m[6]  * m[13] + m[4]  * m[1]  * m[14]
	         - m[4]  * m[2]  * m[13] - m[12] * m[1]  * m[6]  + m[12] * m[2]  * m[5];

	inv[3] = -m[1]  * m[6]  * m[11] + m[1]  * m[7]  * m[10] + m[5]  * m[2]  * m[11]
	        - m[5]  * m[3]  * m[10] - m[9]  * m[2]  * m[7]  + m[9]  * m[3]  * m[6];

	inv[7] =  m[0]  * m[6]  * m[11] - m[0]  * m[7]  * m[10] - m[4]  * m[2]  * m[11]
	        + m[4]  * m[3]  * m[10] + m[8]  * m[2]  * m[7]  - m[8]  * m[3]  * m[6];

	inv[11] = -m[0]  * m[5]  * m[11] + m[0]  * m[7]  * m[9]  + m[4]  * m[1]  * m[11]
	         - m[4]  * m[3]  * m[9]  - m[8]  * m[1]  * m[7]  + m[8]  * m[3]  * m[5];

	inv[15] =  m[0]  * m[5]  * m[10] - m[0]  * m[6]  * m[9]  - m[4]  * m[1]  * m[10]
	         + m[4]  * m[2]  * m[9]  + m[8]  * m[1]  * m[6]  - m[8]  * m[2]  * m[5];

	let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
	if det == T::ZERO {
		return None; // Not invertible
	}

	let inv_det = T::ONE / det;

	Some(Mat4 {
		a11: inv[0] * inv_det, a12: inv[1] * inv_det, a13: inv[2] * inv_det, a14: inv[3] * inv_det,
		a21: inv[4] * inv_det, a22: inv[5] * inv_det, a23: inv[6] * inv_det, a24: inv[7] * inv_det,
		a31: inv[8] * inv_det, a32: inv[9] * inv_det, a33: inv[10] * inv_det, a34: inv[11] * inv_det,
		a41: inv[12] * inv_det, a42: inv[13] * inv_det, a43: inv[14] * inv_det, a44: inv[15] * inv_det,
	})
}

impl_mat_mul_scalar!(Mat4);
impl_mat_mul_vec!(Mat4, Vec4);
impl_mat_mul_mat!(Mat4);

//----------------------------------------------------------------
// Formatting

impl<T: fmt::Display> fmt::Display for Mat4<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Mat4(")?;
		print::print(&move |i| &self.as_array()[i], 0x44, f)?;
		f.write_str(")")
	}
}
impl<T: fmt::Debug> fmt::Debug for Mat4<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Mat4(")?;
		print::print(&move |i| print::Debug(&self.as_array()[i]), 0x44, f)?;
		f.write_str(")")
	}
}

//----------------------------------------------------------------
// Tests

#[test]
fn test_inverse() {
	let mut rng = urandom::seeded(42);

	for _ in 0..1000 {
		let fov_y = Angle::deg(rng.range(1.0..179.0));

		let aspect = rng.range(0.5..4.0);

		let near = rng.range(0.1..10.0);
		let far = near + rng.range(10.0..100.0);

		let hand = if rng.coin_flip() { Hand::RH } else { Hand::LH };
		let clip = if rng.coin_flip() { Clip::NO } else { Clip::ZO };

		let mat = Mat4::perspective(fov_y, aspect, near, far, (hand, clip));
		let inv = mat.inverse();

		let p = Vec4(
			rng.range(-10.0..10.0),
			rng.range(-10.0..10.0),
			rng.range(near..far),
			rng.range(0.1..10.0),
		);

		let projected = mat * p;
		let unprojected = inv * projected;
		let _identity = mat * inv;

		let error = (unprojected - p).len();
		assert!(error < 1e-6, "Failed for fov_y: {fov_y}, aspect: {aspect}, near: {near}, far: {far}, hand: {hand:?}, clip: {clip:?}, p: {p:?}, error: {error}");
	}
}

#[test]
fn test_ortho_inverse() {
	let near = 10.0;
	let far = 2000.0;
	let half_width = 400.0;
	let half_height = 300.0;
	let projection = Mat4f::ortho(
		-half_width, half_width,
		-half_height, half_height,
		near, far,
		(Hand::LH, Clip::NO)
	);

	dbg!(projection.det());

	let inv_proj = projection.inverse();
	let identity = projection * inv_proj;

	dbg!(identity);

	let error = (identity.flat_norm_sqr() - 4.0).abs();
	assert!(error.abs() < 1e-6, "Inverse projection matrix does not yield identity: error = {error}");
}
