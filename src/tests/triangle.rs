use super::*;

#[test]
fn test_ray_hits_triangle_from_outside() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray = Ray {
		origin: Point3(0.25, 0.25, 1.0),
		direction: Vec3(0.0, 0.0, -1.0),
	};

	let mut hits = [TraceHit::default(); 1];
	let hit_count = triangle.trace(&ray, &mut hits);

	assert_eq!(hit_count, 1);
	assert!((hits[0].distance - 1.0).abs() < 1e-6);
	assert_eq!(hits[0].normal, Vec3(0.0, 0.0, 1.0));
}

#[test]
fn test_ray_misses_triangle() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray = Ray {
		origin: Point3(1.1, 1.1, 1.0),
		direction: Vec3(0.0, 0.0, -1.0),
	};

	let mut hits = [TraceHit::default(); 1];
	let hit_count = triangle.trace(&ray, &mut hits);

	assert_eq!(hit_count, 0);
}

#[test]
fn test_ray_originates_inside_triangle() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray = Ray {
		origin: Point3(0.25, 0.25, -1.0),
		direction: Vec3(0.0, 0.0, 1.0),
	};

	let mut hits = [TraceHit::default(); 1];
	let hit_count = triangle.trace(&ray, &mut hits);

	assert_eq!(hit_count, 1);
	assert!((hits[0].distance - 1.0).abs() < 1e-6);
	assert_eq!(hits[0].normal, Vec3(0.0, 0.0, 1.0));
}

#[test]
fn test_inside_method() {
	let triangle = Triangle3 {
		p: Point3(0.0, 0.0, 0.0),
		u: Vec3(1.0, 0.0, 0.0),
		v: Vec3(0.0, 1.0, 0.0),
	};

	let ray_above = Ray {
		origin: Point3(0.5, 0.5, 1.0),
		direction: Vec3(0.0, 0.0, -1.0),
	};

	let ray_below = Ray {
		origin: Point3(0.5, 0.5, -1.0),
		direction: Vec3(0.0, 0.0, 1.0),
	};

	assert_eq!(triangle.inside(&ray_above), false); // In front = outside
	assert_eq!(triangle.inside(&ray_below), true);  // Behind = inside
}
