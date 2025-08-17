use super::*;

#[test]
fn trace_outside() {
	let sphere = Sphere(Point3(0.0, 0.0, 0.0), 1.0);
	let ray = Ray3 {
		origin: Point3(0.0, 0.0, 2.0),
		direction: Vec3(0.0, 0.0, -1.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let inside = ray.inside(&sphere);
	let result = ray.trace(&sphere);
	assert_eq!(inside, false);
	assert!(result.is_some());

	let result = ray.trace(&sphere);
	assert!(result.is_some());
	assert_eq!(result.unwrap().distance, 1.0);
}

#[test]
fn trace_inside() {
	let sphere = Sphere(Point3(0.0, 0.0, 0.0), 10.0);
	let ray = Ray3 {
		origin: Point3(0.0, -5.0, 0.0),
		direction: Vec3(0.0, 1.0, 0.0),
		distance: Interval(0.0, f64::INFINITY),
	};

	let inside = ray.inside(&sphere);
	let result = ray.trace(&sphere);
	assert_eq!(inside, true);
	assert!(result.is_some());

	let result = ray.trace(&sphere);
	assert!(result.is_some());
	assert_eq!(result.unwrap().distance, 15.0);
}

#[test]
fn ray_hits_off_center() {
	let ray = Ray3 {
		origin: Point3::new(0.5, 0.0, -5.0),
		direction: Vec3::new(0.0, 0.0, 1.0),
		distance: Interval(0.0, f64::INFINITY),
	};
	let sphere = Sphere { center: Point3::ZERO, radius: 1.0 };
	let result = ray.trace(&sphere);
	assert!(result.is_some());
}
