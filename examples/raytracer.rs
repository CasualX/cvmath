use std::sync::mpsc;
use cvmath::*;
use rayon::prelude::*;

// Image setup
const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1200;

// Camera setup
const ORIGIN: Point3<f32> = Point3::new(0.0, 1.5, 0.0);
const TARGET: Point3<f32> = Point3::new(0.0, 1.5, -1.0);
const REF_UP: Vec3<f32> = Vec3::new(0.0, 1.0, 0.0);
const FOV_Y: f32 = 90.0;

// Quality setup
const DOF: bool = true; // Enable depth-of-field
const AA: bool = true; // Enable anti-aliasing
const AA_SAMPLES: usize = 64; // Number of anti-aliasing samples per pixel
const MAX_DEPTH: u32 = 6;

// Light setup
const LIGHT_POS: Point3<f32> = Point3::new(5.0, 5.0, -2.0);


#[derive(Copy, Clone, Debug)]
struct Material {
	color: Vec3<f32>,   // RGB [0..1]
	reflectivity: f32,  // 0.0 = matte, 1.0 = perfect mirror
}

#[derive(Copy, Clone, Debug)]
struct Object {
	shape: Shape3<f32>,
	material: Material,
}

#[derive(Clone, Debug)]
struct Scene {
	objects: Vec<Object>,
	light_pos: Point3<f32>,
}

impl Trace3<f32> for Scene {
	fn inside(&self, pt: Point3<f32>) -> bool {
		self.objects.iter().any(|object| object.shape.inside(pt))
	}

	fn trace(&self, ray: &Ray3<f32>) -> Option<Hit3<f32>> {
		ray.trace_collection(self.objects.iter().map(|object| &object.shape))
	}
}

struct Image {
	pixels: Vec<u8>,
	width: i32,
	height: i32,
}
impl Image {
	fn new(width: i32, height: i32) -> Image {
		let size = (width * height * 3) as usize;
		Image {
			pixels: vec![0; size],
			width,
			height,
		}
	}
	fn put(&mut self, x: i32, y: i32, color: Vec3<f32>) {
		if x < 0 || x >= self.width || y < 0 || y >= self.height {
			return;
		}
		let index = (y * self.width + x) as usize * 3;
		let Some(buf) = self.pixels.get_mut(index..index + 3) else { return };
		buf[0] = encode(color.x);
		buf[1] = encode(color.y);
		buf[2] = encode(color.z);
	}
}

fn encode(v: f32) -> u8 {
	let v = v.clamp(0.0, 1.0);

	let gamma_corrected = if v <= 0.0031308 {
		12.92 * v
	} else {
		1.055 * v.powf(1.0 / 2.4) - 0.055
	};

	(gamma_corrected * 255.0 + 0.5).floor() as u8
}

fn get_sky_color(ray: &Ray3<f32>) -> Vec3<f32> {
	let intensity = 1.0 - ray.direction.y;
	let color = Vec3::new(0.7, 0.6, 1.0) * intensity;
	return color;
}

fn ray_setup(
	image_width: i32,
	image_height: i32,
	x: i32,
	y: i32,
	fov_y: Angle<f32>,
	origin: Vec3<f32>,
	target: Vec3<f32>,
	ref_up: Vec3<f32>,
	jitter_x: f32,
	jitter_y: f32,
) -> Ray3<f32> {
	let forward = (target - origin).norm();
	let right = forward.cross(ref_up).norm();
	let up = right.cross(forward).norm();

	let aspect_ratio = image_width as f32 / image_height as f32;
	let viewport_height = 2.0 * (fov_y * 0.5).tan();
	let viewport_width = aspect_ratio * viewport_height;

	// Subpixel jitter
	let u = (x as f32 + 0.5 + jitter_x) / image_width as f32;
	let v = (y as f32 + 0.5 + jitter_y) / image_height as f32;

	let px = (u - 0.5) * viewport_width;
	let py = (0.5 - v) * viewport_height;

	let direction = (forward + right * px + up * py).norm();

	Ray3 { origin, direction, distance: f32::INFINITY }
}

fn trace_ray(ray: &Ray3<f32>, scene: &Scene, depth: u32) -> Vec3<f32> {
	if depth > MAX_DEPTH {
		return Vec3::new(0.0, 0.0, 0.0);
	}

	if let Some(hit) = ray.trace(&scene) {
		let material = &scene.objects[hit.index].material;
		let hit_pos = ray.at(hit.distance);

		let to_light = (scene.light_pos - hit_pos).norm();

		// Shadow check
		let shadow_ray = Ray3::new(hit_pos + hit.normal * f32::EPSILON, to_light, f32::INFINITY);
		let in_shadow = shadow_ray.trace(&scene).is_some();

		let ambient = 0.1;
		let diffuse = if in_shadow {
			ambient
		}
		else {
			ambient + hit.normal.dot(to_light).max(0.0)
		};

		let local_color = material.color * diffuse.min(1.0);

		// Reflection
		if material.reflectivity > 0.0 {
			let reflect_dir = (-ray.direction).reflect(hit.normal);
			let reflect_ray = Ray3::new(hit_pos + hit.normal * f32::EPSILON, reflect_dir.norm(), f32::INFINITY);
			let reflected_color = trace_ray(&reflect_ray, scene, depth + 1);

			return local_color * (1.0 - material.reflectivity) + reflected_color * material.reflectivity;
		}
		else {
			return local_color;
		}
	}

	// Background color (sky blue)
	return get_sky_color(&ray);
}

fn pixel(image_width: i32, image_height: i32, scene: &Scene, x: i32, y: i32) -> Vec3<f32> {
	let mut rng = urandom::new();

	let mut aa_color = Vec3f::ZERO;
	for _ in 0..if AA { AA_SAMPLES } else { 1 } {
		// Jitter for anti-aliasing
		let jitter_x: f32 = if AA { rng.range(-0.5..0.5) } else { 0.0 };
		let jitter_y: f32 = if AA { rng.range(-0.5..0.5) } else { 0.0 };

		// Compute the ray setup
		let mut ray = ray_setup(image_width, image_height, x, y, Angle::deg(FOV_Y), ORIGIN, TARGET, REF_UP, jitter_x, jitter_y);

		// Depth-of-field
		if DOF {
			let sensor_shift = Vec3(rng.range(-0.05..0.05), rng.range(-0.05..0.05), 0.0);
			ray.origin += sensor_shift;
			ray.direction = (ray.direction - sensor_shift * (1.0 / 4.0)).norm();
		}

		aa_color += trace_ray(&ray, scene, 0);
	}

	if AA {
		aa_color * (1.0 / AA_SAMPLES as f32)
	}
	else {
		aa_color
	}
}

// fn scene_render(image: &mut Image, scene: &Scene) {
// 	for y in 0..image.height {
// 		for x in 0..image.width {
// 			let color = pixel(image.width, image.height, &scene, x, y);
// 			image.put(x, y, color);
// 		}
// 	}
// }

fn scene_render(image: &mut Image, scene: &Scene) {
	let (sender, receiver) = mpsc::channel();

	let width = image.width;
	let height = image.height;

	// Required for rayon::spawn_fifo
	let scene = scene.clone();

	// Spawn parallel producer using rayon
	rayon::spawn_fifo(move || {
		(0..width * height)
			.into_par_iter()
			.for_each_with(sender, |sender, index| {
				let x = index % width;
				let y = index / width;
				let color = pixel(width, height, &scene, x, y);
				sender.send((x, y, color)).unwrap();
			});
	});

	// Main thread receives and writes to image buffer
	for _ in 0..(width * height) {
		let (x, y, color) = receiver.recv().unwrap();
		image.put(x, y, color);
	}
}

fn scene_save(path: &str, image: &Image) -> std::io::Result<()> {
	use std::fs::File;
	use std::io::{BufWriter, Write};
	let file = File::create(path)?;
	let mut writer = BufWriter::new(file);

	// Write P6 header
	writer.write_all(format!("P6\n{} {}\n255\n", image.width, image.height).as_bytes())?;

	// Write binary RGB data
	writer.write_all(&image.pixels)?;

	Ok(())
}

fn scene_create() -> Scene {
	let ground_shape = Plane3(Vec3::Y, 0.0);
	let sphere1_shape = Sphere { center: Point3::new(-1.5, 1.0, -5.0), radius: 1.0 };
	let sphere2_shape = Sphere { center: Point3::new(1.5, 2.0, -4.0), radius: 1.5 };
	let box_shape = Bounds3::point(Point3::new(2.8, 1.0, -0.5), Point3::dup(1.0));

	Scene {
		objects: vec![
			Object {
				shape: Shape3::Plane(ground_shape),
				material: Material {
					color: Vec3::new(0.6, 0.6, 0.6),
					reflectivity: 0.3,
				},
			},
			Object {
				shape: Shape3::Sphere(sphere1_shape),
				material: Material {
					color: Vec3::new(1.0, 0.2, 0.2),
					reflectivity: 0.5,
				},
			},
			Object {
				shape: Shape3::Sphere(sphere2_shape),
				material: Material {
					color: Vec3::new(0.2, 0.2, 1.0),
					reflectivity: 0.8,
				},
			},
			Object {
				shape: Shape3::Bounds(box_shape),
				material: Material {
					color: Vec3::new(1.0, 1.0, 0.2),
					reflectivity: 0.0,
				},
			},
		],
		light_pos: LIGHT_POS,
	}
}

fn main() {
	let mut image = Image::new(WIDTH, HEIGHT);
	let scene = scene_create();
	scene_render(&mut image, &scene);
	scene_save("raytracer.ppm", &image).expect("Failed to save image");
}
