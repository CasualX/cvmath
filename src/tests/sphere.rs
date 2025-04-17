use super::*;

#[test]
fn trace_outside() {
	let sphere = Sphere(Point3(0.0, 0.0, 0.0), 1.0);
	let ray = Ray {
		origin: Point3(0.0, 0.0, 2.0),
		direction: Vec3(0.0, 0.0, -1.0),
	};

	let inside = ray.inside(&sphere);
	let count = ray.trace(&sphere, &mut []);
	assert_eq!(inside, false);
	assert_eq!(count, 2);

	let mut hits = [TraceHit::default(); 2];
	let count = ray.trace(&sphere, &mut hits);
	assert_eq!(count, 2);
	assert_eq!(hits[0].distance, 1.0);
	assert_eq!(hits[1].distance, 3.0);
}

#[test]
fn trace_inside() {
	let sphere = Sphere(Point3(0.0, 0.0, 0.0), 10.0);
	let ray = Ray {
		origin: Point3(0.0, -5.0, 0.0),
		direction: Vec3(0.0, 1.0, 0.0),
	};

	let inside = ray.inside(&sphere);
	let count = ray.trace(&sphere, &mut []);
	assert_eq!(inside, true);
	assert_eq!(count, 1);

	let mut hits = [TraceHit::default(); 2];
	let count = ray.trace(&sphere, &mut hits);
	assert_eq!(count, 1);
	assert_eq!(hits[0].distance, 15.0);
}

#[test]
fn ray_hits_off_center() {
	let ray = Ray::new(Point3::new(0.5, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
	let sphere = Sphere { center: Point3::ZERO, radius: 1.0 };
	let mut hits = [TraceHit::default(); 2];
	let count = ray.trace(&sphere, &mut hits);
	assert_eq!(count, 2, "Expected 2 hits, but got {}", count);
}
