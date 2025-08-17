use std::time;
use std::sync::mpsc;
use cvmath::*;
use rayon::prelude::*;

pub mod scenes;


// A small fudge factor to avoid self-intersection issues in ray tracing
const RAY_EPSILON: f32 = 0.001;

pub struct ImageSettings {
	pub width: i32,
	pub height: i32,
	pub nsamples: i32,
	pub max_bounces: i32,
	pub use_rayon: bool,
}

pub struct CameraSettings {
	pub origin: Vec3<f32>,
	pub target: Vec3<f32>,
	pub ref_up: Vec3<f32>,
	pub fov_y: Angle<f32>,
	pub dof_enabled: bool,
	pub aperture_radius: f32,
	pub focus_distance: f32,
}

pub enum Texture {
	None,
	Color(Vec3<f32>),
	Ground,
	Sky,
}
impl Texture {
	pub fn sample(&self, ray: &Ray3<f32>) -> Vec3<f32> {
		match self {
			Texture::None => Vec3::ZERO,
			&Texture::Color(color) => color,
			Texture::Ground => {
				let distance = -ray.origin.y / ray.direction.y;
				let i = ray.at(distance).map(f32::floor).hadd() as i32;
				if i % 2 != 0 {
					Vec3(1.0, 0.0, 0.0) // Red
				}
				else {
					Vec3(1.0, 1.0, 1.0) // White
				}
			}
			Texture::Sky => {
				let intensity = 1.0 - ray.direction.y;
				Vec3(0.7, 0.6, 1.0) * intensity
			}
		}
	}
}

pub struct Material {
	pub texture: Texture,
	pub reflectivity: f32,
	pub hardness: f32,
	pub diffuse_factor: f32,
	pub specular_factor: f32,
}
impl Default for Material {
	fn default() -> Self {
		Material {
			texture: Texture::Color(Vec3::ONE),
			reflectivity: 1.0,
			hardness: 1.0,
			diffuse_factor: 1.0,
			specular_factor: 1.0,
		}
	}
}

pub struct Object {
	pub shape: Shape3<f32>,
	pub material: u32,
}

pub struct Light {
	pub pos: Point3<f32>,
	pub color: Vec3<f32>,
	pub radius: f32,
}

pub struct World {
	pub ambient_light: f32,
	pub light: Light,
	pub materials: Vec<Material>,
	pub objects: Vec<Object>,
}
impl Trace3<f32> for World {
	fn inside(&self, pt: Point3<f32>) -> bool {
		self.objects.iter().any(|object| object.shape.inside(pt))
	}

	fn trace(&self, ray: &Ray3<f32>) -> Option<Hit3<f32>> {
		ray.trace_collection(self.objects.iter().map(|object| &object.shape))
	}
}

pub struct Scene {
	pub image: ImageSettings,
	pub camera: CameraSettings,
	pub world: World,
}

#[derive(Clone, Debug)]
pub struct Image {
	pub pixels: Vec<u8>,
	pub width: i32,
	pub height: i32,
}
impl Image {
	pub fn new(width: i32, height: i32) -> Image {
		let size = (width * height * 3) as usize;
		let pixels = vec![0; size];
		Image { pixels, width, height }
	}
	pub fn put(&mut self, x: i32, y: i32, color: Vec3<f32>) {
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

const SKY_MATERIAL: Material = Material {
	texture: Texture::Sky,
	reflectivity: 0.0,
	hardness: 0.0,
	diffuse_factor: 1.0,
	specular_factor: 0.0,
};

fn rng_circle(rng: &mut urandom::Random<impl urandom::Rng>, radius: f32) -> Vec2<f32> {
	let (s, c) = rng.range(0.0..std::f32::consts::PI * 2.0).sin_cos();
	let r = radius * (rng.range(0.0f32..1.0)).sqrt();
	Vec2(r * c, r * s)
}

fn ray_setup(scene: &Scene, x: i32, y: i32, rng: &mut urandom::Random<impl urandom::Rng>) -> Ray3<f32> {
	// Camera basis
	let forward = (scene.camera.target - scene.camera.origin).norm();
	let right = forward.cross(scene.camera.ref_up).norm();
	let up = right.cross(forward).norm();

	let aspect_ratio = scene.image.width as f32 / scene.image.height as f32;
	let viewport_height = 2.0 * (scene.camera.fov_y * 0.5).tan();
	let viewport_width = aspect_ratio * viewport_height;

	// Anti-aliasing jitter
	let (mut x, mut y) = (x as f32, y as f32);
	if scene.image.nsamples > 1 {
		x += rng.range(-0.5..0.5);
		y += rng.range(-0.5..0.5);
	}

	let u = (x + 0.5) / scene.image.width as f32;
	let v = (y + 0.5) / scene.image.height as f32;

	let px = (u - 0.5) * viewport_width;
	let py = (0.5 - v) * viewport_height;

	let mut origin = scene.camera.origin;
	let mut direction = (forward + right * px + up * py).norm();

	// Depth-of-field
	if scene.camera.dof_enabled {
		// Focus target point along the ray direction
		let pt = origin.mul_add(direction, scene.camera.focus_distance);

		// Random aperture offset in lens plane
		let lens_sample = rng_circle(rng, scene.camera.aperture_radius);
		let focus_offset = up * lens_sample.y + right * lens_sample.x;

		// Offset the ray origin and aim at the focus point
		origin += focus_offset;
		direction = (pt - origin).norm();
	}

	Ray3 { origin, direction, distance: Interval(RAY_EPSILON, f32::INFINITY) }
}

fn pixel_color(scene: &Scene, x: i32, y: i32) -> Vec3<f32> {
	let mut rng = urandom::new();

	let mut final_color = Vec3f::ZERO;
	let nsamples = scene.image.nsamples.max(1);

	for _ in 0..nsamples {
		let mut ray = ray_setup(scene, x, y, &mut rng);

		let mut ray_energy_left = 1.0;
		let mut ray_color = Vec3f::ZERO;

		for _ in 0..scene.image.max_bounces {
			let (material, color);

			if let Some(hit) = ray.trace(&scene.world) {
				let index = hit.index;
				let object = &scene.world.objects[index];
				material = &scene.world.materials[object.material as usize];
				let mtl_color = material.texture.sample(&ray);

				// Check if the hit point is lit by the light source
				let light = &scene.world.light;
				let light_at = light.pos + Vec3::from([(); 3].map(|_| rng.range(-light.radius..light.radius)));
				let light_dir = (light_at - hit.point).norm();
				let is_lit = Ray3(hit.point, light_dir, Interval(RAY_EPSILON, f32::INFINITY)).trace(&scene.world).is_none();

				if is_lit {
					// Blinn-Phong specular lighting model
					let ndotl = hit.normal.dot(light_dir).max(0.0);
					let half_vec = (light_dir - ray.direction).norm();
					let spec_angle = hit.normal.dot(half_vec).max(0.0);
					let specular = spec_angle.powf(material.hardness) * material.specular_factor;

					color =
						mtl_color * scene.world.ambient_light +
						mtl_color * ndotl * material.diffuse_factor +
						light.color * specular;
				}
				else {
					color = mtl_color * scene.world.ambient_light;
				}

				// Reflect the ray for the next bounce
				ray = ray.reflect(&hit);
			}
			else {
				material = &SKY_MATERIAL;
				color = material.texture.sample(&ray);
			}

			// Accumulate color
			ray_color = ray_color + (color * (ray_energy_left * (1.0 - material.reflectivity)));
			ray_energy_left *= material.reflectivity;
			if ray_energy_left <= 0.0 {
				break;
			}
		}

		final_color += ray_color;
	}

	return final_color * (1.0 / nsamples as f32);
}

struct ProgressReporter {
	timer: time::Instant,
}
impl ProgressReporter {
	fn new() -> Self {
		ProgressReporter { timer: time::Instant::now() }
	}

	fn report(&self, i: i32, total: i32) {
		if i & 0xfff == 0 || i == total {
			let progress = i as f64 / total as f64;
			let elapsed = self.timer.elapsed().as_secs_f64();
			let remaining = elapsed * (1.0 / progress - 1.0);
			print!("Rendering: {:.2}% - Elapsed: {:.1} sec - Remaining: {:.1} sec    \r", progress * 100.0, elapsed, remaining);
			if i == total {
				println!();
			}
			else {
				use std::io::{self, Write};
				let _ = io::stdout().flush();
			}
		}
	}
}

fn scene_render_slow(scene: Scene) -> Image {
	let mut image = Image::new(scene.image.width, scene.image.height);
	let pr = ProgressReporter::new();

	for y in 0..image.height {
		for x in 0..image.width {
			pr.report(y * image.width + x, image.width * image.height);

			let color = pixel_color(&scene, x, y);
			image.put(x, y, color);
		}
	}

	pr.report(image.width * image.height, image.width * image.height);
	return image;
}

fn scene_render_fast(scene: Scene) -> Image {
	let (sender, receiver) = mpsc::channel();

	let width = scene.image.width;
	let height = scene.image.height;

	// Spawn parallel producer using rayon
	rayon::spawn_fifo(move || {
		(0..width * height)
			.into_par_iter()
			.for_each_with(sender, |sender, index| {
				let x = index % width;
				let y = index / width;
				let color = pixel_color(&scene, x, y);
				sender.send((x, y, color)).unwrap();
			});
	});

	// Main thread receives and writes to image buffer
	let mut image = Image::new(width, height);
	let pr = ProgressReporter::new();

	for i in 0..width * height {
		pr.report(i, width * height);

		let (x, y, color) = receiver.recv().unwrap();
		image.put(x, y, color);
	}

	pr.report(width * height, width * height);
	return image;
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

fn main() {
	let (file_name, scene) = scenes::raytracing();
	let render = if scene.image.use_rayon { scene_render_fast } else { scene_render_slow };
	let image = render(scene);
	scene_save(file_name, &image).expect("Failed to save image");
}
