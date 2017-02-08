/*!
3D Euler angles.
*/

// use ::angle::Angle;

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
