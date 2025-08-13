use super::*;

#[test]
fn test_trace_random_bounds() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		let x1 = rng.range(-100.0..100.0).round();
		let y1 = rng.range(-100.0..100.0).round();
		let x2 = rng.range(-100.0..100.0).round();
		let y2 = rng.range(-100.0..100.0).round();
		let bounds = Bounds2(Point2(x1, y1), Point2(x2, y2)).norm();

		// Skip degenerate bounds
		if bounds.area() < 1e-6 {
			continue;
		}

		let tx = rng.range(bounds.mins.x..bounds.maxs.x);
		let ty = rng.range(bounds.mins.y..bounds.maxs.y);
		let target = Point2(tx, ty);

		let (vy, vx) = Angle::deg(rng.range(0.0..360.0)).sin_cos();
		let origin = (Vec2(vx, vy) * rng.range(200.0..1000.0)).round();

		let direction = (target - origin).norm();
		let mut ray = Ray2 { origin, direction, distance: Interval(0.0, f64::INFINITY) };

		// Trace towards the bounds should hit
		let hit = ray.trace(&bounds);
		assert!(hit.is_some(), "{ray:?} should hit {bounds:?}");

		// Trace away from the bounds should miss
		ray.direction = -ray.direction;
		let hit = ray.trace(&bounds);
		assert!(hit.is_none(), "{ray:?} should miss {bounds:?}");
	}
}
