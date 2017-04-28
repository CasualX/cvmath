/*!
Affine 3D transformation matrix.
*/

/// Affine 3D transformation matrix.
///
/// A 3x4 row-major matrix.
#[cfg(feature = "row-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine3<T> {
	pub a11: T, pub a12: T, pub a13: T, pub a14: T,
	pub a21: T, pub a22: T, pub a23: T, pub a24: T,
	pub a31: T, pub a32: T, pub a33: T, pub a34: T,
}

/// Affine 3D transformation matrix.
///
/// A 3x4 column-major matrix.
#[cfg(feature = "column-major")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Affine2<T> {
	pub a11: T, pub a21: T, pub a31: T,
	pub a12: T, pub a22: T, pub a32: T,
	pub a13: T, pub a23: T, pub a33: T,
	pub a14: T, pub a24: T, pub a34: T,
}
