use super::*;

/// Capsule2 shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Capsule2<T> {
	pub origin: Point2<T>,
	pub axis: Vec2<T>,
	pub radius: T,
}

/// Capsule2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Capsule2<T>(origin: Point2<T>, axis: Vec2<T>, radius: T) -> Capsule2<T> {
	Capsule2 { origin, axis, radius }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Capsule2<T> {}

impl<T> Capsule2<T> {
	/// Constructs a new capsule.
	#[inline]
	pub const fn new(origin: Point2<T>, axis: Vec2<T>, radius: T) -> Capsule2<T> {
		Capsule2 { origin, axis, radius }
	}

	/// The second endpoint of the capsule segment.
	#[inline]
	pub fn end(self) -> Point2<T> where T: Copy + ops::Add<Output = T> {
		self.origin + self.axis
	}
}

impl<T: Scalar> Capsule2<T> {
	/// Bounds of the capsule.
	#[inline]
	pub fn bounds(&self) -> Bounds2<T> {
		let half = Vec2::dup(self.radius);
		let mins = self.origin.min(self.end()) - half;
		let maxs = self.origin.max(self.end()) + half;
		Bounds2 { mins, maxs }
	}
}

impl<T: Float> Capsule2<T> {
	/// Area of the capsule.
	#[inline]
	pub fn area(&self) -> T {
		let axis_len = self.axis.len();
		T::PI * self.radius * self.radius + (T::TWO * self.radius * axis_len)
	}
}

impl<T: Scalar> Lerp for Capsule2<T> {
	type T = T;

	#[inline]
	fn lerp(self, target: Capsule2<T>, t: T) -> Capsule2<T> {
		Capsule2 {
			origin: lerp(self.origin, target.origin, t),
			axis: lerp(self.axis, target.axis, t),
			radius: lerp(self.radius, target.radius, t),
		}
	}
}

impl<T: Float> Trace2<T> for Capsule2<T> {
	#[inline]
	fn inside(&self, pt: Point2<T>) -> bool {
		let closest = self.origin + (pt - self.origin).project_sat(self.axis);
		closest.distance_sqr(pt) < self.radius * self.radius
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		let end = self.end();

		if self.radius == T::ZERO {
			return Line2(self.origin, end).trace(ray);
		}

		let (axis_dir, axis_len) = self.axis.norm_len();
		if axis_len == T::ZERO {
			return Circle(self.origin, self.radius).trace(ray);
		}

		let side_dir = axis_dir.cw();
		let rel_origin = ray.origin - self.origin;
		let local_origin = Point2(rel_origin.dot(axis_dir), rel_origin.dot(side_dir));
		let local_direction = Vec2(ray.direction.dot(axis_dir), ray.direction.dot(side_dir));
		let local_ray = Ray2 { origin: local_origin, direction: local_direction, distance: ray.distance };

		let mut result: Option<Hit2<T>> = None;

		let mut update = |hit: Hit2<T>| {
			let replace = match result {
				Some(current) => hit.distance < current.distance,
				None => true,
			};
			if replace {
				result = Some(hit);
			}
		};

		if local_direction.y != T::ZERO {
			for sign in [-T::ONE, T::ONE] {
				let distance = (self.radius * sign - local_origin.y) / local_direction.y;
				if !(distance > ray.distance.min && distance <= ray.distance.max) {
					continue;
				}

				let x = local_origin.x + local_direction.x * distance;
				if !(x >= T::ZERO && x <= axis_len) {
					continue;
				}

				let outward = Vec2(T::ZERO, sign);
				let outward_world = axis_dir * outward.x + side_dir * outward.y;
				let side = if ray.direction.dot(outward_world) < T::ZERO { HitSide::Entry } else { HitSide::Exit };
				let normal = if side == HitSide::Entry { outward_world } else { -outward_world };

				update(Hit2 {
					point: ray.at(distance),
					distance,
					normal,
					index: 0,
					side,
				});
			}
		}

		if let Some(hit) = local_ray.trace(&Circle(Point2::ZERO, self.radius)) {
			update(local_hit(ray, axis_dir, side_dir, hit));
		}

		if let Some(hit) = local_ray.trace(&Circle(Point2(axis_len, T::ZERO), self.radius)) {
			update(local_hit(ray, axis_dir, side_dir, hit));
		}

		result
	}
}

#[inline]
fn local_hit<T: Float>(ray: &Ray2<T>, axis_dir: Vec2<T>, side_dir: Vec2<T>, hit: Hit2<T>) -> Hit2<T> {
	let normal = axis_dir * hit.normal.x + side_dir * hit.normal.y;
	Hit2 {
		point: ray.at(hit.distance),
		distance: hit.distance,
		normal,
		index: 0,
		side: hit.side,
	}
}
