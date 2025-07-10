use super::*;

/// Rotation vector.
///
/// A 3D rotation represented as an axis-angle vector.
/// The direction is the rotation axis, and the length is the angle in radians.
#[derive(Copy, Clone, Debug, Default, PartialEq, Hash)]
#[repr(transparent)]
pub struct RotationVector<T> {
	pub v: Vec3<T>,
}

/// Rotation vector constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn RotationVector<T>(v: Vec3<T>) -> RotationVector<T> {
	RotationVector { v }
}

impl<T: Float> RotationVector<T> {
	#[inline]
	pub fn mat3(self) -> Mat3<T> {
		let (axis, radians) = self.v.normalize_len();
		Mat3::rotate(axis, Angle { radians })
	}
	#[inline]
	pub fn quat(self) -> Quat<T> {
		let (axis, radians) = self.v.normalize_len();
		Quat::from_axis_angle(axis, Angle { radians })
	}
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for RotationVector<T> {}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for RotationVector<T> where Vec3<T>: serde::Serialize {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.v.serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for RotationVector<T> where Vec3<T>: serde::Deserialize<'de> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let v = Vec3::<T>::deserialize(deserializer)?;
		Ok(RotationVector { v })
	}
}
