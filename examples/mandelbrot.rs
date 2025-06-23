use std::{f64, fs};
use std::fmt::Write;
use cvmath::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

static COLORS: [[u8; 3]; 16] = [
	[66, 30, 15],
	[25, 7, 26],
	[9, 1, 47],
	[4, 4, 73],
	[0, 7, 100],
	[12, 44, 138],
	[24, 82, 177],
	[57, 125, 209],
	[134, 181, 229],
	[211, 236, 248],
	[241, 233, 191],
	[248, 201, 95],
	[255, 170, 0],
	[204, 128, 0],
	[153, 87, 0],
	[106, 52, 3],
];

fn sample(p: f64) -> [u8; 3] {
	let color_index = ((p * COLORS.len() as f64) as i32 as usize).min(COLORS.len() - 1);
	COLORS[color_index]
}

fn main() {
	let mut surface = String::with_capacity(WIDTH * HEIGHT);
	write!(surface, "P3\n{WIDTH} {HEIGHT} 255\n").unwrap();

	// let mat = Transform2::fit(
	// 	Bounds2::c(0.0, 0.0, WIDTH as f32, HEIGHT as f32),
	// 	Bounds2::c(-0.8, 0.1, -0.7, -0.1),
	// );
	let mat = Transform2d::fit(
		Bounds2::c(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
		Bounds2::c(-0.743643887037151, 0.13182590420633, -0.743643887036151, 0.13182590420533),
	);

	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			let v = Vec2(x as f64, y as f64);
			let v1 = Complex::from(mat * (v + Vec2(0.25, 0.25)));
			let v2 = Complex::from(mat * (v + Vec2(-0.25, 0.25)));
			let v3 = Complex::from(mat * (v + Vec2(0.25, -0.25)));
			let v4 = Complex::from(mat * (v + Vec2(-0.25, -0.25)));

			let s1 = mandelbrot(v1, 5000);
			let s2 = mandelbrot(v2, 5000);
			let s3 = mandelbrot(v3, 5000);
			let s4 = mandelbrot(v4, 5000);
			let [red, green, blue] = sample((s1 + s2 + s3 + s4) / 4.0);

			write!(surface, "{red} {green} {blue} ").unwrap();
		}
		surface.push_str("\n");
	}

	fs::write("mandelbrot.ppm", surface).unwrap();
}

fn mandelbrot(c0: Complexd, max_iters: usize) -> f64 {
	let mut v = Complexd::ZERO;
	for iter in 0..max_iters {
		if v.abs_sqr() > 4.0 {
			// Apply smooth escape time formula
			let log_zn = (v.abs_sqr()).ln() / 2.0;
			let nu = (log_zn / f64::consts::LN_2).ln() / f64::consts::LN_2;
			let smooth = iter as f64 + 1.0 - nu;
			return smooth.log(1.3) % 1.0;
		}
		v = v.sqr() + c0;
	}
	return 1.0;
}
