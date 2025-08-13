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

		let (vy, vx) = Angle::deg(rng.range(0.0..360.0)).sin_cos();
		let origin = Vec2(vx, vy) * rng.range(200.0..1000.0);

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

		let (vy, vx) = Angle::deg(rng.range(0.0..360.0)).sin_cos();
		let origin = Vec2(vx, vy) * rng.range(200.0..1000.0);

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
		let target = start.lerp(end, t);

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
