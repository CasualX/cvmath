use super::*;

#[test]
fn test_collinear_points() {
	let pt1 = Point3(0.0, 0.0, 0.0);
	let pt2 = Point3(1.0, 1.0, 1.0);
	let pt3 = Point3(2.0, 2.0, 2.0);

	let plane = Plane::points(pt1, pt2, pt3);
	assert_eq!(plane.normal, Vec3(0.0, 0.0, 0.0));
	assert_eq!(plane.distance, 0.0);
}

#[test]
fn test_trace() {
	let plane = Plane(Vec3(0.0, 0.0, 1.0), 0.0);
	let ray = Ray {
		origin: Point3(0.0, 0.0, -5.0),
		direction: Vec3(0.0, 0.0, 1.0),
	};

	let inside = ray.inside(&plane);
	assert_eq!(inside, true);

	let count = ray.trace(&plane, &mut []);
	assert_eq!(count, 1);
}

#[test]
fn test_trace_both_sides() {
	fn check(dir: f64, pt: f64, success: bool) {
		let plane = Plane(Vec3::Z, 0.0);
		let ray = Ray {
			origin: Point3(0.0, 0.0, pt),
			direction: Vec3(0.0, 0.0, dir),
		};
		let mut hits = [TraceHit::default(); 2];
		let count = ray.trace(&plane, &mut hits);
		if success {
			assert_eq!(count, 1);
			assert_eq!(hits[0].distance, pt.abs());
			assert_eq!(hits[0].normal, Vec3::Z);
		}
		else {
			assert_eq!(count, 0);
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
	let plane = Plane(Vec3(0.0, 0.0, 1.0), 0.0);
	let ray = Ray {
		origin: Point3(0.0, 0.0, 0.0),
		direction: Vec3(0.0, 0.0, 1.0),
	};

	let inside = ray.inside(&plane);
	assert_eq!(inside, true);

	let count = ray.trace(&plane, &mut []);
	assert_eq!(count, 1);
}
