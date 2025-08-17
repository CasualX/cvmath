use std::time;
use std::sync::mpsc;
use cvmath::*;
use rayon::prelude::*;

pub mod scenes;

pub struct EnvironmentLighting {
	pub sky_color_horizon: Vec3<f32>,
	pub sky_color_zenith: Vec3<f32>,
	pub sun_light_direction: Vec3<f32>,
	pub sun_focus: f32,
	pub sun_intensity: f32,
	pub ground_color: Vec3<f32>,
}

pub struct Material {
	pub color: Vec3<f32>,
	pub emissive: Vec3<f32>,
	pub roughness: f32, // 0.0 = mirror, 1.0 = diffuse
	pub metallic: f32, // 0.0 = dielectric, 1.0 = metal
}

pub struct Object {
	pub shape: Shape3<f32>,
	pub material: u32,
}

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

pub struct World {
	pub env_light: Option<EnvironmentLighting>,
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

// Gamma encode a color component value
fn encode(v: f32) -> u8 {
	let v = v.clamp(0.0, 1.0);

	let gamma_corrected = if v <= 0.0031308 { 12.92 * v }
	else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };

	(gamma_corrected * 255.0 + 0.5).floor() as u8
}

fn rng_circle(rng: &mut urandom::Random<impl urandom::Rng>, radius: f32) -> Vec2<f32> {
	let (s, c) = rng.range(0.0..std::f32::consts::PI * 2.0).sin_cos();
	let r = radius * (rng.next_f32() - 1.0).sqrt();
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

	Ray3 { origin, direction, distance: Interval(1e-4, f32::INFINITY) }
}

fn random_direction(rng: &mut urandom::Random<impl urandom::Rng>) -> Vec3<f32> {
	let distr = urandom::distr::StandardNormal;
	let x = rng.sample(&distr);
	let y = rng.sample(&distr);
	let z = rng.sample(&distr);
	Vec3(x, y, z).norm()
}

fn get_env_light(ray: &Ray3<f32>, env_light: &EnvironmentLighting) -> Vec3<f32> {
	fn smoothstep(x: f32, y: f32, t: f32) -> f32 {
		let t = ((t - x) / (y - x)).clamp(0.0, 1.0);
		t * t * (3.0 - 2.0 * t)
	}

	let sky_gradient_t = smoothstep(0.0, 0.4, ray.direction.y).powf(0.35);

	let sky_gradient = Vec3::lerp(env_light.sky_color_horizon, env_light.sky_color_zenith, sky_gradient_t);
	let sun = ray.direction.dot(-env_light.sun_light_direction).max(0.0).powf(env_light.sun_focus) * env_light.sun_intensity;

	let ground_to_sky_t = smoothstep(-0.01, 0.0, ray.direction.y);
	let sun_mask = if ground_to_sky_t >= 1.0 { Vec3::dup(sun) } else { Vec3::ZERO };

	return Vec3::lerp(env_light.ground_color, sky_gradient, ground_to_sky_t) + sun_mask;
}

fn fresnel_schlick(f0: Vec3f, cos_theta: f32) -> Vec3f {
	f0 + (Vec3f::ONE - f0) * (1.0 - cos_theta).powi(5)
}

fn pixel_color(scene: &Scene, x: i32, y: i32) -> Vec3<f32> {
	let mut rng = urandom::new();

	let mut total_incoming_light = Vec3f::ZERO;
	let nsamples = scene.image.nsamples.max(1);

	for _ in 0..nsamples {
		let mut ray = ray_setup(scene, x, y, &mut rng);
		if ray.inside(&scene.world) {
			if let Some(hit) = ray.trace(&scene.world) {
				ray.origin = ray.at(hit.distance);
				if !ray.inside(&scene.world) {
					continue; // Skip if still inside after stepping
				}
			}
		}

		let mut incoming_light = Vec3f::ZERO;
		let mut ray_color = Vec3f::ONE;

		for _ in 0..scene.image.max_bounces {
			if let Some(hit) = ray.trace(&scene.world) {
				let object = &scene.world.objects[hit.index];
				let material = &scene.world.materials[object.material as usize];

				// Choose reflection direction
				let diffuse_dir = (hit.normal + random_direction(&mut rng)).norm();
				let specular_dir = (-ray.direction).reflect(hit.normal);

				// Fresnel base reflectivity (F0)
				let f0 = Vec3::lerp(Vec3::dup(0.04), material.color, material.metallic);

				// Fresnel at this angle
				// let half_vec = (specular_dir - ray.direction).norm();
				// let cos_theta = (-ray.direction).dot(half_vec).max(0.0);
				let cos_theta = hit.normal.dot(specular_dir).max(0.0);
				let fresnel = fresnel_schlick(f0, cos_theta);

				// Diffuse term (only for non-metals)
				let diffuse_color = material.color * (1.0 - material.metallic);

				// Sample reflection direction based on Fresnel
				if rng.next_f32() < 1.0 + fresnel.vmax() {
					// Specular bounce with roughness-based sampling
					let alpha = material.roughness * material.roughness;
					let jittered_specular = (specular_dir + random_direction(&mut rng) * alpha).norm();
					ray.origin = hit.point;
					ray.direction = jittered_specular;
					ray_color *= fresnel;
				}
				else {
					// Diffuse bounce
					ray.origin = hit.point;
					ray.direction = diffuse_dir;
					ray_color *= diffuse_color;
				}

				// Emission
				incoming_light += material.emissive * ray_color;
			}
			else if let Some(env_light) = &scene.world.env_light {
				incoming_light += get_env_light(&ray, env_light) * ray_color;
				break;
			}
			else {
				incoming_light = Vec3f::ZERO;
				break;
			}
		}

		total_incoming_light += incoming_light;
	}

	return total_incoming_light * (1.0 / nsamples as f32);
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
	// let (file_name, scene) = scenes::cornell_box();
	for (file_name, scene) in scenes::all() {
		let render = if scene.image.use_rayon { scene_render_fast } else { scene_render_slow };
		let image = render(scene);
		scene_save(file_name, &image).unwrap();
	}
}
