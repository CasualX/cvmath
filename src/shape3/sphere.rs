use super::*;

/// Sphere shape.
#[derive(Copy, Clone, Debug, PartialEq)]
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
pub const fn Sphere<T>(center: Point3<T>, radius: T) -> Sphere<T> {
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

impl<T: Scalar> Sphere<T> {
	/// Linear interpolation between the shapes.
	#[inline]
	pub fn lerp(self, target: Sphere<T>, t: T) -> Sphere<T> {
		Sphere {
			center: self.center.lerp(target.center, t),
			radius: self.radius + (target.radius - self.radius) * t,
		}
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace3<T> for Sphere<T> {
	#[inline]
	fn inside(&self, pt: Point3<T>) -> bool {
		pt.distance_sqr(self.center) < self.radius * self.radius
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let oc = self.center - ray.origin;
		let tc = oc.dot(ray.direction);

		let d2 = oc.dot(oc) - tc * tc;
		let r2 = self.radius * self.radius;
		let t1c2 = r2 - d2;

		if !(t1c2 >= T::ZERO) {
			return None;
		}
		let t1c = t1c2.sqrt();

		let t1 = tc - t1c;
		let t2 = tc + t1c;

		if t1 > T::EPSILON && t1 <= ray.distance {
			let normal = (ray.at(t1) - self.center).norm();
			return Some(Hit3 { distance: t1, normal, index: 0 });
		}
		if t2 > T::EPSILON && t2 <= ray.distance {
			let normal = (ray.at(t2) - self.center).norm();
			return Some(Hit3 { distance: t2, normal, index: 0 });
		}
		return None;
	}
}
