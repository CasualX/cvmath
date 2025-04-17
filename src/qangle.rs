use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct QAngle {
	pub pitch: Deg<f32>,
	pub yaw: Deg<f32>,
	pub roll: Deg<f32>,
}

#[allow(non_snake_case)]
#[inline]
pub const fn QAngle(pitch: Deg<f32>, yaw: Deg<f32>, roll: Deg<f32>) -> QAngle {
	QAngle { pitch, yaw, roll }
}

#[cfg(feature = "dataview")]
unsafe impl dataview::Pod for QAngle {}

impl QAngle {
	pub fn from_vec(vec: Vec3<f32>) -> QAngle {
		let pitch;
		let yaw;
		if vec.x == 0.0 && vec.y == 0.0 {
			yaw = 0.0;
			pitch = if vec.z > 0.0 { -90.0 } else { 90.0 };
		}
		else {
			yaw = f32::atan2(vec.y, vec.x).to_degrees();
			pitch = f32::atan2(-vec.z, vec.xy().len()).to_degrees();
		}
		QAngle { pitch: Deg(pitch), yaw: Deg(yaw), roll: Deg(0.0) }
	}
	pub fn from_vecs(_forward: Vec3<f32>, _up: Vec3<f32>) -> QAngle {
		unimplemented!()
	}
	pub fn to_vec(self) -> Vec3<f32> {
		let (sp, cp) = self.pitch.sin_cos();
		let (sy, cy) = self.yaw.sin_cos();
		Vec3 {
			x: cp * cy,
			y: cp * sy,
			z: -sp,
		}
	}
	pub fn to_vecs(self) -> (Vec3<f32>, Vec3<f32>, Vec3<f32>) {
		let (sp, cp) = self.pitch.sin_cos();
		let (sy, cy) = self.yaw.sin_cos();
		let (sr, cr) = self.roll.sin_cos();
		(
			Vec3 {
				x: cp * cy,
				y: cp * sy,
				z: -sp,
			},
			Vec3 {
				x: sr * sp * cy + cr * sy,
				y: sr * sp * sy - cr * cy,
				z: -(sr * cp),
			},
			Vec3 {
				x: cr * sp * cy + sr * sy,
				y: cr * sp * sy - sr * cy,
				z: -(cr * cp),
			},
		)
	}
	#[inline]
	pub fn normalize(mut self) -> QAngle {
		while self.pitch.value <= -180.0 { self.pitch.value += 360.0; }
		while self.pitch.value > 180.0 { self.pitch.value -= 360.0; }
		while self.yaw.value <= -180.0 { self.yaw.value += 360.0; }
		while self.yaw.value > 180.0 { self.yaw.value -= 360.0; }
		self
	}
	#[inline]
	pub fn diff(self, rhs: QAngle) -> f32 {
		let delta = (self - rhs).normalize();
		(delta.pitch.value * delta.pitch.value + delta.yaw.value * delta.yaw.value).sqrt()
	}
}

impl From<[f32; 3]> for QAngle {
	#[inline]
	fn from([pitch, yaw, roll]: [f32; 3]) -> QAngle {
		QAngle { pitch: Deg(pitch), yaw: Deg(yaw), roll: Deg(roll) }
	}
}
impl From<QAngle> for [f32; 3] {
	#[inline]
	fn from(QAngle { pitch, yaw, roll }: QAngle) -> [f32; 3] {
		[pitch.value, yaw.value, roll.value]
	}
}

impl ops::Add for QAngle {
	type Output = QAngle;
	#[inline]
	fn add(self, other: QAngle) -> QAngle {
		QAngle {
			pitch: self.pitch + other.pitch,
			yaw: self.yaw + other.yaw,
			roll: self.roll + other.roll,
		}
	}
}
impl ops::Sub for QAngle {
	type Output = QAngle;
	#[inline]
	fn sub(self, other: QAngle) -> QAngle {
		QAngle {
			pitch: self.pitch - other.pitch,
			yaw: self.yaw - other.yaw,
			roll: self.roll - other.roll,
		}
	}
}
impl ops::AddAssign for QAngle {
	#[inline]
	fn add_assign(&mut self, other: QAngle) {
		self.pitch += other.pitch;
		self.yaw += other.yaw;
		self.roll += other.roll;
	}
}
impl ops::SubAssign for QAngle {
	#[inline]
	fn sub_assign(&mut self, other: QAngle) {
		self.pitch -= other.pitch;
		self.yaw -= other.yaw;
		self.roll -= other.roll;
	}
}
