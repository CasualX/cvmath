use super::*;

/// Capsule3 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Capsule3<T> {
	pub origin: Point3<T>,
	pub axis: Vec3<T>,
	pub radius: T,
}

/// Capsule3 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Capsule3<T>(origin: Point3<T>, axis: Vec3<T>, radius: T) -> Capsule3<T> {
	Capsule3 { origin, axis, radius }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Capsule3<T> {}

impl<T> Capsule3<T> {
	/// Constructs a new capsule.
	#[inline]
	pub const fn new(origin: Point3<T>, axis: Vec3<T>, radius: T) -> Capsule3<T> {
		Capsule3 { origin, axis, radius }
	}

	/// The second endpoint of the capsule segment.
	#[inline]
	pub fn end(self) -> Point3<T> where T: Copy + ops::Add<Output = T> {
		self.origin + self.axis
	}
	}

impl<T: Scalar> Capsule3<T> {
	/// Bounds of the capsule.
	#[inline]
	pub fn bounds(&self) -> Bounds3<T> {
		let half = Vec3::dup(self.radius);
		let mins = self.origin.min(self.end()) - half;
		let maxs = self.origin.max(self.end()) + half;
		Bounds3 { mins, maxs }
	}
}

impl<T: Scalar> Lerp for Capsule3<T> {
	type T = T;

	#[inline]
	fn lerp(self, target: Capsule3<T>, t: T) -> Capsule3<T> {
		Capsule3 {
			origin: lerp(self.origin, target.origin, t),
			axis: lerp(self.axis, target.axis, t),
			radius: lerp(self.radius, target.radius, t),
		}
	}
}

impl<T: Float> Trace3<T> for Capsule3<T> {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		let closest = self.origin + (pt - self.origin).project_sat(self.axis);
		closest.distance_sqr(pt) < self.radius * self.radius
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let end = self.end();

		if self.radius == T::ZERO {
			return Line3(self.origin, end).trace(ray);
		}

		let (axis_dir, axis_len) = self.axis.norm_len();
		if axis_len == T::ZERO {
			return Sphere(self.origin, self.radius).trace(ray);
		}

		let side_dir0 = axis_dir.any_perp();
		let side_dir1 = axis_dir.cross(side_dir0);
		let rel_origin = ray.origin - self.origin;
		let local_origin = Point3(
			rel_origin.dot(axis_dir),
			rel_origin.dot(side_dir0),
			rel_origin.dot(side_dir1),
		);
		let local_direction = Vec3(
			ray.direction.dot(axis_dir),
			ray.direction.dot(side_dir0),
			ray.direction.dot(side_dir1),
		);
		let local_ray = Ray3 { origin: local_origin, direction: local_direction, distance: ray.distance };

		let mut result: Option<Hit3<T>> = None;
		let mut update = |hit: Hit3<T>| {
			let replace = match result {
				Some(current) => hit.distance < current.distance,
				None => true,
			};
			if replace {
				result = Some(hit);
			}
		};

		let a = local_direction.y * local_direction.y + local_direction.z * local_direction.z;
		if a != T::ZERO {
			let b = local_origin.y * local_direction.y + local_origin.z * local_direction.z;
			let c = local_origin.y * local_origin.y + local_origin.z * local_origin.z - self.radius * self.radius;
			let disc = b * b - a * c;

			if disc >= T::ZERO {
				let root = disc.sqrt();
				for sign in [-T::ONE, T::ONE] {
					let distance = (-b + sign * root) / a;
					if !(distance > ray.distance.min && distance <= ray.distance.max) {
						continue;
					}

					let x = local_origin.x + local_direction.x * distance;
					if !(x >= T::ZERO && x <= axis_len) {
						continue;
					}

					let local_point = local_origin + local_direction * distance;
					let outward = Vec3(T::ZERO, local_point.y, local_point.z) * (T::ONE / self.radius);
					let world_outward = local_vec(axis_dir, side_dir0, side_dir1, outward);
					let side = if ray.direction.dot(world_outward) < T::ZERO { HitSide::Entry } else { HitSide::Exit };
					let normal = if side == HitSide::Entry { world_outward } else { -world_outward };

					update(Hit3 {
						point: ray.at(distance),
						distance,
						normal,
						index: 0,
						side,
					});
				}
			}
		}

		if let Some(hit) = local_ray.trace(&Sphere(Point3::ZERO, self.radius)) {
			update(local_hit(ray, axis_dir, side_dir0, side_dir1, hit));
		}

		if let Some(hit) = local_ray.trace(&Sphere(Point3(axis_len, T::ZERO, T::ZERO), self.radius)) {
			update(local_hit(ray, axis_dir, side_dir0, side_dir1, hit));
		}

		result
	}
}

#[inline]
fn local_vec<T: Float>(axis_dir: Vec3<T>, side_dir0: Vec3<T>, side_dir1: Vec3<T>, vec: Vec3<T>) -> Vec3<T> {
	axis_dir * vec.x + side_dir0 * vec.y + side_dir1 * vec.z
}

#[inline]
fn local_hit<T: Float>(ray: &Ray3<T>, axis_dir: Vec3<T>, side_dir0: Vec3<T>, side_dir1: Vec3<T>, hit: Hit3<T>) -> Hit3<T> {
	let normal = local_vec(axis_dir, side_dir0, side_dir1, hit.normal);
	Hit3 {
		point: ray.at(hit.distance),
		distance: hit.distance,
		normal,
		index: 0,
		side: hit.side,
	}
}