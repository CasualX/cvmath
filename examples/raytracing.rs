use std::sync::mpsc;
use cvmath::*;
use rayon::prelude::*;

pub mod scenes;


// A small fudge factor to avoid self-intersection issues in ray tracing
const RAY_EPSILON: f32 = 0.001;

#[derive(Clone, Debug)]
pub struct Settings {
	pub width: i32,
	pub height: i32,
	pub rayon: bool,
	pub fov_y: Angle<f32>,
	pub origin: Vec3<f32>,
	pub target: Vec3<f32>,
	pub ref_up: Vec3<f32>,
	pub dof: bool,
	pub aa: bool,
	pub aa_samples: usize,
	pub max_bounces: u32,
	pub ambient_light: f32,
}


#[derive(Clone, Debug)]
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
					Vec3::new(1.0, 0.0, 0.0) // Red
				}
				else {
					Vec3::new(1.0, 1.0, 1.0) // White
				}
			}
			Texture::Sky => {
				let intensity = 1.0 - ray.direction.y;
				Vec3::new(0.7, 0.6, 1.0) * intensity
			}
		}
	}
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Material {
	pub texture: Texture,
	pub reflectivity: f32,
	pub hardness: f32,
	pub diffuse_factor: f32,
	pub specular_factor: f32,
	pub roughness: f32,
}
impl Default for Material {
	fn default() -> Self {
		Material {
			texture: Texture::Color(Vec3::ONE),
			reflectivity: 1.0,
			hardness: 1.0,
			diffuse_factor: 1.0,
			specular_factor: 1.0,
			roughness: 0.0,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Object {
	pub shape: Shape3<f32>,
	pub material: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Light {
	pub pos: Point3<f32>,
	pub color: Vec3<f32>,
	pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct Scene {
	pub materials: Vec<Material>,
	pub objects: Vec<Object>,
	pub light: Light,
}
impl Trace3<f32> for Scene {
	fn inside(&self, pt: Point3<f32>) -> bool {
		self.objects.iter().any(|object| object.shape.inside(pt))
	}

	fn trace(&self, ray: &Ray3<f32>) -> Option<Hit3<f32>> {
		ray.trace_collection(self.objects.iter().map(|object| &object.shape))
	}
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
		Image {
			pixels: vec![0; size],
			width,
			height,
		}
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
	roughness: 0.0,
};

fn ray_setup(conf: &Settings, x: f32, y: f32) -> Ray3<f32> {
	let forward = (conf.target - conf.origin).norm();
	let right = forward.cross(conf.ref_up).norm();
	let up = right.cross(forward).norm();

	let aspect_ratio = conf.width as f32 / conf.height as f32;
	let viewport_height = 2.0 * (conf.fov_y * 0.5).tan();
	let viewport_width = aspect_ratio * viewport_height;

	let u = (x + 0.5) / conf.width as f32;
	let v = (y + 0.5) / conf.height as f32;

	let px = (u - 0.5) * viewport_width;
	let py = (0.5 - v) * viewport_height;

	let origin = conf.origin;
	let direction = (forward + right * px + up * py).norm();

	Ray3 { origin, direction, distance: f32::INFINITY }
}

fn trace_ray(conf: &Settings, scene: &Scene, x: i32, y: i32) -> Vec3<f32> {
	let mut rng = urandom::new();

	let mut aa_color = Vec3f::ZERO;
	for _ in 0..if conf.aa { conf.aa_samples } else { 1 } {
		// Anti-aliasing jitter
		let jx: f32 = if conf.aa { rng.range(-0.5..0.5) } else { 0.0 };
		let jy: f32 = if conf.aa { rng.range(-0.5..0.5) } else { 0.0 };
		let x = x as f32 + jx;
		let y = y as f32 + jy;

		// Compute the ray setup
		let mut ray = ray_setup(conf, x, y);

		// Depth-of-field
		if conf.dof {
			let sensor_shift = Vec3(rng.range(-0.05..0.05), rng.range(-0.05..0.05), 0.0);
			ray.origin += sensor_shift;
			ray.direction = (ray.direction - sensor_shift * (1.0 / 4.0)).norm();
		}

		let mut final_color = Vec3::new(0.0, 0.0, 0.0);
		let mut ray_energy_left = 1.0;

		for _ in 0..conf.max_bounces {
			let (material, color);

			if let Some(hit) = ray.trace(scene) {
				let index = hit.index;
				let material_index = scene.objects[index].material as usize;
				material = &scene.materials[material_index];
				let mtl_color = material.texture.sample(&ray);
				let hit_point = ray.at(hit.distance);

				// Check if the hit point is lit by the light source
				let light = &scene.light;
				let light_at = light.pos + Vec3::from([(); 3].map(|_| rng.range(-light.radius..light.radius)));
				let light_dir = (light_at - hit_point).norm();
				let is_lit = Ray3(hit_point, light_dir, f32::INFINITY).step(RAY_EPSILON).trace(scene).is_none();

				if is_lit {
					// Blinn-Phong specular lighting model
					let ndotl = hit.normal.dot(light_dir).max(0.0);
					let half_vec = (light_dir - ray.direction).norm();
					let spec_angle = hit.normal.dot(half_vec).max(0.0);
					let specular = spec_angle.powf(material.hardness) * material.specular_factor;

					color =
						mtl_color * conf.ambient_light +
						mtl_color * ndotl * material.diffuse_factor +
						light.color * specular;
				}
				else {
					color = mtl_color * conf.ambient_light;
				}

				// Reflect the ray for the next bounce
				ray = Ray3(hit_point, (-ray.direction).reflect(hit.normal), f32::INFINITY).step(RAY_EPSILON);
			}
			else {
				material = &SKY_MATERIAL;
				color = material.texture.sample(&ray);
			}

			// Accumulate color
			final_color = final_color + (color * (ray_energy_left * (1.0 - material.reflectivity)));
			ray_energy_left *= material.reflectivity;
			if ray_energy_left <= 0.0 {
				break;
			}
		}

		aa_color += final_color;
	}

	if conf.aa {
		aa_color * (1.0 / conf.aa_samples as f32)
	}
	else {
		aa_color
	}
}

fn scene_render_slow(conf: Settings, scene: Scene) -> Image {
	let mut image = Image::new(conf.width, conf.height);
	for y in 0..image.height {
		for x in 0..image.width {
			let color = trace_ray(&conf, &scene, x, y);
			image.put(x, y, color);
		}
	}
	return image;
}

fn scene_render_fast(conf: Settings, scene: Scene) -> Image {
	let (sender, receiver) = mpsc::channel();

	let width = conf.width;
	let height = conf.height;

	// Spawn parallel producer using rayon
	rayon::spawn_fifo(move || {
		(0..width * height)
			.into_par_iter()
			.for_each_with(sender, |sender, index| {
				let x = index % width;
				let y = index / width;
				let color = trace_ray(&conf, &scene, x, y);
				sender.send((x, y, color)).unwrap();
			});
	});

	// Main thread receives and writes to image buffer
	let mut image = Image::new(width, height);
	for _ in 0..(width * height) {
		let (x, y, color) = receiver.recv().unwrap();
		image.put(x, y, color);
	}
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
	let (conf, scene) = scenes::raytracing();
	let image = if conf.rayon {
		scene_render_fast(conf, scene)
	}
	else {
		scene_render_slow(conf, scene)
	};
	scene_save("raytracing.ppm", &image).expect("Failed to save image");
}
