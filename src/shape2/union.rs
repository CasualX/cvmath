use super::*;

impl<T: Float, S1: Trace2<T>, S2: Trace2<T>> Trace2<T> for Union<S1, S2> {
	fn inside(&self, pt: Point2<T>) -> bool {
		self.shape1.inside(pt) || self.shape2.inside(pt)
	}

	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		if self.shape1.inside(ray.origin) {
			let hit1 = self.shape1.trace(ray)?;
			let ray2 = ray.step(hit1.distance);
			if self.shape2.inside(ray2.origin) {
				let hit2 = self.shape2.trace(&ray2);
				if hit2.is_some() {
					return hit2;
				}
			}
			Some(hit1)
		}
		else if self.shape2.inside(ray.origin) {
			let hit1 = self.shape2.trace(ray)?;
			let ray2 = ray.step(hit1.distance);
			if self.shape1.inside(ray2.origin) {
				let hit2 = self.shape1.trace(&ray2);
				if hit2.is_some() {
					return hit2;
				}
			}
			Some(hit1)
		}
		else {
			let hit1 = self.shape1.trace(ray);
			let hit2 = self.shape2.trace(ray);

			match (hit1, hit2) {
				(Some(h1), Some(h2)) => if h1.distance < h2.distance { Some(h1) } else { Some(h2) },
				(Some(h1), None) => Some(h1),
				(None, Some(h2)) => Some(h2),
				(None, None) => None,
			}
		}
	}
}
