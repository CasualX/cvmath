use std::fs::File;
use std::io::{BufWriter, Write};
use cvmath::*;

const MAX_DEPTH: u32 = 6; // For reflection bounce depth

enum Shape {
	Plane(Plane<f32>),
	Sphere(Sphere<f32>),
	Box(Bounds3<f32>),
}

impl TraceRay<f32> for Shape {
	fn inside(&self, ray: &Ray<f32>) -> bool {
		match self {
			Shape::Plane(plane) => plane.inside(ray),
			Shape::Sphere(sphere) => sphere.inside(ray),
			Shape::Box(bounds) => bounds.inside(ray),
		}
	}

	fn trace(&self, ray: &Ray<f32>, hits: &mut [TraceHit<f32>]) -> usize {
		match self {
			Shape::Plane(plane) => plane.trace(ray, hits),
			Shape::Sphere(sphere) => sphere.trace(ray, hits),
			Shape::Box(bounds) => bounds.trace(ray, hits),
		}
	}
}

#[derive(Copy, Clone)]
struct Material {
	color: Vec3<f32>,   // RGB [0..1]
	reflectivity: f32,  // 0.0 = matte, 1.0 = perfect mirror
}

struct Object {
	shape: Shape,
	material: Material,
}

struct Scene {
	objects: Vec<Object>,
	light_pos: Point3<f32>,
}

struct PixelBuffer {
	pixels: Vec<u8>,
	width: i32,
	height: i32,
}
impl PixelBuffer {
	fn new(width: i32, height: i32) -> PixelBuffer {
		let size = (width * height * 3) as usize;
		PixelBuffer {
			pixels: vec![0; size],
			width,
			height,
		}
	}
	fn index(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || x >= self.width || y < 0 || y >= self.height {
			return None;
		}
		Some((y * self.width + x) as usize * 3)
	}
}

// Vector get_sky_color(const Vector& ray_direction)
// {
//   //return {0, 0, 255};
//   return Vector{0.7, 0.6, 1.0}*255 * std::pow(1-ray_direction.y, 2);
// }

fn get_sky_color(ray: &Ray<f32>) -> Vec3<f32> {
	let intensity = 1.0 - ray.direction.y;
	let color = Vec3::new(0.7, 0.6, 1.0) * intensity;
	return color;
}


fn trace_ray(ray: Ray<f32>, scene: &Scene, depth: u32) -> Vec3<f32> {
	if depth > MAX_DEPTH {
		return Vec3::new(0.0, 0.0, 0.0);
	}

	let mut closest: Option<(TraceHit<f32>, &Material, Point3<f32>)> = None;
	let mut hit_data = [TraceHit::<f32>::default(); 4];

	for object in &scene.objects {
		let count = ray.trace(&object.shape, &mut hit_data);
		for i in 0..count {
			let hit = hit_data[i];
			if hit.distance > 0.0 {
				if closest.is_none() || hit.distance < closest.as_ref().unwrap().0.distance {
					let hit_pos = ray.at(hit.distance);
					closest = Some((hit, &object.material, hit_pos));
				}
			}
		}
	}

	if let Some((hit, material, hit_pos)) = closest {
		let normal = hit.normal;
		let to_light = (scene.light_pos - hit_pos).norm();

		// Shadow check
		let shadow_ray = Ray::new(hit_pos + normal * 0.001, to_light);
		let mut in_shadow = false;

		for object in &scene.objects {
			let mut shadow_hits = [TraceHit::<f32>::default(); 4];
			let count = shadow_ray.trace(&object.shape, &mut shadow_hits);
			for i in 0..count {
				if shadow_hits[i].distance > 0.0 && shadow_hits[i].distance < (scene.light_pos - hit_pos).len() {
					in_shadow = true;
					break;
				}
			}
			if in_shadow { break; }
		}

		let ambient = 0.1;
		let diffuse = if in_shadow {
			ambient
		}
		else {
			ambient + normal.dot(to_light).max(0.0)
		};

		let local_color = material.color * diffuse.min(1.0);

		// Reflection
		if material.reflectivity > 0.0 {
			let reflect_dir = ray.direction - normal * 2.0 * ray.direction.dot(normal);
			let reflect_ray = Ray::new(hit_pos + normal * 0.001, reflect_dir.norm());
			let reflected_color = trace_ray(reflect_ray, scene, depth + 1);

			return local_color * (1.0 - material.reflectivity) + reflected_color * material.reflectivity;
		}
		else {
			return local_color;
		}
	}

	// Background color (sky blue)
	return get_sky_color(&ray);
}

fn scene_render(buffer: &mut PixelBuffer) {
	let scene = {
		let ground_shape = Plane::from_point(Vec3::new(0.0, 1.0, 0.0), Point3::new(0.0, -1.0, 0.0));
		let sphere1_shape = Sphere { center: Point3::new(-1.5, 1.0, -5.0), radius: 1.0 };
		let sphere2_shape = Sphere { center: Point3::new(1.5, 2.0, -4.0), radius: 1.5 };
		let box_shape = Bounds3::point(
			Point3::new(-1.0, 1.5, -1.0),
			Point3::dup(1.0),
		);
		let light_pos = Point3::new(5.0, 5.0, -2.0);

		let objects = vec![
			Object {
				shape: Shape::Plane(ground_shape),
				material: Material {
					color: Vec3::new(0.6, 0.6, 0.6),
					reflectivity: 0.3,
				},
			},
			Object {
				shape: Shape::Sphere(sphere1_shape),
				material: Material {
					color: Vec3::new(1.0, 0.2, 0.2),
					reflectivity: 0.5,
				},
			},
			Object {
				shape: Shape::Sphere(sphere2_shape),
				material: Material {
					color: Vec3::new(0.2, 0.2, 1.0),
					reflectivity: 0.8,
				},
			},
			Object {
				shape: Shape::Box(box_shape),
				material: Material {
					color: Vec3::new(1.0, 1.0, 0.2),
					reflectivity: 0.0,
				},
			}
		];
		Scene { objects, light_pos }
	};

	let camera_pos = Point3::new(0.0, 1.5, 0.0);
	let fov = 90.0f32;
	let aspect_ratio = buffer.width as f32 / buffer.height as f32;
	let scale = (fov.to_radians() * 0.5).tan();

	for y in 0..buffer.height {
		for x in 0..buffer.width {
			let px = (((x as f32 + 0.5) / buffer.width as f32) * 2.0 - 1.0) * aspect_ratio * scale;
			let py = (1.0 - (y as f32 + 0.5) / buffer.height as f32 * 2.0) * scale;

			let ray_dir = Vec3::new(px, py, -1.0).norm();
			let ray = Ray::new(camera_pos, ray_dir);

			let color = trace_ray(ray, &scene, 0);
			let offset = buffer.index(x, y).expect("Invalid pixel index");

			buffer.pixels[offset + 0] = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
			buffer.pixels[offset + 1] = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
			buffer.pixels[offset + 2] = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
		}
	}
}

fn scene_save(path: &str, buffer: &PixelBuffer) -> std::io::Result<()> {
	let file = File::create(path)?;
	let mut writer = BufWriter::new(file);

	// Write P6 header
	writer.write_all(format!("P6\n{} {}\n255\n", buffer.width, buffer.height).as_bytes())?;

	// Write binary RGB data
	writer.write_all(&buffer.pixels)?;

	Ok(())
}

fn main() {
	let mut buffer = PixelBuffer::new(800, 600);
	scene_render(&mut buffer);
	scene_save("scene.ppm", &buffer).expect("Failed to save image");
	println!("Image written to scene.ppm");
}
