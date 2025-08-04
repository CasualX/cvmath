use super::*;

#[test]
fn test_y_intercept() {
	// Plane: normal = (0, 1), distance = -3
	// Equation: 0*x + 1*y - 3 = 0 => y = 3
	let plane = Plane2 {
		normal: Vec2(0.0, 1.0),
		distance: -3.0,
	};
	assert_eq!(plane.y_intercept(), Some(3.0));
	assert_eq!(plane.x_intercept(), None); // normal.x == 0 means no x intercept
}

#[test]
fn test_x_intercept() {
	// Plane: normal = (2, 0), distance = -4
	// Equation: 2*x + 0*y - 4 = 0 => x = 2
	let plane = Plane2 {
		normal: Vec2(2.0, 0.0),
		distance: -4.0,
	};
	assert_eq!(plane.x_intercept(), Some(2.0));
	assert_eq!(plane.y_intercept(), None); // normal.y == 0 means no y intercept
}

#[test]
fn test_intercept() {
	// Plane: normal = (1, 1), distance = -3
	// Equation: x + y - 3 = 0
	// y intercept (x=0): y = 3
	// x intercept (y=0): x = 3
	let plane = Plane2 {
		normal: Vec2(1.0, 1.0),
		distance: -3.0,
	};
	assert_eq!(plane.y_intercept(), Some(3.0));
	assert_eq!(plane.x_intercept(), Some(3.0));
}

#[test]
fn test_intersect_basic() {
	// Plane1: x = 0 (normal = (1, 0), distance = 0)
	let p1 = Plane2 {
		normal: Vec2(1.0, 0.0),
		distance: 0.0,
	};
	// Plane2: y = 0 (normal = (0, 1), distance = 0)
	let p2 = Plane2 {
		normal: Vec2(0.0, 1.0),
		distance: 0.0,
	};

	let intersection = p1.intersect(p2).unwrap();
	assert_eq!(intersection.x, 0.0);
	assert_eq!(intersection.y, 0.0);
}

#[test]
fn test_intersect_parallel() {
	// Both planes are x = 1 and x = 2, so no intersection
	let p1 = Plane2 {
		normal: Vec2(1.0, 0.0),
		distance: -1.0,
	};
	let p2 = Plane2 {
		normal: Vec2(1.0, 0.0),
		distance: -2.0,
	};

	assert!(p1.intersect(p2).is_none());
}

#[test]
fn test_intersect_random() {
	let mut rng = urandom::new();
	let epsilon = f32::EPSILON as f64;

	for _ in 0..1000 {
		let x1 = rng.range(-256.0..256.0).round();
		let y1 = rng.range(-256.0..256.0).round();
		let x2 = rng.range(-256.0..256.0).round();
		let y2 = rng.range(-256.0..256.0).round();
		let d1 = rng.range(-256.0..256.0);
		let d2 = rng.range(-256.0..256.0);

		let norm1 = Vec2(x1, y1).norm();
		let norm2 = Vec2(x2, y2).norm();

		if norm1.cross(norm2).abs() < epsilon {
			continue;
		}

		let plane1 = Plane2 {
			normal: norm1,
			distance: d1,
		};
		let plane2 = Plane2 {
			normal: norm2,
			distance: d2,
		};

		let intersection = plane1.intersect(plane2);
		let p = intersection.expect("Planes should intersect");

		// Verify intersection lies on both planes (within tolerance)
		let dist1 = plane1.distance(p);
		let dist2 = plane2.distance(p);

		assert!(dist1.abs() < epsilon, "Point {p} not on plane1 {plane1:?}: {dist1}");
		assert!(dist2.abs() < epsilon, "Point {p} not on plane2 {plane2:?}: {dist2}");
	}
}
