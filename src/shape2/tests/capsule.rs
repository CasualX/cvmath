use super::*;

#[test]
fn test_trace_capsule_body_entry() {
	let capsule = Capsule2(Point2(0.0, 0.0), Vec2(4.0, 0.0), 1.0);
	let ray = Ray2 {
		origin: Point2(2.0, -3.0),
		direction: Vec2(0.0, 1.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let hit = ray.trace(&capsule).expect("Ray should hit the capsule body");
	assert_eq!(hit.distance, 2.0);
	assert_eq!(hit.point, Point2(2.0, -1.0));
	assert_eq!(hit.normal, Vec2(0.0, -1.0));
	assert_eq!(hit.side, HitSide::Entry);
	assert_eq!(ray.inside(&capsule), false);
}

#[test]
fn test_trace_capsule_cap_exit_from_inside() {
	let capsule = Capsule2(Point2(0.0, 0.0), Vec2(4.0, 0.0), 1.0);
	let ray = Ray2 {
		origin: Point2(0.0, 0.0),
		direction: Vec2(-1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let hit = ray.trace(&capsule).expect("Ray should exit through the left cap");
	assert_eq!(hit.distance, 1.0);
	assert_eq!(hit.point, Point2(-1.0, 0.0));
	assert_eq!(hit.normal, Vec2(1.0, 0.0));
	assert_eq!(hit.side, HitSide::Exit);
	assert_eq!(ray.inside(&capsule), true);
}

#[test]
fn test_trace_capsule_parallel_miss() {
	let capsule = Capsule2(Point2(0.0, 0.0), Vec2(4.0, 0.0), 1.0);
	let ray = Ray2 {
		origin: Point2(-2.0, 2.0),
		direction: Vec2(1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	assert!(ray.trace(&capsule).is_none(), "Parallel ray above the capsule should miss");
}

#[test]
fn test_trace_capsule_degenerate_matches_circle() {
	let capsule = Capsule2(Point2(1.0, -1.0), Vec2::ZERO, 2.0);
	let circle = Circle(Point2(1.0, -1.0), 2.0);
	let ray = Ray2 {
		origin: Point2(-4.0, -1.0),
		direction: Vec2(1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let capsule_hit = ray.trace(&capsule).expect("Degenerate capsule should trace like a circle");
	let circle_hit = ray.trace(&circle).expect("Circle should be hit");

	assert_eq!(capsule_hit.distance, circle_hit.distance);
	assert_eq!(capsule_hit.point, circle_hit.point);
	assert_eq!(capsule_hit.normal, circle_hit.normal);
	assert_eq!(capsule_hit.side, circle_hit.side);
}

#[test]
fn test_trace_capsule_boundary_start_moving_away() {
	let capsule = Capsule2(Point2(0.0, 0.0), Vec2(4.0, 0.0), 1.0);
	let ray = Ray2 {
		origin: Point2(2.0, -1.0),
		direction: Vec2(0.0, -1.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	assert!(ray.trace(&capsule).is_none(), "Ray starting on the boundary and moving away should not self-hit");
}

#[test]
fn test_trace_capsule_zero_radius_matches_line() {
	let capsule = Capsule2(Point2(0.0, 0.0), Vec2(4.0, 0.0), 0.0);
	let line = Line2(Point2(0.0, 0.0), Point2(4.0, 0.0));
	let ray = Ray2 {
		origin: Point2(2.0, -3.0),
		direction: Vec2(0.0, 1.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let capsule_hit = ray.trace(&capsule).expect("Zero-radius capsule should trace like a line");
	let line_hit = ray.trace(&line).expect("Line should be hit");

	assert_eq!(capsule_hit.distance, line_hit.distance);
	assert_eq!(capsule_hit.point, line_hit.point);
	assert_eq!(capsule_hit.normal, line_hit.normal);
	assert_eq!(capsule_hit.side, line_hit.side);
	assert_eq!(capsule.inside(ray.origin), false);
}

#[test]
fn test_lerp() {
	let capsule = lerp(
		Capsule2(Point2(1.0, 2.0), Vec2(3.0, 4.0), 5.0),
		Capsule2(Point2(5.0, 6.0), Vec2(7.0, 8.0), 9.0),
		0.5,
	);
	assert_eq!(capsule, Capsule2(Point2(3.0, 4.0), Vec2(5.0, 6.0), 7.0));
}