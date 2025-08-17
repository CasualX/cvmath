#![allow(dead_code)]

use super::*;

#[cfg(feature = "serde")]
pub mod dto;

fn add_material(materials: &mut Vec<Material>, material: Material) -> u32 {
	let id = materials.len() as u32;
	materials.push(material);
	return id;
}

pub fn raytracer() -> (&'static str, Scene) {
	let image = ImageSettings {
		width: 1600,
		height: 1200,
		nsamples: 256,
		max_bounces: 6,
		use_rayon: true,
	};
	let camera = CameraSettings {
		origin: Point3(0.0, 1.5, 0.0),
		target: Point3(0.0, 1.5, -1.0),
		ref_up: Vec3(0.0, 1.0, 0.0),
		fov_y: Angle::deg(90.0),
		dof_enabled: true,
		aperture_radius: 0.05,
		focus_distance: 4.0,
	};

	const M: Material = Material {
		texture: Texture::None,
		reflectivity: 0.0,
		hardness: 1.0,
		diffuse_factor: 1.0,
		specular_factor: 0.0,
	};

	let mut materials = Vec::new();
	let ground_material = add_material(&mut materials, Material {
		texture: Texture::Color(Vec3::new(0.6, 0.6, 0.6)),
		reflectivity: 0.3,
		..M
	});
	let sphere1_material = add_material(&mut materials, Material {
		texture: Texture::Color(Vec3::new(1.0, 0.2, 0.2)),
		reflectivity: 0.5,
		..M
	});
	let sphere2_material = add_material(&mut materials, Material {
		texture: Texture::Color(Vec3::new(0.2, 0.2, 1.0)),
		reflectivity: 0.8,
		..M
	});
	let bounds_material = add_material(&mut materials, Material {
		texture: Texture::Color(Vec3::new(1.0, 1.0, 0.2)),
		reflectivity: 0.0,
		..M
	});

	let ground_shape = Plane3(-Vec3f::Y, 0.0);
	let sphere1_shape = Sphere { center: Point3::new(-1.5, 1.0, -5.0), radius: 1.0 };
	let sphere2_shape = Sphere { center: Point3::new(1.5, 2.0, -4.0), radius: 1.5 };
	let bounds_shape = Bounds3::point(Point3::new(2.8, 1.0, -0.5), Point3::dup(1.0));

	let world = World {
		ambient_light: 0.1,
		light: Light {
			pos: Point3(5.0, 5.0, -2.0),
			color: Vec3(1.0, 1.0, 1.0),
			radius: 0.0,
		},
		materials,
		objects: vec![
			Object {
				shape: Shape3::Plane(ground_shape),
				material: ground_material,
			},
			Object {
				shape: Shape3::Sphere(sphere1_shape),
				material: sphere1_material,
			},
			Object {
				shape: Shape3::Sphere(sphere2_shape),
				material: sphere2_material,
			},
			Object {
				shape: Shape3::Bounds(bounds_shape),
				material: bounds_material,
			},
		],
	};

	let scene = Scene { image, camera, world };
	("raytracer.ppm", scene)
}


pub fn raytracing() -> (&'static str, Scene) {
	let image = ImageSettings {
		width: 1920 / 2,
		height: 1080 / 2,
		nsamples: 2048,
		max_bounces: 6,
		use_rayon: true,
	};
	let camera = CameraSettings {
		origin: Point3(0.0, 1.25, -4.0),
		target: Point3(0.0, 1.0, 0.0),
		ref_up: Vec3(0.0, 1.0, 0.0),
		fov_y: Angle::deg(40.0),
		dof_enabled: true,
		aperture_radius: 0.05,
		focus_distance: 4.0,
	};

	let mut materials = Vec::new();
	let ground_material = add_material(&mut materials, Material {
		texture: Texture::Ground,
		reflectivity: 0.0,
		diffuse_factor: 0.8,
		specular_factor: 0.0,
		..Default::default()
	});
	let sphere1_material = add_material(&mut materials, Material {
		texture: Texture::None,
		reflectivity: 0.95,
		diffuse_factor: 0.0,
		hardness: 100.0,
		..Default::default()
	});
	let sphere2_material = add_material(&mut materials, Material {
		texture: Texture::Color(Vec3(1.0, 0.65, 0.0)),
		reflectivity: 0.05,
		diffuse_factor: 0.9,
		specular_factor: 1.0,
		hardness: 99.0,
		..Default::default()
	});
	let octahedron_material = add_material(&mut materials, Material {
		diffuse_factor: 0.1,
		..Material::default()
	});

	let world = World {
		ambient_light: 0.3,
		light: Light {
			pos: Point3(0.0, 100.0, 0.0),
			color: Vec3(1.0, 1.0, 1.0),
			radius: 5.0,
		},
		materials,
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
				shape: Shape3::Plane(Plane3(-Vec3f::Y, 0.0)),
				material: ground_material,
			},

			Object {
				shape: Shape3::Sphere(Sphere(Vec3(-1.0, 2.0, 3.0), 0.5)),
				material: sphere1_material,
			},
			Object {
				shape: Shape3::Sphere(Sphere(Vec3(1.25, 0.8, 0.0), 0.25)),
				material: sphere2_material,
			},

			// Octahedron — Bottom half
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
					Vec3(-1.0, 1.0, 0.0),
				)),
				material: octahedron_material,
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(-1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
				)),
				material: octahedron_material,
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
					Vec3(1.0, 1.0, 0.0),
				)),
				material: octahedron_material,
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 0.0, 0.0),
					Vec3(1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
				)),
				material: octahedron_material,
			},

			// Octahedron — Top half
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(-1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
				)),
				material: octahedron_material,
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(0.0, 1.0, 1.0),
					Vec3(1.0, 1.0, 0.0),
				)),
				material: octahedron_material,
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(1.0, 1.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
				)),
				material: octahedron_material,
			},
			Object {
				shape: Shape3::Triangle(Triangle3::points(
					Vec3(0.0, 2.0, 0.0),
					Vec3(0.0, 1.0, -1.0),
					Vec3(-1.0, 1.0, 0.0),
				)),
				material: octahedron_material,
			},
		],
	};

	let scene = Scene { image, camera, world };
	("raytracing.ppm", scene)
}

pub fn all() -> Vec<(&'static str, Scene)> {
	vec![
		raytracer(),
		raytracing(),
	]
}
