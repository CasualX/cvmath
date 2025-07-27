use cvmath::*;

struct Material {
	color: Vec3<f32>,
}

struct Object {
	shape: Shape3<f32>,
	material: Material,
}

struct Scene {
	objects: Vec<Object>,
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

fn get_sky_color(ray: &Ray<f32>) -> Vec3<f32> {
	let intensity = 1.0 - ray.direction.y;
	let color = Vec3::new(0.7, 0.6, 1.0) * intensity;
	return color;
}

fn trace_ray(ray: &Ray<f32>) -> Vec3<f32> {
	if ray.direction.y < 0.0 {
		get_ground_color(ray)
	}
	else {
		get_sky_color(ray)
	}
}

fn scene_render(image: &mut Image, scene: &Scene) {
	// Left-handed coordinate system
	const X: Vec3<f32> = Vec3(0.002, 0.0,   0.0); // X = right
	const Y: Vec3<f32> = Vec3(0.0,   0.002, 0.0); // Y = up
	const Z: Vec3<f32> = Vec3(0.0,   0.0,   1.0); // Z = forward

	let mut hits = [TraceHit::default(); 16];

	for y in 0..image.height {
		for x in 0..image.width {
			let ray = {
				let origin = Point3(0.0, 1.0, -4.0);
				let direction = (X * ((x - image.width / 2) as f32 - 0.5) + Y * (-(y - image.height / 2) as f32 - 0.5) + Z).norm();
				Ray { origin, direction }
			};

			let n_hits = ray.trace(scene, &mut hits);
			let color = if n_hits > 0 {
				let hit = hits[..n_hits].iter().min_by(|a, b| a.distance.total_cmp(&b.distance)).unwrap();
				let index = hit.index;
				scene.objects[index].material.color
			}
			else {
				trace_ray(&ray)
			};

			image.put(x, y, color);
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
			Object {
				shape: Shape3::Triangle(Triangle::points(Vec3(-1.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Vec3(0.0, 1.73, 0.0))),
				material: Material { color: Vec3(0.0, 0.0, 1.0) },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(Vec3(2.0, 0.0, 2.0), Vec3(1.0, 1.73, 2.0), Vec3(0.0, 0.0, 2.0))),
				material: Material { color: Vec3(0.0, 1.0, 0.0) },
			},
			Object {
				shape: Shape3::Triangle(Triangle::points(Vec3(-0.25, 0.75, -1.0), Vec3(0.75, 0.75, -1.0), Vec3(0.25, 2.0, -1.0))),
				material: Material { color: Vec3(1.0, 0.0, 0.0) },
			}
		]
	};
	let mut image = Image::new(1600, 1200);
	scene_render(&mut image, &scene);
	scene_save("raytracing.ppm", &image).expect("Failed to save image");
}
