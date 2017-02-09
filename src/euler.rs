/*!
3D Euler angles.
*/

use ::angle::{Angle};
use ::vec::Vec3;
use ::num::{Scalar, Float};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Euler<A> {
	pub pitch: A,
	pub yaw: A,
	pub roll: A,
}

impl<A> Euler<A> {
	pub fn new(pitch: A, yaw: A, roll: A) -> Euler<A> {
		Euler { pitch: pitch, yaw: yaw, roll: roll }
	}
}

impl<T: Scalar + Float, A: Angle<T = T>> From<Vec3<T>> for Euler<A> {
	fn from(vec: Vec3<T>) -> Euler<A> {
		let zero = T::zero();
		let pitch;
		let yaw;
		if vec.x == zero && vec.y == zero {
			yaw = A::zero();
			pitch = if vec.z > zero { A::quarter() } else { -A::quarter() };
		}
		else {
			yaw = A::atan2(vec.y, vec.x);
			pitch = A::atan2(vec.z, vec.xy().len());
		}
		Euler { pitch: pitch, yaw: yaw, roll: A::zero() }
	}
}

fmt!(Euler { pitch, yaw, roll });
