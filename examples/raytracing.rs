use cvmath::*;

fn white(_: &Ray<f32>) -> Vec3<f32> {
	Vec3::dup(1.0)
}

#[derive(Copy, Clone, Debug)]
struct Material {
	color: fn(&Ray<f32>) -> Vec3<f32>,
	reflectivity: f32,
	hardness: f32,
	diffuse_f: f32,
	specular_f: f32,
	roughness: f32,
}
impl Default for Material {
	fn default() -> Self {
		Material {
			color: white,
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

#[derive(Clone, Debug)]
struct Scene {
	objects: Vec<Object>,
	light_at: Vec3<f32>,
	light_color: Vec3<f32>,
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
		buf[0] = (color.x * 255.0) as u8;
		buf[1] = (color.y * 255.0) as u8;
		buf[2] = (color.z * 255.0) as u8;
	}
}

fn get_ground_color(ray: &Ray<f32>) -> Vec3<f32> {
	let distance = -ray.origin.y / ray.direction.y;
	let x = ray.origin.x + ray.direction.x * distance;
	let z = ray.origin.z + ray.direction.z * distance;

	if (x.abs().floor() as i32 % 2) == (z.abs().floor() as i32 % 2) {
		Vec3::new(1.0, 0.0, 0.0) // Red
	}
	else {
		Vec3::new(1.0, 1.0, 1.0) // White
	}
}

fn get_sky_color(ray: &Ray<f32>) -> Material {
	return Material {
		color: |ray| {
			let intensity = 1.0 - ray.direction.y;
			let color = Vec3::new(0.7, 0.6, 1.0) * intensity;
			return color;
		},
		reflectivity: 0.0,
		hardness: 0.0,
		specular_f: 0.0,
		..Default::default()
	};
}

// fn trace_ray(ray: &Ray<f32>) -> Material {
// 	if ray.direction.y < 0.0 {
// 		get_ground_color(ray)
// 	}
// 	else {
// 		get_sky_color(ray)
// 	}
// }

fn scene_render(image: &mut Image, scene: &Scene) {
	// Left-handed coordinate system
	const X: Vec3<f32> = Vec3(0.001, 0.0,   0.0); // X = right
	const Y: Vec3<f32> = Vec3(0.0,   0.001, 0.0); // Y = up
	const Z: Vec3<f32> = Vec3(0.0,   0.0,   1.0); // Z = forward

	const NSAMPLES: usize = 16;

	let mut hits = [TraceHit::default(); 16];
	let mut rng = urandom::new();

	for y in 0..image.height {
		for x in 0..image.width {

			let mut aa_color = Vec3f::ZERO;
			for _ in 0..NSAMPLES {

				let mut ray = {
					let origin = Point3(0.0, 1.0, -4.0);
					let direction = (
						X * ((x - image.width / 2) as f32 - 0.5 + rng.range(-0.5..0.5)) +
						Y * (-(y - image.height / 2) as f32 - 0.5 + rng.range(-0.5..0.5)) +
						Z).norm();
					Ray { origin, direction }
				};

				let mut final_color = Vec3::new(0.0, 0.0, 0.0);
				let mut ray_energy_left = 1.0;

				for i in 0..100 {
					let material;
					let mut color;

					let n_hits = ray.trace(scene, &mut hits);
					if n_hits > 0 {
						let hit = hits[..n_hits].iter().min_by(|a, b| a.distance.total_cmp(&b.distance)).unwrap().clone();
						let index = hit.index;
						material = scene.objects[index].material;
						color = (material.color)(&ray);

						// Reflect the ray
						ray.origin = ray.at(hit.distance) + hit.normal * 0.001;
						ray.direction = (-ray.direction).reflect(hit.normal);

						// if x == image.width / 2 && y == image.height / 2 {
						// 	dbg!(&hit);
						// }

						let is_lit = Ray(ray.origin, (scene.light_at - ray.origin).norm()).trace(scene, &mut hits) == 0;

						let ambient_light = 0.3;
						if is_lit {
							let diffuse_light = hit.normal.dot((scene.light_at - ray.origin).norm()).max(0.0);
							let specular_factor = (scene.light_at - ray.origin).norm().dot(ray.direction);
							color =
								color * ambient_light +
								color * diffuse_light * material.diffuse_f +
								scene.light_color * specular_factor.powf(material.hardness) * material.specular_f;
						}
						else {
							color = color * ambient_light;
						}
					}
					else {
						material = get_sky_color(&ray);
						color = (material.color)(&ray);
					}

					// if x == image.width / 2 && y == image.height / 2 {
					// 	dbg!(i, &material);
					// }

					final_color = final_color + (color * (ray_energy_left * (1.0 - material.reflectivity)));
					ray_energy_left *= material.reflectivity;
					if ray_energy_left <= 0.0 {
						break;
					}
				}

				aa_color += final_color * (1.0 / NSAMPLES as f32);
			}

			image.put(x, y, aa_color);
		}
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

fn main() {
	let scene = Scene {
		objects: vec![
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(-1.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Vec3(0.0, 1.73, 0.0))),
			// 	material: Material { color: Vec3(0.0, 0.0, 1.0), reflectivity: 0.5 },
			// },
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(2.0, 0.0, 2.0), Vec3(1.0, 1.73, 2.0), Vec3(0.0, 0.0, 2.0))),
			// 	material: Material { color: Vec3(0.0, 1.0, 0.0), reflectivity: 0.5 },
			// },
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(-0.25, 0.75, -1.0), Vec3(0.75, 0.75, -1.0), Vec3(0.25, 2.0, -1.0))),
			// 	material: Material { color: Vec3(1.0, 0.0, 0.0), reflectivity: 0.5 },
			// },

			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(-2.0, 0.0, -1.0), Vec3(2.0, 0.0, -1.0), Vec3(0.0, 3.0, -1.1))),
			// 	material: Material { color: Vec3(0.0, 0.0, 1.0), reflectivity: 0.5 },
			// },
			// Object {
			// 	shape: Shape3::Triangle(Triangle::points(Vec3(2.0, 0.0, -5.0), Vec3(-2.0, 0.0, -5.0), Vec3(0.0, 3.0, -4.9))),
			// 	material: Material { color: Vec3(0.0, 1.0, 0.0), reflectivity: 0.5 },
			// },

			Object {
				shape: Shape3::Plane(Plane(Vec3(0.0, 1.0, 0.0), 0.0)),
				material: Material {
					color: get_ground_color,
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
					color: |_| Vec3(0.0, 0.0, 0.0),
					reflectivity: 0.95,
					diffuse_f: 0.0,
					roughness: 0.75,
					..Default::default()
				},
			},
			Object {
				shape: Shape3::Sphere(Sphere(Vec3(-1.25, 0.8, 0.0), 0.25)),
				material: Material {
					color: |_| Vec3f(255.0, 165.0, 0.0) / 255.0,
					reflectivity: 0.25,
					diffuse_f: 0.9,
					specular_f: 1.0,
					hardness: 99.0,
					..Default::default()
				},
			},

			// Octahedron — Bottom half
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
					Vec3(-1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(-1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
					Vec3(1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},

			// Octahedron — Top half
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(-1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
					Vec3(1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
					Vec3(-1.0, 1.0, 0.0),
				)),
				material: Material { diffuse_f: 0.0, ..Material::default() },
			},
		],
		light_at: Vec3(0.0, 100.0, 0.0),
		light_color: Vec3(1.0, 1.0, 1.0),
	};

	let mut image = Image::new(1600, 1200);
	scene_render(&mut image, &scene);
	scene_save("raytracing.ppm", &image).expect("Failed to save image");
}
