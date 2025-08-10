use super::*;

impl<T: Float, S1: Trace3<T>, S2: Trace3<T>> Trace3<T> for Intersection<S1, S2> {
	fn inside(&self, pt: Point3<T>) -> bool {
		self.shape1.inside(pt) && self.shape2.inside(pt)
	}

	fn trace(&self, ray: &Ray3<T>) -> Option<Hit3<T>> {
		let mut inside1 = self.shape1.inside(ray.origin);
		let mut inside2 = self.shape2.inside(ray.origin);

		let mut r = *ray;
		let mut acc = T::ZERO;

		for _ in 0..64 {
			let h1 = self.shape1.trace(&r);
			let h2 = self.shape2.trace(&r);

			// If already inside the intersection, the next exit (nearest hit) is the boundary.
			if inside1 && inside2 {
				let hit = match (h1, h2) {
					(Some(a), Some(b)) => if a.distance <= b.distance { a } else { b },
					(Some(a), None) => a,
					(None, Some(b)) => b,
					(None, None) => return None,
				};
				if acc + hit.distance > ray.distance { return None; }
				return Some(Hit3 { distance: acc + hit.distance, normal: hit.normal, index: 0 });
			}

			// Otherwise, march to the next event and toggle that shape's inside flag.
			let (which, hit) = match (h1, h2) {
				(Some(a), Some(b)) => if a.distance <= b.distance { (0u8, a) } else { (1u8, b) },
				(Some(a), None) => (0, a),
				(None, Some(b)) => (1, b),
				(None, None) => return None,
			};

			if acc + hit.distance > ray.distance { return None; }

			// Toggle state at the crossing
			if which == 0 { inside1 = !inside1; } else { inside2 = !inside2; }

			acc = acc + hit.distance;

			// If we just entered the intersection, this crossing is the hit.
			if inside1 && inside2 {
				return Some(Hit3 { distance: acc, normal: hit.normal, index: 0 });
			}

			// Advance slightly to avoid re-hitting the same surface.
			r = r.step(hit.distance + T::EPSILON);
		}

		None
	}
}
