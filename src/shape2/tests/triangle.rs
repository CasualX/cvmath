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
fn test_trace_random_triangles() {
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

		let mut ray = Ray2::new(origin, direction, distance + 0.1);
		assert!(tri.trace(&ray).is_some(), "{ray:?} to {target:?} should hit triangle");

		ray.direction = -ray.direction;
		assert!(tri.trace(&ray).is_none(), "{ray:?} away from triangle should miss");
	}
}

#[test]
fn test_random_points_inside() {
	use urandom::Distribution;
	let mut rng = urandom::new();
	let distr = urandom::distr::Uniform::from(-10.0..10.0).map(|x| x.round());

	for _ in 0..1000 {
		let p1 = Point2(rng.sample(&distr), rng.sample(&distr));
		let p2 = Point2(rng.sample(&distr), rng.sample(&distr));
		let p3 = Point2(rng.sample(&distr), rng.sample(&distr));
		let tri = Triangle2::points(p1, p2, p3);

		// Skip degenerate triangles
		if tri.area() < 1e-4 {
			continue;
		}

		let point = Point2(rng.sample(&distr), rng.sample(&distr));

		let Vec2 { x, y } = tri.decompose(point);
		let inside = x >= 0.0 && y >= 0.0 && (x + y) <= 1.0;
		assert_eq!(inside, tri.inside(point), "Expected {point:?} inside {tri:?}");
	}
}
