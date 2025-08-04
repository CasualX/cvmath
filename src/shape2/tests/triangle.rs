use super::*;

#[test]
fn test_trace_simple() {
	let tri = Triangle2(Point2(0.0, 0.0), Point2(1.0, 0.0), Point2(0.0, 1.0));
	let ray1 = Ray2 { origin: Point2(0.5, -1.0), direction: Vec2(0.0, 1.0), distance: 10.0 };
	let hit1 = tri.trace(&ray1).expect("Ray should hit the triangle");
	assert!(hit1.distance - 1.0 < 1e-6, "Intersection distance should be close to 1.0");
	let ray2 = Ray2 { origin: Point2(-0.5, 0.5), direction: Vec2(1.0, 0.0), distance: 10.0 };
	let hit2 = tri.trace(&ray2).expect("Ray should hit the triangle");
	assert!(hit2.distance - 0.5 < 1e-6, "Intersection distance should be close to 0.5");
}

#[test]
fn test_trace_triangle_centroid() {
	let tri = Triangle2::new(Point2(0.0, 0.0), Point2(1.0, 0.0), Point2(0.0, 1.0));
	let origin = Point2(-1.0, -1.0);
	let (direction, distance) = (tri.centroid() - origin).norm_len();
	let ray = Ray2::new(origin, direction, distance);

	let hit = tri.trace(&ray).expect("Ray should hit the triangle at centroid");
	assert!((hit.distance - (2.0).sqrt()).abs() < 1e-6, "Intersection distance should match");
}

#[test]
fn test_trace_triangle_vertices() {
	let tri = Triangle2::new(Point2(0.0, 0.0), Point2(1.0, 0.0), Point2(0.0, 1.0));
	let vertices = [Point2(0.0, 0.0), Point2(1.0, 0.0), Point2(0.0, 1.0)];
	let targets = [Point2(0.5, 0.5), Point2(0.0, 0.5), Point2(0.5, 0.0)];

	for (origin, target) in vertices.iter().zip(targets.iter()) {
		let (direction, distance) = (*target - *origin).norm_len();
		let ray = Ray2::new(*origin, direction, distance + 0.1);
		assert!(tri.trace(&ray).is_some(), "Ray from {:?} to {:?} should hit", origin, target);
	}
}

#[test]
fn test_trace_triangle_random_inside() {
	let tri = Triangle2(Point2(0.0, 0.0), Point2(1.0, 0.0), Point2(0.0, 1.0));
	let mut rng = urandom::new();

	for _ in 0..1000 {
		// Random barycentric coordinates inside triangle
		let u = rng.range(0.0..1.0);
		let v = rng.range(0.0..1.0 - u);
		let w = 1.0 - u - v;
		let target = Point2(
			u * 0.0 + v * 1.0 + w * 0.0,
			u * 0.0 + v * 0.0 + w * 1.0,
		);
		let origin = Point2(-2.0, -2.0);
		let (direction, distance) = (target - origin).norm_len();
		let ray = Ray2::new(origin, direction, distance + 0.1);

		assert!(tri.trace(&ray).is_some(), "Ray from {origin:?} to {target:?} should hit triangle");
	}
}
