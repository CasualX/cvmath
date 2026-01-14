/*!
Transformation matrices.
*/

use super::*;

#[macro_use]
mod macros;

mod mat2;
mod mat3;
mod mat4;

mod transform2;
mod transform3;

mod print;

pub use self::mat2::Mat2;
pub use self::mat3::Mat3;
pub use self::mat4::Mat4;

pub use self::transform2::Transform2;
pub use self::transform3::Transform3;

/// Handedness of the coordinate system.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hand {
	/// Left-handed coordinate system.
	///
	/// Mnemonic (left hand): Thumb along +X (to the right), index finger along +Y (up), middle finger along +Z.
	LH,
	/// Right-handed coordinate system.
	///
	/// Mnemonic (right hand): Thumb along +X (to the right), index finger along +Y (up), middle finger along +Z.
	RH,
}

/// Clip range.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Clip {
	/// Clip range zero to one.
	///
	/// Used by Direct3D, Vulkan, Metal.
	ZO,
	/// Clip range negative to one.
	///
	/// Used by OpenGL, OpenGL ES, WebGL.
	NO,
}

specialized_type!(Mat2, Mat2f, f32, a11, a12, a21, a22);
specialized_type!(Mat3, Mat3f, f32, a11, a12, a13, a21, a22, a23, a31, a32, a33);
specialized_type!(Mat4, Mat4f, f32, a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34, a41, a42, a43, a44);
specialized_type!(Transform2, Transform2f, f32, a11, a12, a13, a21, a22, a23);
specialized_type!(Transform3, Transform3f, f32, a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34);

specialized_type!(Mat2, Mat2d, f64, a11, a12, a21, a22);
specialized_type!(Mat3, Mat3d, f64, a11, a12, a13, a21, a22, a23, a31, a32, a33);
specialized_type!(Mat4, Mat4d, f64, a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34, a41, a42, a43, a44);
specialized_type!(Transform2, Transform2d, f64, a11, a12, a13, a21, a22, a23);
specialized_type!(Transform3, Transform3d, f64, a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34);
