use super::*;

#[test]
fn test_trace_random_lines() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		let start = Point2(rng.next_f64(), rng.next_f64());
		let end = Point2(rng.next_f64(), rng.next_f64());
		let line = Line2(start, end);

		// Skip zero-length lines
		if start.distance(end) < 1e-6 {
			continue;
		}

		let t = rng.range(-0.5..1.5);
		let target = start.lerp(end, t);

		let origin = Point2(rng.next_f64(), rng.next_f64());
		let origin = origin * 10.0;

		// Skip degenerate cases
		if Triangle2::points(start, end, origin).area() < 1e-6 {
			continue;
		}

		let direction = (target - origin).norm();
		let mut ray = Ray2(origin, direction, f64::INFINITY);
		let hit = ray.trace(&line);

		if t > 0.01 && t < 0.99 {
			assert!(hit.is_some(), "Ray should hit the line: {:?} -> {:?}", ray, line);
			let hit = hit.unwrap();
			let d = line.distance(ray.at(hit.distance));
			assert!(d.abs() < 1e-4, "Hit point should be on the line: {d}");
		}
		else if t < -0.01 || t > 1.01 {
			assert!(hit.is_none(), "Ray should not hit the line: {:?} -> {:?}", ray, line);
		}

		ray.direction = -ray.direction;
		assert!(ray.trace(&line).is_none(), "Ray should not hit the line when moving away");
	}
}
