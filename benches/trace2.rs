#![feature(test)]

extern crate test;

use cvmath::*;
use std::hint::black_box;
use test::Bencher;

#[bench]
fn bench_trace2_random_bounds(b: &mut Bencher) {
	let mut rng = urandom::new();

	let data: Vec<_> = black_box((0..1000).filter_map(|_| {
		let x1 = rng.range(-100.0..100.0);
		let y1 = rng.range(-100.0..100.0);
		let x2 = rng.range(-100.0..100.0);
		let y2 = rng.range(-100.0..100.0);
		let bounds = Bounds2(Point2(x1, y1), Point2(x2, y2)).norm();

		// Skip degenerate bounds
		if bounds.area() < 1e-6 {
			return None;
		}

		let tx = rng.range(bounds.mins.x..bounds.maxs.x);
		let ty = rng.range(bounds.mins.y..bounds.maxs.y);
		let target = Point2(tx, ty);

		let dir = Angle::deg(rng.range(0.0..360.0)).vec2();
		let origin = dir * rng.range(200.0..1000.0);

		let direction = (target - origin).norm();
		let ray = Ray2 { origin, direction, distance: Interval(0.0, f64::INFINITY) };

		Some((bounds, ray))
	}).collect());

	b.iter(|| {
		let (bounds, ray) = rng.choose(&data).unwrap();
		black_box(black_box(ray).trace(black_box(bounds)));
	});
}

#[bench]
fn bench_trace2_random_circles(b: &mut Bencher) {
	let mut rng = urandom::new();

	let data: Vec<_> = black_box((0..1000).filter_map(|_| {
		let center = Point2(rng.next_f64() * 50.0, rng.next_f64() * 50.0);
		let radius = rng.range(1.0..50.0);
		let circle = Circle { center, radius };
		let bounds = circle.bounds();

		let tx = rng.range(bounds.mins.x..bounds.maxs.x);
		let ty = rng.range(bounds.mins.y..bounds.maxs.y);
		let target = Point2(tx, ty);

		// Skip points outside the circle
		let distance = target.distance(center);
		if distance + 1e-6 >= radius {
			return None;
		}

		let dir = Angle::deg(rng.range(0.0..360.0)).vec2();
		let origin = dir * rng.range(200.0..1000.0);

		let direction = (target - origin).norm();
		let ray = Ray2 { origin, direction, distance: Interval(0.0, f64::INFINITY) };

		Some((circle, ray))
	}).collect());

	b.iter(|| {
		let (circle, ray) = rng.choose(&data).unwrap();
		black_box(black_box(ray).trace(black_box(circle)));
	});
}

#[bench]
fn bench_trace2_random_lines(b: &mut Bencher) {
	let mut rng = urandom::new();

	let data: Vec<_> = black_box((0..1000).filter_map(|_| {
		let start = Point2(rng.next_f64() * 50.0, rng.next_f64() * 50.0);
		let end = Point2(rng.next_f64() * 50.0, rng.next_f64() * 50.0);
		let line = Line2(start, end);

		// Skip zero-length lines
		if start.distance(end) < 1e-6 {
			return None;
		}

		let t = rng.range(-0.5..1.5);
		let target = lerp(start, end, t);

		let origin = Point2(rng.next_f64() * 50.0, rng.next_f64() * 50.0);

		let direction = (target - origin).norm();
		let ray = Ray2 { origin, direction, distance: Interval(0.0, f64::INFINITY) };

		Some((line, ray))
	}).collect());

	b.iter(|| {
		let (line, ray) = rng.choose(&data).unwrap();
		black_box(black_box(ray).trace(black_box(line)));
	});
}

#[bench]
fn bench_trace2_random_planes(b: &mut Bencher) {
	let mut rng = urandom::new();

	let data: Vec<_> = black_box((0..1000).filter_map(|_| {
		// Random plane
		let normal = Vec2(rng.next_f64(), rng.next_f64()).norm();
		let distance: f64 = rng.next();
		let plane = Plane2 { normal, distance };

		// Trace towards the plane
		let origin = normal * (distance + rng.next_f64());
		let direction = -(normal + Vec2(rng.range(-0.5..0.5), rng.range(-0.5..0.5))).norm();
		let ray = Ray2 { origin, direction, distance: Interval(0.0, f64::INFINITY) };

		Some((plane, ray))
	}).collect());

	b.iter(|| {
		let (plane, ray) = rng.choose(&data).unwrap();
		black_box(black_box(ray).trace(black_box(plane)));
	});
}

#[bench]
fn bench_trace2_random_triangles(b: &mut Bencher) {
	let tri = Triangle2(Point2(0.0, 0.0), Point2(1.0, 0.0), Point2(0.0, 1.0));
	let mut rng = urandom::new();

	let data: Vec<_> = black_box((0..1000).filter_map(|_| {
		// Random barycentric coordinates inside triangle
		let u = rng.range(0.0..1.0);
		let v = rng.range(0.0..1.0 - u);
		let w = 1.0 - u - v;
		let target = Point2(
			u * 0.0 + v * 1.0 + w * 0.0,
			u * 0.0 + v * 0.0 + w * 1.0,
		);

		// Trace towards the triangle
		let origin = Point2(-2.0, -2.0);
		let direction = (target - origin).norm();
		let ray = Ray2 { origin, direction, distance: Interval(0.0, f64::INFINITY) };

		Some((tri.clone(), ray))
	}).collect());

	b.iter(|| {
		let (tri, ray) = rng.choose(&data).unwrap();
		black_box(black_box(ray).trace(black_box(tri)));
	});
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
	circles: Vec<Circle<f32>>,
	bvh: Bvh2<f32>,
	rays: Vec<Ray2<f32>>,
}

impl CircleField {
	fn new(config: CircleFieldConfig) -> CircleField {
		let mut rng = urandom::seeded(config.seed);
		let unit = config.unit;
		let region = Bounds2(Point2(unit, -unit), Point2(unit * 3.0, unit));
		let circles = (0..config.circle_count)
			.map(|_| Circle {
				center: Point2(
					rng.range(region.mins.x + config.radius..region.maxs.x - config.radius),
					rng.range(region.mins.y + config.radius..region.maxs.y - config.radius),
				),
				radius: config.radius,
			})
			.collect::<Vec<_>>();
		let bvh = Bvh2::build(circles.iter().map(Circle::bounds).enumerate());
		let rays = (0..config.ray_count)
			.map(|_| {
				let target = Point2(unit * 3.0, rng.range(-unit * 0.5..unit * 0.5));
				Ray2::new(Point2::ZERO, target, Interval(0.0, f32::INFINITY))
			})
			.collect();

		CircleField { circles, bvh, rays }
	}

	fn trace_linear(&self, ray: &Ray2<f32>) -> Option<Hit2<f32>> {
		ray.trace_collection(&self.circles)
	}

	fn trace_bvh(&self, ray: &Ray2<f32>) -> Option<Hit2<f32>> {
		self.bvh.trace(ray, |index, ray| {
			self.circles[index].trace(ray).map(|hit| Hit2 { index, ..hit })
		})
	}
}

fn circle_field_bench() -> CircleField {
	CircleField::new(CircleFieldConfig {
		unit: 100.0,
		circle_count: 1000,
		// Chosen so a ray through the 2A-tall field misses roughly half the circles field-wide.
		radius: 0.07,
		ray_count: 4096,
		seed: 41,
	})
}

#[bench]
fn bench_trace2_circle_field_linear(b: &mut Bencher) {
	let scene = black_box(circle_field_bench());
	let mut rng = urandom::new();

	b.iter(|| {
		let ray = rng.choose(&scene.rays).unwrap();
		black_box(scene.trace_linear(black_box(ray)));
	});
}

#[bench]
fn bench_trace2_circle_field_bvh(b: &mut Bencher) {
	let scene = black_box(circle_field_bench());
	let mut rng = urandom::new();

	b.iter(|| {
		let ray = rng.choose(&scene.rays).unwrap();
		black_box(scene.trace_bvh(black_box(ray)));
	});
}
