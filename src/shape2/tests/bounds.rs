use super::*;

#[test]
fn test_trace_bounds() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		let x1 = rng.range(-100.0..100.0).round();
		let y1 = rng.range(-100.0..100.0).round();
		let x2 = rng.range(-100.0..100.0).round();
		let y2 = rng.range(-100.0..100.0).round();

		if x1 == x2 || y1 == y2 {
			continue;
		}

		let bounds = Bounds2(Point2(x1, y1), Point2(x2, y2)).norm();

		let tx = rng.range(bounds.mins.x..bounds.maxs.x);
		let ty = rng.range(bounds.mins.y..bounds.maxs.y);
		let target = Point2(tx, ty);

		let (vy, vx) = Angle::deg(rng.range(0.0..360.0)).sin_cos();
		let origin = (Vec2(vx, vy) * rng.range(200.0..1000.0)).round();

		let ray = Ray2 { origin, direction: target - origin, distance: 2000.0 };

		let hit = ray.trace(&bounds);

		if hit.is_none() {
			ray.trace(&bounds);
		}

		assert!(hit.is_some(), "Ray should hit the bounds: {:?} -> {:?}", ray, bounds);
	}
}
