use super::*;

/// Sphere structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Sphere<T> {
	/// The center of the sphere.
	pub center: Point3<T>,
	/// The radius of the sphere.
	pub radius: T,
}

/// Sphere constructor.
#[allow(non_snake_case)]
#[inline]
pub fn Sphere<T>(center: Point3<T>, radius: T) -> Sphere<T> {
	Sphere { center, radius }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Sphere<T> {}

impl<T> Sphere<T> {
	/// Constructs a new sphere.
	#[inline]
	pub const fn new(center: Point3<T>, radius: T) -> Sphere<T> {
		Sphere { center, radius }
	}
}

//----------------------------------------------------------------

impl<T: Float> TraceRay<T> for Sphere<T> {
	fn inside(&self, ray: &Ray<T>) -> bool {
		ray.origin.distance_sqr(self.center) < self.radius * self.radius
	}

	fn trace(&self, ray: &Ray<T>, hits: &mut [TraceHit<T>]) -> usize {
		let oc = self.center - ray.origin;
		let tc = oc.dot(ray.direction);

		let d2 = oc.dot(oc) - tc * tc;
		let r2 = self.radius * self.radius;
		let t1c2 = r2 - d2;

		if t1c2 < T::ZERO {
			return 0;
		}
		let t1c = t1c2.sqrt();

		let t1 = tc - t1c;
		let t2 = tc + t1c;

		let mut count = 0;
		if t1 >= T::ZERO {
			if count < hits.len() {
				let normal = (ray.at(t1) - self.center).normalize();
				hits[count] = TraceHit { distance: t1, normal };
			}
			count += 1;
		}
		if t2 >= T::ZERO && t1 != t2 {
			if count < hits.len() {
				let normal = (ray.at(t2) - self.center).normalize();
				hits[count] = TraceHit { distance: t2, normal };
			}
			count += 1;
		}

		return count;
	}
}
