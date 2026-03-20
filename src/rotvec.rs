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

impl<T: Float> Lerp for RotationVector<T> {
	type T = T;

	#[inline]
	fn lerp(self, other: Self, t: T) -> Self {
		RotationVector { v: lerp(self.v, other.v, t) }
	}
}

impl<T: Float> RotationVector<T> {
	#[inline]
	pub fn mat3(self) -> Mat3<T> {
		let (axis, radians) = self.v.norm_len();
		Mat3::rotation(axis, Angle { radians })
	}
	#[inline]
	pub fn quat(self) -> Quat<T> {
		let (axis, radians) = self.v.norm_len();
		Quat::from_axis_angle(axis, Angle { radians })
	}
}

impl<T: Float> From<(Vec3<T>, Angle<T>)> for RotationVector<T> {
	#[inline]
	fn from((axis, angle): (Vec3<T>, Angle<T>)) -> Self {
		let v = axis * angle.radians;
		RotationVector { v }
	}
}
impl<T: Float> From<RotationVector<T>> for (Vec3<T>, Angle<T>) {
	#[inline]
	fn from(rotvec: RotationVector<T>) -> Self {
		let (axis, radians) = rotvec.v.norm_len();
		(axis, Angle { radians })
	}
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for RotationVector<T> {}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for RotationVector<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeTupleStruct;
		let mut state = serializer.serialize_tuple_struct("RotationVector", 3)?;
		state.serialize_field(&self.v.x)?;
		state.serialize_field(&self.v.y)?;
		state.serialize_field(&self.v.z)?;
		state.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for RotationVector<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let (x, y, z) = {
			#[derive(serde::Deserialize)]
			struct RotationVector<T>(T, T, T);
			let RotationVector(x, y, z) = RotationVector::<T>::deserialize(deserializer)?;
			(x, y, z)
		};
		Ok(RotationVector { v: Vec3 { x, y, z } })
	}
}
