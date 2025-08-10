use super::*;

pub fn pathtracing() -> (&'static str, Scene) {
	let scene = Scene {
		image: ImageSettings {
			width: 1920 / 2,
			height: 1080 / 2,
			nsamples: 200,
			max_bounces: 10,
			use_rayon: true,
		},
		camera: CameraSettings {
			origin: Vec3(0.0, 0.0, 4.0),
			target: Vec3(0.0, -1.0, 0.0),
			ref_up: Vec3(0.0, 1.0, 0.0),
			fov_y: Angle::deg(90.0),
			dof_enabled: true,
			aperture_radius: 0.05,
			focus_distance: 2.0,
		},
		world: World {
			env_light: Some(EnvironmentLighting {
				sky_color_horizon: Vec3(0.5, 0.7, 1.0),
				sky_color_zenith: Vec3(0.2, 0.2, 0.5),
				sun_light_direction: Vec3(0.0, -1.0, -1.0).norm(),
				sun_focus: 16.0,
				sun_intensity: 20.0,
				ground_color: Vec3(0.2, 0.3, 0.1),
			}),
			materials: vec![
				Material {
					color: Vec3(0.8, 0.6, 0.2),
					emissive: Vec3::ZERO,
					roughness: 0.1,
					metallic: 1.0,
				},
				Material {
					color: Vec3(0.91, 0.08, 0.08), // Red sphere
					emissive: Vec3::ZERO,
					roughness: 0.4,
					metallic: 0.4,
				},
				Material {
					color: Vec3(0.02, 0.19, 0.84), // Blue sphere
					emissive: Vec3::ZERO,
					roughness: 0.3,
					metallic: 0.15,
				},
				Material {
					color: Vec3(0.5, 0.0, 0.5), // Purpleish sphere
					emissive: Vec3::ZERO,
					roughness: 0.7,
					metallic: 0.02,
				},
				Material {
					color: Vec3::dup(0.8),
					emissive: Vec3::dup(10.0),
					roughness: 1.0,
					metallic: 0.0,
				},
			],
			objects: vec![
				Object {
					shape: Shape3::Sphere(Sphere {
						center: Vec3::ZERO,
						radius: 2.0,
					}),
					material: 0,
				},
				Object {
					shape: Shape3::Sphere(Sphere {
						center: Vec3(-1.0, -1.0, 2.0),
						radius: 0.5,
					}),
					material: 1,
				},
				Object {
					shape: Shape3::Sphere(Sphere {
						center: Vec3(1.0, 0.0, 2.0),
						radius: 0.5,
					}),
					material: 2,
				},
				Object {
					shape: Shape3::Sphere(Sphere {
						center: Vec3(0.0, -52.0, 0.0),
						radius: 50.0,
					}),
					material: 3,
				},
				Object {
					shape: Shape3::Sphere(Sphere {
						center: Vec3(400.0, 100.0, 0.0),
						radius: 200.0,
					}),
					material: 4,
				},
			],
		},
	};
	("pathtracing.ppm", scene)
}

pub fn random_spheres() -> (&'static str, Scene) {
	let image = ImageSettings {
		width: 1920 / 2,
		height: 1080 / 2,
		nsamples: 512,
		max_bounces: 10,
		use_rayon: true,
	};
	let camera = CameraSettings {
		origin: Vec3(0.0, 5.0, 5.0),
		target: Vec3(0.0, 0.0, 0.0),
		ref_up: Vec3(0.0, 1.0, 0.0),
		fov_y: Angle::deg(90.0),
		dof_enabled: true,
		aperture_radius: 0.01,
		focus_distance: f32::sqrt(5.0 * 5.0 + 5.0 * 5.0),
	};
	let env_light = EnvironmentLighting {
		sky_color_horizon: Vec3(0.7, 0.8, 1.0),
		sky_color_zenith: Vec3(0.2, 0.4, 0.8),
		sun_light_direction: Vec3(1.0, -1.0, -1.0).norm(),
		sun_focus: 16.0,
		sun_intensity: 10.0,
		ground_color: Vec3(0.15, 0.15, 0.15),
	};
	let mut materials = Vec::new();
	let mut objects = Vec::new();
	// Add random spheres with random materials and sizes
	let mut rng = urandom::new();
	for i in 0..100 {
		let material = Material {
			color: Vec3(rng.next_f32() - 1.0, rng.next_f32() - 1.0, rng.next_f32() - 1.0),
			emissive: Vec3::ZERO,
			roughness: rng.next_f32() - 1.0,
			metallic: rng.next_f32() - 1.0,
		};
		materials.push(material);

		let radius = rng.range(0.1..1.0);
		let z = rng.range(-5.0..5.0);
		let x = rng.range(-4.0-(5.0-z)..4.0+(5.0-z));
		let position = Vec3(x, radius, z);
		let object = Object {
			shape: Shape3::Sphere(Sphere { center: position, radius }),
			material: i,
		};
		objects.push(object);
	}
	let scene = Scene { image, camera, world: World { env_light: Some(env_light), materials, objects } };
	("random_spheres.ppm", scene)
}

pub fn cornell_box() -> (&'static str, Scene) {
	let image = ImageSettings {
		width: 512,
		height: 512,
		nsamples: 256,
		max_bounces: 10,
		use_rayon: true,
	};
	let camera = CameraSettings {
		origin: Vec3(5.0, 5.0, -5.0),
		target: Vec3(5.0, 5.0, 5.0),
		ref_up: Vec3(0.0, 1.0, 0.0),
		fov_y: Angle::deg(90.0),
		dof_enabled: true,
		aperture_radius: 0.1,
		focus_distance: 10.0,
	};
	let mut materials = Vec::new();
	let mut objects = Vec::new();
	let mut add_object = |shape: Shape3<f32>, material: Material| {
		let object = Object { shape, material: materials.len() as u32 };
		objects.push(object);
		materials.push(material);
	};

	let diffuse = Material {
		color: Vec3::ONE,
		emissive: Vec3::ZERO,
		roughness: 1.0,
		metallic: 0.0,
	};

	add_object(Shape3::Plane(Plane3( Vec3f::X,  0.0)), Material { color: Vec3(0.8, 0.0, 0.0), ..diffuse }); // Red wall
	add_object(Shape3::Plane(Plane3(-Vec3f::X, 10.0)), Material { color: Vec3(0.15, 0.40, 0.7), ..diffuse }); // Blue wall
	add_object(Shape3::Plane(Plane3(-Vec3f::Z, 10.0)), Material { color: Vec3(0.5, 0.5, 0.5), ..diffuse }); // Back wall
	add_object(Shape3::Plane(Plane3( Vec3f::Z,  0.0)), Material { color: Vec3(0.2, 0.2, 0.2), ..diffuse }); // Front wall
	add_object(Shape3::Plane(Plane3(-Vec3f::Y, 10.0)), Material { color: Vec3(0.9, 0.9, 0.9), ..diffuse }); // Ceiling
	add_object(Shape3::Plane(Plane3( Vec3f::Y,  0.0)), Material { color: Vec3(0.13, 0.5, 0.11), ..diffuse }); // Green Floor

	// Light source
	add_object(
		Shape3::Bounds(Bounds3::point(Vec3(5.0, 10.0, 5.0), Vec3(2.0, 0.1, 2.0))),
		Material { color: Vec3f::ONE, emissive: Vec3f(10.0, 10.0, 10.0), roughness: 0.0, metallic: 0.0 },
	);

	// Two metallic spheres
	add_object(
		Shape3::Sphere(Sphere { center: Vec3(3.0, 3.0, 3.0), radius: 2.0 }),
		Material { color: Vec3(0.8, 0.6, 0.2), emissive: Vec3::ZERO, roughness: 0.1, metallic: 0.5 },
	);
	add_object(
		Shape3::Sphere(Sphere { center: Vec3(7.0, 3.0, 7.0), radius: 3.0 }),
		Material { color: Vec3f::ONE, emissive: Vec3f::ZERO, roughness: 0.0, metallic: 1.0 },
	);

	let scene = Scene { image, camera, world: World { env_light: None, materials, objects } };
	("cornell_box.ppm", scene)
}

pub fn all() -> Vec<(&'static str, Scene)> {
	vec![
		pathtracing(),
		random_spheres(),
		cornell_box(),
	]
}
