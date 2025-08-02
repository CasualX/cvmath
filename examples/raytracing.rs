use std::sync::mpsc;
use cvmath::*;
use rayon::prelude::*;


// Image setup
const WIDTH: i32 = 800;
const HEIGHT: i32 = 1200;

// Camera setup
const ORIGIN: Point3<f32> = Point3(0.0, 1.0, -4.0);
const TARGET: Point3<f32> = Point3(0.0, 1.0, 0.0);
const UP: Vec3<f32> = Vec3(0.0, 1.0, 0.0);
const FOV_Y: f32 = 60.0;

// Quality setup
const DOF: bool = true; // Enable depth-of-field
const AA: bool = true; // Enable anti-aliasing
const AA_SAMPLES: usize = 512; // Number of anti-aliasing samples per pixel
const MAX_BOUNCES: u32 = 6; // Maximum reflection bounce depth

// Light setup
const LIGHT: Light = Light {
	pos: Point3(0.0, 100.0, 0.0),
	color: Vec3(1.0, 1.0, 1.0),
	radius: 5.0,
};


#[derive(Copy, Clone, Debug)]
enum Texture {
	None,
	Color(Vec3<f32>),
	Ground,
	Sky,
}
impl Texture {
	fn sample(&self, ray: &Ray<f32>) -> Vec3<f32> {
		match self {
			Texture::None => Vec3::ZERO,
			&Texture::Color(color) => color,
			Texture::Ground => {
				let distance = -ray.origin.y / ray.direction.y;
				let i = ray.at(distance).map(f32::floor).hadd() as i32;
				if i % 2 == 0 {
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

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
struct Material {
	texture: Texture,
	reflectivity: f32,
	hardness: f32,
	diffuse_f: f32,
	specular_f: f32,
	roughness: f32,
}
impl Default for Material {
	fn default() -> Self {
		Material {
			texture: Texture::Color(Vec3::ONE),
			reflectivity: 1.0,
			hardness: 1.0,
			diffuse_f: 1.0,
			specular_f: 1.0,
			roughness: 0.0,
		}
	}
}

#[derive(Copy, Clone, Debug)]
struct Object {
	shape: Shape3<f32>,
	material: Material,
}

#[derive(Copy, Clone, Debug)]
struct Light {
	pos: Point3<f32>,
	color: Vec3<f32>,
	radius: f32,
}

#[derive(Clone, Debug)]
struct Scene {
	objects: Vec<Object>,
	light: Light,
}
impl TraceRay<f32> for Scene {
	fn inside(&self, ray: &Ray<f32>) -> bool {
		self.objects.iter().any(|object| object.shape.inside(ray))
	}

	fn trace(&self, ray: &Ray<f32>, hits: &mut [TraceHit<f32>]) -> usize {
		let mut count = 0;
		for (index, object) in self.objects.iter().enumerate() {
			let n = object.shape.trace(ray, &mut hits[count..]);
			for hit in &mut hits[count..count + n] {
				hit.index = index;
			}
			count += n;
		}
		return count;
	}
}

#[derive(Clone, Debug)]
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

const SKY_MATERIAL: Material = Material {
	texture: Texture::Sky,
	reflectivity: 0.0,
	hardness: 0.0,
	diffuse_f: 1.0,
	specular_f: 0.0,
	roughness: 0.0,
};

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
) -> Ray<f32> {
	let forward = (target - origin).norm();
	let right = ref_up.cross(forward).norm();
	let up = right.cross(forward).norm();

	let aspect_ratio = image_width as f32 / image_height as f32;
	let viewport_height = 2.0 * (fov_y * 0.5).tan();
	let viewport_width = aspect_ratio * viewport_height;

	// Subpixel jitter
	let u = (x as f32 + 0.5 + jitter_x) / image_width as f32;
	let v = (y as f32 + 0.5 + jitter_y) / image_height as f32;

	let px = (u - 0.5) * viewport_width;
	let py = (v - 0.5) * viewport_height;

	let direction = (forward + right * px + up * py).norm();

	Ray { origin, direction }
}

fn trace_ray(image_width: i32, image_height: i32, scene: &Scene, x: i32, y: i32) -> Vec3<f32> {
	let mut hits = [TraceHit::default(); 16];
	let mut rng = urandom::new();

	let mut aa_color = Vec3f::ZERO;
	for _ in 0..if AA { AA_SAMPLES } else { 1 } {
		// Compute the ray setup
		let jitter_x: f32 = if AA { rng.range(-0.5..0.5) } else { 0.0 };
		let jitter_y: f32 = if AA { rng.range(-0.5..0.5) } else { 0.0 };
		let mut ray = ray_setup(image_width, image_height, x, y, Angle::deg(FOV_Y), ORIGIN, TARGET, UP, jitter_x, jitter_y);

		// Depth-of-field
		if DOF {
			let sensor_shift = Vec3(rng.range(-0.05..0.05), rng.range(-0.05..0.05), 0.0);
			ray.origin += sensor_shift;
			ray.direction = (ray.direction - sensor_shift * (1.0 / 4.0)).norm();
		}

		let mut final_color = Vec3::new(0.0, 0.0, 0.0);
		let mut ray_energy_left = 1.0;

		for _ in 0..MAX_BOUNCES {
			let material;
			let mut color;

			let n_hits = ray.trace(scene, &mut hits);
			if n_hits > 0 {
				let hit = hits[..n_hits].iter().min_by(|a, b| a.distance.total_cmp(&b.distance)).unwrap().clone();
				let index = hit.index;
				material = scene.objects[index].material;
				color = material.texture.sample(&ray);

				// Reflect the ray
				ray.origin = ray.at(hit.distance) + hit.normal * 0.001;
				ray.direction = (-ray.direction).reflect(hit.normal);

				// Check if the hit point is lit by the light source
				let light_at = if AA { scene.light.pos + Vec3::from([(); 3].map(|_| rng.range(-scene.light.radius..scene.light.radius))) } else { scene.light.pos };
				let is_lit = Ray(ray.origin, (light_at - ray.origin).norm()).trace(scene, &mut hits) == 0;

				// Calculate lighting
				let ambient_light = 0.3;
				if is_lit {
					let diffuse_light = hit.normal.dot((light_at - ray.origin).norm()).max(0.0);
					let specular_factor = (light_at - ray.origin).norm().dot(ray.direction);
					color =
						color * ambient_light +
						color * diffuse_light * material.diffuse_f +
						scene.light.color * specular_factor.powf(material.hardness) * material.specular_f;
				}
				else {
					color = color * ambient_light;
				}
			}
			else {
				material = SKY_MATERIAL;
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
// 			let color = trace_ray(image.width, image.height, scene, x, y);
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
			.for_each_with(sender, |s, i| {
				let x = i % width;
				let y = i / width;
				let color = trace_ray(width, height, &scene, x, y);
				s.send((x, y, color)).unwrap();
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
	Scene {
		objects: vec![
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(-1.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Vec3(0.0, 1.73, 0.0))),
			// 	material: Material { color: |_| Vec3(0.0, 0.0, 1.0), reflectivity: 0.5, ..Default::default() },
			// },
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(2.0, 0.0, 2.0), Vec3(1.0, 1.73, 2.0), Vec3(0.0, 0.0, 2.0))),
			// 	material: Material { color: |_| Vec3(0.0, 1.0, 0.0), reflectivity: 0.5, ..Default::default() },
			// },
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(-0.25, 0.75, -1.0), Vec3(0.75, 0.75, -1.0), Vec3(0.25, 2.0, -1.0))),
			// 	material: Material { color: |_| Vec3(1.0, 0.0, 0.0), reflectivity: 0.5, ..Default::default() },
			// },

			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(-2.0, 0.0, -1.0), Vec3(2.0, 0.0, -1.0), Vec3(0.0, 3.0, -1.1))),
			// 	material: Material { color: |_| Vec3(0.0, 0.0, 1.0), reflectivity: 0.5, ..Default::default() },
			// },
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(2.0, 0.0, -5.0), Vec3(-2.0, 0.0, -5.0), Vec3(0.0, 3.0, -4.9))),
			// 	material: Material { color: |_| Vec3(0.0, 1.0, 0.0), reflectivity: 0.5, ..Default::default() },
			// },

			Object {
				shape: Shape3::Plane(Plane(Vec3(0.0, 1.0, 0.0), 0.0)),
				material: Material {
					texture: Texture::Ground,
					reflectivity: 0.0,
					diffuse_f: 0.8,
					specular_f: 0.0,
					roughness: 0.0,
					..Default::default()
				},
			},

			Object {
				shape: Shape3::Sphere(Sphere(Vec3(1.0, 2.0, 3.0), 0.5)),
				material: Material {
					texture: Texture::None,
					reflectivity: 0.95,
					diffuse_f: 0.0,
					roughness: 0.75,
					..Default::default()
				},
			},
			Object {
				shape: Shape3::Sphere(Sphere(Vec3(-1.25, 0.8, 0.0), 0.25)),
				material: Material {
					texture: Texture::Color(Vec3f(255.0, 165.0, 0.0) / 255.0),
					reflectivity: 0.05,
					diffuse_f: 0.9,
					specular_f: 1.0,
					hardness: 99.0,
					..Default::default()
				},
			},

			// Octahedron — Bottom half
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
					Vec3(-1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(-1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
					Vec3(1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},

			// Octahedron — Top half
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(-1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
					Vec3(1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
					Vec3(-1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
		],
		light: LIGHT,
	}
}

fn main() {
	let mut image = Image::new(WIDTH, HEIGHT);
	let scene = scene_create();
	scene_render(&mut image, &scene);
	scene_save("raytracing.ppm", &image).expect("Failed to save image");
}
