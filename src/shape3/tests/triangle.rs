use super::*;

#[test]
fn test_ray_hits_triangle_from_outside() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray = Ray3 {
		origin: Point3(0.25, 0.25, 1.0),
		direction: Vec3(0.0, 0.0, -1.0),
		distance: f64::INFINITY,
	};

	let result = triangle.trace(&ray);

	assert!(result.is_some());
	assert!((result.unwrap().distance - 1.0).abs() < 1e-6);
	assert_eq!(result.unwrap().normal, Vec3(0.0, 0.0, 1.0));
}

#[test]
fn test_ray_misses_triangle() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray = Ray3 {
		origin: Point3(1.1, 1.1, 1.0),
		direction: Vec3(0.0, 0.0, -1.0),
		distance: f64::INFINITY,
	};

	let result = triangle.trace(&ray);

	assert!(result.is_none());
}

#[test]
fn test_ray_originates_inside_triangle() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray = Ray3 {
		origin: Point3(0.25, 0.25, -1.0),
		direction: Vec3(0.0, 0.0, 1.0),
		distance: f64::INFINITY,
	};

	let result = triangle.trace(&ray);

	assert!(result.is_some());
	assert!((result.unwrap().distance - 1.0).abs() < 1e-6);
	assert_eq!(result.unwrap().normal, Vec3(0.0, 0.0, 1.0));
}

#[test]
fn test_trace_random_triangles() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		// Random triangle
		let p1 = Point3(rng.next_f32(), rng.next_f32(), rng.next_f32());
		let p2 = Point3(rng.next_f32(), rng.next_f32(), rng.next_f32());
		let p3 = Point3(rng.next_f32(), rng.next_f32(), rng.next_f32());
		let triangle = Triangle3::points(p1, p2, p3);

		// Skip degenerate triangles
		if triangle.area().abs() < 1e-6 {
			continue;
		}

		// Trace away from the triangle
		let direction = triangle.plane().normal;
		let origin = triangle.centroid() + direction;

		// Should not hit
		let mut ray = Ray3 { origin, direction, distance: f32::INFINITY };
		let hit = ray.trace(&triangle);
		assert!(hit.is_none(), "Ray should not hit the triangle when moving away from it");

		// Sample random point on the plane nearby the triangle
		let x = rng.range(-0.5..1.5);
		let y = rng.range(-0.5..1.5);
		let p = triangle.p + triangle.u * x + triangle.v * y;

		// Aim at the point
		ray.direction = (p - ray.origin).norm();
		let hit = ray.trace(&triangle);

		// Should only hit if the point is inside the triangle if we're sure about the accuracy
		let is_inside = x > 0.01 && x < 0.99 && y > 0.01 && y < 0.99 && x + y < 0.99;
		if is_inside {
			assert!(hit.is_some(), "Ray should hit the triangle when moving towards it {x} {y}");
		}
		let is_outside = x < -0.01 || x > 1.01 || y < -0.01 || y > 1.01 || x + y > 1.01;
		if is_outside {
			assert!(hit.is_none(), "Ray should miss the triangle when moving away from it {x} {y}");
		}

		// Hit point is on the triangle's plane
		if let Some(hit) = hit {
			let point = ray.at(hit.distance);
			let d = triangle.plane().distance(point);
			assert!(d.abs() < 1e-4, "Hit point d={d} should be on the triangle's plane");
		}
	}
}
