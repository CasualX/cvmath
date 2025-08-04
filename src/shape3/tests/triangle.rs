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
fn test_inside_method() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray_above = Ray3 {
		origin: Point3(0.5, 0.5, 1.0),
		direction: Vec3(0.0, 0.0, -1.0),
		distance: f64::INFINITY,
	};

	let ray_below = Ray3 {
		origin: Point3(0.5, 0.5, -1.0),
		direction: Vec3(0.0, 0.0, 1.0),
		distance: f64::INFINITY,
	};

	assert_eq!(triangle.plane().inside(ray_above.origin), false); // In front = outside
	assert_eq!(triangle.plane().inside(ray_below.origin), true);  // Behind = inside
}
