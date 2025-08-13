use super::*;

#[test]
fn test_ray_accuracy() {
	// Plane must be slightly offset to trigger accuracy issues
	let plane = Plane2(Vec2(1.0, 0.01).norm(), 0.0);

	// Start the ray aimed directly at the plane
	let mut ray = Ray2(Point2(1.0, 1.0), Vec2(-1.0, 0.0).norm(), Interval(0.0, f32::INFINITY));

	for _ in 0..1000 {
		let hit = plane.trace(&ray).unwrap();

		// Reflected ray should not intersect the plane again
		let reflected = (-ray.direction).reflect(hit.normal);
		let ray2 = Ray2(hit.point + reflected * 0.00001, reflected, Interval(0.0, f32::INFINITY));

		if let Some(hit2) = plane.trace(&ray2) {
			panic!("{ray:?} should not hit the {plane:?} again: {hit2:?}");
		}

		// Increase the angle which we're aiming at
		ray.direction.y -= 10.0;
		ray.direction = ray.direction.norm();
	}
}

#[test]
fn test_reflect_refract() {
	const WIDTH: f32 = 1200.0;
	const HEIGHT: f32 = 600.0;
	const NRAYS: i32 = 300;
	const IOR_AIR: f32 = 1.000293; // Air
	const IOR_GLASS: f32 = 1.458; // Glass

	let mut svg = SvgWriter::new(WIDTH, HEIGHT);
	svg.rect(Bounds2(Point2::ZERO, Point2(WIDTH, HEIGHT)))
		.fill("black")
		.stroke("none");

	let ray = Ray2 {
		origin: Point2(50.0, HEIGHT * 0.5),
		direction: Vec2(1.0, 0.0),
		distance: Interval(0.0001, 5000.0),
	};

	let circle = Circle {
		center: Point2(WIDTH * 0.5, HEIGHT * 0.5),
		radius: HEIGHT * 0.25,
	};
	svg.circle(circle).fill("none").stroke("white").stroke_width(2.0);
	let shape = Shape2::Circle(circle);

	struct TraceContext<'a> {
		svg: &'a mut SvgWriter,
		shape: Shape2<f32>,
	}
	let mut ctx = TraceContext { svg: &mut svg, shape };

	fn trace(ctx: &mut TraceContext, ray: Ray2<f32>, depth: i32) {
		if depth > 8 {
			return;
		}

		if let Some(hit) = ctx.shape.trace(&ray) {
			ctx.svg.line(Line2(ray.origin, hit.point))
				.stroke(&format!("rgba(255, 255, 0, {})", 1.0 / (1.0 + depth as f32)))
				.stroke_width(1.0);

			let reflect_ray = ray.reflect(&hit);
			trace(ctx, reflect_ray, depth + 1);

			if let Some(refract_ray) = ray.refract(&hit, IOR_AIR, IOR_GLASS) {
				trace(ctx, refract_ray, depth + 1);
			}
		}
		else {
			ctx.svg.arrow(ray.origin, ray.at(ray.distance.max), 5.0)
				.stroke(&format!("rgba(255, 255, 0, {})", 1.0 / (1.0 + depth as f32)))
				.stroke_width(1.0);
		}
	}

	for i in -NRAYS / 2..NRAYS / 2 {
		let y = i as f32 / (NRAYS as f32 / 3.0) * HEIGHT * 0.25;
		let direction = (Point2(WIDTH * 0.5, HEIGHT * 0.5 + y) - ray.origin).norm();
		let ray = Ray2 { direction, ..ray };

		trace(&mut ctx, ray, 0);
	}

	let _ = std::fs::create_dir("target/visual");
	std::fs::write("target/visual/test_reflect_refract.svg", svg.close()).unwrap();
}
