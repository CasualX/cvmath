use cvmath::*;

const NSHAPES: usize = 50;
const NRAYS: i32 = 20;
const MAX_BOUNCES: i32 = 10;
const MAX_DISTANCE: f32 = f32::INFINITY;
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

#[path = "../../src/shape2/tests/svgwriter.rs"]
mod writer;

fn main() {
	let seed: u32 = urandom::new().next_u32();
	// let seed: u32 = 0x583d44f0; // Ray starts inside sphere clipped by triangle
	// let seed: u32 = 0xe2c551aa; // Ray starts inside sphere clipped by triangle
	// let seed: u32 = 0xeff6f32f; // Ray starts inside triangle
	// let seed: u32 = 0x70394217; // Ray starts inside triangle clipped by line and triangle
	// let seed: u32 = 0x13266554; // Ray starts inside bounds
	// let seed: u32 = 0x26970eab; // Ray starts inside bounds
	// let seed: u32 = 0xef853999; // Ray starts inside bounds clipped by circle and triangle
	let mut rand = urandom::seeded(seed as u64);

	println!("Using seed={seed:#x}.");

	let mut svg = writer::SvgWriter::new(WIDTH, HEIGHT);

	svg.rect(Bounds2 {
		mins: Point2(0.0, 0.0),
		maxs: Point2(WIDTH, HEIGHT),
	}).fill("#222");

	// Add the edges of the playground
	let mut shapes: Vec<Shape2<f32>> = vec![
		Shape2::Line(Line2(Point2(0.0, 0.0), Point2(WIDTH, 0.0))),
		Shape2::Line(Line2(Point2(WIDTH, HEIGHT), Point2(WIDTH, 0.0))),
		Shape2::Line(Line2(Point2(WIDTH, HEIGHT), Point2(0.0, HEIGHT))),
		Shape2::Line(Line2(Point2(0.0, 0.0), Point2(0.0, HEIGHT))),

		// Shape2::Bounds(Bounds2 {
		// 	mins: Point2(0.0, 0.0),
		// 	maxs: Point2(WIDTH, HEIGHT),
		// }.norm()),
	];

	// Add random shapes
	for _ in 0..NSHAPES {
		shapes.push(random_shape(&mut rand));
	}

	// Render the shapes as SVG
	for shape in &shapes {
		draw_shape(&mut svg, shape);
	}

	let start_angle = Angle::deg(rand.range(0.0..360.0));
	let angle_step = Angle::deg(360.0f32) / NRAYS as f32;

	let origin = Point2(rand.range(10.0..WIDTH - 10.0), rand.range(10.0..HEIGHT - 10.0));

	for ray_index in 0..NRAYS {
		let mut ray = Ray2 {
			origin,
			direction: Polar(1.0f32, start_angle + angle_step * ray_index as f32).complex().vec2(),
			distance: Interval(0.01, MAX_DISTANCE),
		};

		// Simple bounce loop with reflections
		for bounce_index in 0..MAX_BOUNCES {
			if let Some(hit) = ray.trace_collection(&shapes) {
				let alpha = 1.0f32 / (1.0f32 + bounce_index as f32);
				svg.arrow(hit.point, hit.point + hit.normal * 10.0, 5.0)
					.stroke(&format!("rgba(255, 255, 255, {})", alpha))
					.stroke_width(0.5);
				svg.line(Line2(ray.origin, hit.point))
					.stroke(&format!("rgba(255, 255, 255, {})", alpha))
					.stroke_width(0.5);

				ray = ray.reflect(&hit);
			}
			else {
				if ray.distance.max.is_finite() {
					let alpha = 1.0f32 / (1.0f32 + bounce_index as f32);
					svg.arrow(ray.origin, ray.at(ray.distance.max), 5.0)
						.stroke(&format!("rgba(255, 255, 255, {})", alpha))
						.stroke_width(0.5);
				}
				break;
			}
		}
	}

	std::fs::write("rays2d.svg", svg.close()).expect("Unable to write file");
}

fn random_shape(rand: &mut urandom::Random<impl urandom::Rng>) -> Shape2<f32> {
	let x = rand.range(50.0..WIDTH - 50.0);
	let y = rand.range(50.0..HEIGHT - 50.0);

	match rand.range(0..4) {
		0 => {
			let dx = rand.range(-100.0..100.0);
			let dy = rand.range(-100.0..100.0);
			Shape2::Line(Line2 {
				start: Point2(x, y),
				end: Point2(x + dx, y + dy),
			})
		}
		1 => {
			let dx = rand.range(-100.0..100.0);
			let dy = rand.range(-100.0..100.0);
			Shape2::Bounds(Bounds2 {
				mins: Point2(x, y),
				maxs: Point2(x + dx, y + dy),
			}.norm())
		}
		2 => {
			let ux = rand.range(-100.0..100.0);
			let uy = rand.range(-100.0..100.0);
			let vx = rand.range(-100.0..100.0);
			let vy = rand.range(-100.0..100.0);
			Shape2::Triangle(Triangle2(
				Point2(x, y),
				Point2(ux, uy),
				Point2(vx, vy),
			).norm())
		}
		3 => {
			let radius = rand.range(10.0..40.0);
			Shape2::Circle(Circle {
				center: Point2(x, y),
				radius,
			})
		}
		_ => unimplemented!(),
	}
}

fn draw_shape(svg: &mut writer::SvgWriter, &shape: &Shape2<f32>) {
	match shape {
		Shape2::Circle(circle) => {
			svg.circle(circle).fill("#C084F5").stroke("black").stroke_width(0.5);
		}
		Shape2::Bounds(bounds) => {
			svg.rect(bounds).fill("#FF8C69").stroke("black").stroke_width(0.5);
		}
		Shape2::Line(line) => {
			svg.line(line).stroke("#4FD2D2").stroke_width(2.0);
		}
		Shape2::Triangle(triangle) => {
			svg.triangle(triangle).fill("#FFC857").stroke("black").stroke_width(0.5);
		}
		_ => unimplemented!(),
	}
}
