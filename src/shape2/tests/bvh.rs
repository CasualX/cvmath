use super::*;

#[derive(Clone, Debug, PartialEq)]
struct IndexedCircle {
	id: usize,
	circle: Circle<f32>,
}

fn trace_indexed_circles(circles: &[IndexedCircle], ray: &Ray2<f32>) -> Option<Hit2<f32>> {
	let mut best = None;
	for circle in circles {
		if let Some(hit) = circle.circle.trace(ray) {
			if best.as_ref().is_none_or(|best: &Hit2<f32>| hit.distance < best.distance) {
				best = Some(Hit2 { index: circle.id, ..hit });
			}
		}
	}
	best
}

#[test]
fn test_bvh_optimize_reorder_reorders_circle_items() {
	let mut circles = vec![
		IndexedCircle { id: 0, circle: Circle { center: Point2(-9.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 1, circle: Circle { center: Point2(9.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 2, circle: Circle { center: Point2(-7.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 3, circle: Circle { center: Point2(7.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 4, circle: Circle { center: Point2(-5.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 5, circle: Circle { center: Point2(5.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 6, circle: Circle { center: Point2(-3.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 7, circle: Circle { center: Point2(3.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 8, circle: Circle { center: Point2(-1.0, 0.0), radius: 0.75 } },
		IndexedCircle { id: 9, circle: Circle { center: Point2(1.0, 0.0), radius: 0.75 } },
	];
	let original = circles.clone();

	let mut bvh = Bvh2::build(circles.iter().enumerate().map(|(index, item)| (index, item.circle.bounds())));
	circles = bvh.optimize_reorder(circles);

	assert_ne!(circles, original);

	for y in [-0.5, 0.0, 0.5] {
		let ray = Ray2(Point2(-12.0, y), Vec2(1.0, 0.0), Interval(0.0, 24.0));
		let linear = trace_indexed_circles(&circles, &ray);
		let traced = bvh.trace(&ray, |index, clipped| {
			circles[index].circle.trace(clipped).map(|hit| Hit2 { index: circles[index].id, ..hit })
		});

		match (linear, traced) {
			(None, None) => (),
			(Some(linear), Some(traced)) => {
				assert_eq!(linear.index, traced.index);
				assert!(linear.distance.is_close(traced.distance));
			}
			(linear, traced) => panic!("linear={linear:?} traced={traced:?}"),
		}
	}
}

#[derive(Copy, Clone, Debug)]
struct CircleFieldConfig {
	unit: f32,
	circle_count: usize,
	radius: f32,
	ray_count: usize,
	seed: u64,
}

#[derive(Clone, Debug)]
struct CircleField {
	region: Bounds2<f32>,
	circles: Vec<Circle<f32>>,
	bvh: Bvh2<f32>,
	rays: Vec<Ray2<f32>>,
}

impl CircleField {
	fn new(config: CircleFieldConfig) -> CircleField {
		let mut rng = urandom::seeded(config.seed);
		let scale = 2.5;
		let margin = 24.0;
		let unit = config.unit * scale;
		let radius = config.radius * scale;
		let origin = Point2(margin, margin + unit);
		let region = Bounds2(Point2(margin + unit, margin), Point2(margin + unit * 3.0, margin + unit * 2.0));

		let mut circles = Vec::with_capacity(config.circle_count);
		for _ in 0..config.circle_count {
			let center = Point2(
				rng.range(region.mins.x + radius..region.maxs.x - radius),
				rng.range(region.mins.y + radius..region.maxs.y - radius),
			);
			circles.push(Circle { center, radius });
		}

		let bvh = Bvh2::build(circles.iter().map(Circle::bounds).enumerate());

		let mut rays = Vec::with_capacity(config.ray_count);
		for _ in 0..config.ray_count {
			let target = Point2(region.maxs.x, rng.range(region.mins.y..region.maxs.y));
			let ray = Ray2::new(origin, target - origin, Interval(0.0, origin.distance(target)));
			rays.push(ray);
		}

		CircleField { region, circles, bvh, rays }
	}

	// Trace the ray against all circles without using the BVH
	fn trace_linear(&self, ray: &Ray2<f32>) -> Option<Hit2<f32>> {
		ray.trace_collection(&self.circles)
	}

	// Trace the ray using the BVH to find potential hits
	fn trace_bvh(&self, ray: &Ray2<f32>) -> Option<Hit2<f32>> {
		self.bvh.trace(ray, |index, clipped| {
			self.circles[index].trace(clipped).map(|hit| Hit2 { index, ..hit })
		})
	}

	fn ray_end(&self, ray: &Ray2<f32>, hit: Option<Hit2<f32>>) -> Point2<f32> {
		hit.map_or(ray.at(ray.distance.max), |hit| hit.point)
	}

	fn draw_trace(&self, svg: &mut SvgWriter, ray: &Ray2<f32>, end: Point2<f32>, color: &str, dashed: bool) {
		let mut line = svg.line(Line2(ray.origin, end));
		line.stroke(color)
			.stroke_width(0.3)
			.stroke_opacity(0.8);
		if dashed {
			line.stroke_dasharray(&[2.0, 2.0]);
		}
	}
}

#[test]
fn test_bvh_circle_field_matches_linear_and_renders_svg() {
	let scene = CircleField::new(CircleFieldConfig {
		unit: 100.0,
		circle_count: 100,
		radius: 2.0,
		ray_count: 4096,
		seed: 41,
	});

	// Render the scene as SVG
	let width = scene.region.maxs.x + 24.0;
	let height = scene.region.maxs.y + 24.0;
	let mut svg = SvgWriter::new(width, height);
	svg.rect(Bounds2(Point2::ZERO, Point2(width, height)))
		.fill("black")
		.stroke("none");
	svg.rect(scene.region)
		.fill("none")
		.stroke("rgba(255,255,255,0.35)")
		.stroke_width(1.0);
	for &circle in &scene.circles {
		svg.circle(circle)
			.fill("rgba(90, 180, 255, 0.22)")
			.stroke("rgba(90, 180, 255, 0.5)")
			.stroke_width(0.35);
	}

	let mut mismatches = Vec::new();
	for (ray_index, ray) in scene.rays.iter().enumerate() {
		let lin = scene.trace_linear(ray);
		let bvh = scene.trace_bvh(ray);

		let lin_end = scene.ray_end(ray, lin);
		let bvh_end = scene.ray_end(ray, bvh);

		// Draw the ray paths for both linear and BVH traces
		scene.draw_trace(&mut svg, ray, lin_end, "rgba(120, 255, 160, 0.9)", true);
		scene.draw_trace(&mut svg, ray, bvh_end, "rgba(255, 210, 80, 0.9)", false);

		// Check if the BVH hit matches the linear hit
		// (both should be None or both should have the same index and similar distance)
		match (lin, bvh) {
			(None, None) => (),
			(Some(linear), Some(bvh)) => {
				if !(linear.index == bvh.index && linear.distance.is_close(bvh.distance)) {
					mismatches.push(format!("ray {ray_index}: linear={linear:?} bvh={bvh:?}"));
				}
			}
			(linear, bvh) => mismatches.push(format!("ray {ray_index}: linear={linear:?} bvh={bvh:?}")),
		}
	}

	let _ = std::fs::create_dir("target/visual");
	svg.save("target/visual/test_bvh_circle_field.svg").unwrap();

	assert!(mismatches.is_empty(), "{} BVH mismatches\n{}", mismatches.len(), mismatches.join("\n"));
}
