use super::*;

#[test]
fn test_trace_capsule_body_entry() {
	let capsule = Capsule3(Point3(0.0, 0.0, 0.0), Vec3(4.0, 0.0, 0.0), 1.0);
	let ray = Ray3 {
		origin: Point3(2.0, -3.0, 0.0),
		direction: Vec3(0.0, 1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let hit = ray.trace(&capsule).expect("Ray should hit the capsule body");
	assert_eq!(hit.distance, 2.0);
	assert_eq!(hit.point, Point3(2.0, -1.0, 0.0));
	assert_eq!(hit.normal, Vec3(0.0, -1.0, 0.0));
	assert_eq!(hit.side, HitSide::Entry);
	assert_eq!(ray.inside(&capsule), false);
}

#[test]
fn test_trace_capsule_cap_exit_from_inside() {
	let capsule = Capsule3(Point3(0.0, 0.0, 0.0), Vec3(4.0, 0.0, 0.0), 1.0);
	let ray = Ray3 {
		origin: Point3(0.0, 0.0, 0.0),
		direction: Vec3(-1.0, 0.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let hit = ray.trace(&capsule).expect("Ray should exit through the start cap");
	assert_eq!(hit.distance, 1.0);
	assert_eq!(hit.point, Point3(-1.0, 0.0, 0.0));
	assert_eq!(hit.normal, Vec3(1.0, 0.0, 0.0));
	assert_eq!(hit.side, HitSide::Exit);
	assert_eq!(ray.inside(&capsule), true);
}

#[test]
fn test_trace_capsule_parallel_miss() {
	let capsule = Capsule3(Point3(0.0, 0.0, 0.0), Vec3(4.0, 0.0, 0.0), 1.0);
	let ray = Ray3 {
		origin: Point3(-2.0, 2.0, 0.0),
		direction: Vec3(1.0, 0.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	assert!(ray.trace(&capsule).is_none(), "Parallel ray outside the cylinder radius should miss");
}

#[test]
fn test_trace_capsule_degenerate_matches_sphere() {
	let capsule = Capsule3(Point3(1.0, -1.0, 2.0), Vec3::ZERO, 2.0);
	let sphere = Sphere(Point3(1.0, -1.0, 2.0), 2.0);
	let ray = Ray3 {
		origin: Point3(-4.0, -1.0, 2.0),
		direction: Vec3(1.0, 0.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let capsule_hit = ray.trace(&capsule).expect("Degenerate capsule should trace like a sphere");
	let sphere_hit = ray.trace(&sphere).expect("Sphere should be hit");

	assert_eq!(capsule_hit.distance, sphere_hit.distance);
	assert_eq!(capsule_hit.point, sphere_hit.point);
	assert_eq!(capsule_hit.normal, sphere_hit.normal);
	assert_eq!(capsule_hit.side, sphere_hit.side);
}

#[test]
fn test_trace_capsule_boundary_start_moving_away() {
	let capsule = Capsule3(Point3(0.0, 0.0, 0.0), Vec3(4.0, 0.0, 0.0), 1.0);
	let ray = Ray3 {
		origin: Point3(2.0, -1.0, 0.0),
		direction: Vec3(0.0, -1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	assert!(ray.trace(&capsule).is_none(), "Ray starting on the boundary and moving away should not self-hit");
}

#[test]
fn test_trace_capsule_zero_radius_matches_line_behavior() {
	let capsule = Capsule3(Point3(0.0, 0.0, 0.0), Vec3(4.0, 0.0, 0.0), 0.0);
	let line = Line3(Point3(0.0, 0.0, 0.0), Point3(4.0, 0.0, 0.0));
	let ray = Ray3 {
		origin: Point3(2.0, -3.0, 0.0),
		direction: Vec3(0.0, 1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	assert_eq!(ray.trace(&capsule), ray.trace(&line));
	assert_eq!(capsule.inside(ray.origin), false);
}

#[test]
fn test_lerp() {
	let capsule = lerp(
		Capsule3(Point3(1.0, 2.0, 3.0), Vec3(4.0, 5.0, 6.0), 7.0),
		Capsule3(Point3(5.0, 6.0, 7.0), Vec3(8.0, 9.0, 10.0), 11.0),
		0.5,
	);
	assert_eq!(capsule, Capsule3(Point3(3.0, 4.0, 5.0), Vec3(6.0, 7.0, 8.0), 9.0));
}