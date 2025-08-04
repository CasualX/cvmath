use super::*;

#[test]
fn test_trace_lines() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		let x = rng.range(-100.0..100.0).round();
		let y = rng.range(-100.0..100.0).round();
		let t = rng.range(0.0001..0.9999);

		if x == y {
			continue;
		}

		let start = Point2(x, y);
		let end = Point2(-x, -y);
		let line = Line2(start, end);

		let target = start.lerp(end, t);
		let origin = Point2(-1001.0, -203.123901);
		let (direction, distance) = (target - origin).norm_len();
		let ray = Ray2(origin, direction, distance + 0.1);

		assert!(ray.trace(&line).is_some(), "Ray should hit the line: {:?} -> {:?}", ray, line);
	}
}
