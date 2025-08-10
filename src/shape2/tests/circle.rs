use super::*;

#[test]
fn test_trace_random_circles() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		let center = Point2(rng.next_f64() * 50.0, rng.next_f64() * 50.0).round();
		let radius = rng.range(1.0..50.0).round();
		let circle = Circle { center, radius };
		let bounds = circle.bounds();

		let tx = rng.range(bounds.mins.x..bounds.maxs.x);
		let ty = rng.range(bounds.mins.y..bounds.maxs.y);
		let target = Point2(tx, ty);

		// Skip points outside the circle
		let distance = target.distance(center);
		if distance + 1e-6 >= radius {
			continue;
		}

		let (vy, vx) = Angle::deg(rng.range(0.0..360.0)).sin_cos();
		let origin = (Vec2(vx, vy) * rng.range(200.0..1000.0)).round();

		let direction = (target - origin).norm();
		let mut ray = Ray2 { origin, direction, distance: f64::INFINITY };

		// Trace towards the circle should hit
		let hit = ray.trace(&circle);
		assert!(hit.is_some(), "{ray:?} should hit {circle:?}");

		// Trace away from the circle should miss
		ray.direction = -ray.direction;
		let hit = ray.trace(&circle);
		assert!(hit.is_none(), "{ray:?} should miss {circle:?}");
	}
}
