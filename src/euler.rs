/*!
3D Euler angles.
*/

use ::angle::{Angle};
use ::vec::{Vec3};
use ::num::{Zero};

type Euler<T> = Vec3<T>;

#[cfg(feature = "invert-pitch")]
macro_rules! invert { ($e:expr) => (-$e); }
#[cfg(not(feature = "invert-pitch"))]
macro_rules! invert { ($e:expr) => ($e); }

impl<A: Angle> Euler<A> {
	pub fn from_vec(vec: Vec3<A::T>) -> Euler<A> {
		let zero = A::T::zero();
		let pitch;
		let yaw;
		if vec.x == zero && vec.y == zero {
			yaw = A::zero();
			pitch = if vec.z > zero { invert!(A::quarter()) } else { -invert!(A::quarter()) };
		}
		else {
			yaw = A::atan2(vec.y, vec.x);
			pitch = A::atan2(invert!(vec.z), vec.xy().len());
		}
		Euler {
			x: pitch,
			y: yaw,
			z: A::zero(),
		}
	}
	pub fn from_vecs(_forward: Vec3<A::T>, _up: Vec3<A::T>) -> Euler<A> {
		unimplemented!()
	}
	pub fn to_vec(self) -> Vec3<A::T> {
		let (sp, cp) = self.x.sin_cos();
		let (sy, cy) = self.y.sin_cos();
		Vec3 {
			x: cp * cy,
			y: cp * sy,
			z: invert!(sp),
		}
	}
	pub fn to_vecs(self) -> (Vec3<A::T>, Vec3<A::T>, Vec3<A::T>) {
		let (sp, cp) = self.x.sin_cos();
		let (sy, cy) = self.y.sin_cos();
		let (sr, cr) = self.z.sin_cos();
		(
			Vec3 {
				x: cp * cy,
				y: cp * sy,
				z: invert!(sp),
			},
			Vec3 {
				x: sr * sp * cy + cr * sy,
				y: sr * sp * sy - cr * cy,
				z: invert!(sr * cp),
			},
			Vec3 {
				x: cr * sp * cy + sr * sy,
				y: cr * sp * sy - sr * cy,
				z: invert!(cr * cp),
			},
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::angle::{Deg};
	#[test]
	fn mul_test() {
		let qa = Euler { x: Deg(1.0), y: Deg(2.0), z: Deg(0.0) } * 2.0;
		assert_eq!(qa, Euler { x: Deg(2.0), y: Deg(4.0), z: Deg(0.0) });
	}
}
