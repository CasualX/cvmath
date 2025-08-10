use super::*;

#[test]
fn test_collinear_points() {
	let pt1 = Point3(0.0, 0.0, 0.0);
	let pt2 = Point3(1.0, 1.0, 1.0);
	let pt3 = Point3(2.0, 2.0, 2.0);

	let plane = Plane3::triangle(pt1, pt2, pt3);
	assert_eq!(plane.normal, Vec3(0.0, 0.0, 0.0));
	assert_eq!(plane.distance, 0.0);
}

#[test]
fn test_trace() {
	let plane = Plane3(Vec3(0.0, 0.0, 1.0), 0.0);
	let ray = Ray3 {
		origin: Point3(0.0, 0.0, -5.0),
		direction: Vec3(0.0, 0.0, 1.0),
		distance: f64::INFINITY,
	};

	let inside = ray.inside(&plane);
	assert_eq!(inside, false);

	let result = ray.trace(&plane);
	assert!(result.is_some());
}

#[test]
fn test_trace_both_sides() {
	fn check(dir: f64, pt: f64, success: bool) {
		let plane = Plane3(Vec3::Z, 0.0);
		let ray = Ray3 {
			origin: Point3(0.0, 0.0, pt),
			direction: Vec3(0.0, 0.0, dir),
			distance: f64::INFINITY,
		};
		let result = ray.trace(&plane);
		if success {
			assert!(result.is_some());
			assert_eq!(result.unwrap().distance, pt.abs());
			assert_eq!(result.unwrap().normal, plane.normal);
		}
		else {
			assert!(result.is_none());
		}
	}

	// Trace towards the plane from both sides.
	check( 1.0, -2.0, true);
	check(-1.0,  2.0, true);

	// Trace away from the plane from both sides.
	check( 1.0,  2.0, false);
	check(-1.0, -2.0, false);
}

#[test]
fn test_trace_on_plane() {
	let plane = Plane3(Vec3(0.0, 0.0, 1.0), 0.0);
	let ray = Ray3 {
		origin: Point3(0.0, 0.0, 0.0),
		direction: Vec3(0.0, 0.0, 1.0),
		distance: f64::INFINITY,
	};

	let inside = ray.inside(&plane);
	assert_eq!(inside, true);

	let result = ray.trace(&plane);
	assert!(result.is_none()); // No intersection since the ray is on the plane
}

#[test]
fn test_trace_random_planes() {
	let mut rng = urandom::new();

	for _ in 0..1000 {
		// Random plane
		let normal = Vec3(rng.next_f32(), rng.next_f32(), rng.next_f32()).norm();
		let distance = rng.next_f32() * 10.0;
		let plane = Plane3(normal, -distance);

		// Trace away from the plane: start just outside the plane on the normal side
		let origin = normal * (distance + rng.next_f32());
		let direction = (normal + Vec3(rng.range(-0.5..0.5), rng.range(-0.5..0.5), rng.range(-0.5..0.5))).norm();
		let mut ray = Ray3 { origin, direction, distance: f32::INFINITY };

		// Should not hit
		let hit = ray.trace(&plane);
		assert!(hit.is_none(), "Ray should not hit the plane when moving away from it");

		// Reverse direction should hit
		ray.direction = -ray.direction;
		let hit = ray.trace(&plane).expect("Ray should hit the plane when moving towards it");

		// Hit point should be on the plane
		let d = plane.distance(ray.at(hit.distance));
		assert!(d.abs() < 1e-4, "Hit point should be on the plane: {d}");
	}
}
