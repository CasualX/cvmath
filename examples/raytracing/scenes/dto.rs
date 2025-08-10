use std::collections::HashMap;
use cvmath::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Texture {
	None,
	Color(Vec3<f32>),
	Ground,
	Sky,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Material {
	pub texture: Texture,
	pub reflectivity: f32,
	pub hardness: f32,
	pub diffuse_factor: f32,
	pub specular_factor: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Object {
	pub shape: Shape3<f32>,
	pub material: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ImageSettings {
	pub width: i32,
	pub height: i32,
	pub nsamples: i32,
	pub max_bounces: i32,
	pub use_rayon: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CameraSettings {
	pub origin: Vec3f,
	pub target: Vec3f,
	pub ref_up: Vec3f,
	pub fov_y: Angle<f32>,
	pub dof_enabled: bool,
	pub aperture_radius: f32,
	pub focus_distance: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Light {
	pub pos: Point3<f32>,
	pub color: Vec3<f32>,
	pub radius: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct World {
	pub ambient_light: f32,
	pub light: Light,
	pub materials: HashMap<String, Material>,
	pub objects: Vec<Object>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Scene {
	pub image: ImageSettings,
	pub camera: CameraSettings,
	pub world: World,
}

pub fn from_str(s: &str) -> crate::Scene {
	let scene: Scene = ron::from_str(s).expect("Failed to parse scene");

	let mut mat2index = HashMap::new();
	let mut materials = Vec::new();
	for (name, material) in scene.world.materials {
		let index = materials.len();
		mat2index.insert(name, index);
		materials.push(crate::Material {
			texture: match material.texture {
				Texture::None => crate::Texture::None,
				Texture::Color(color) => crate::Texture::Color(color),
				Texture::Ground => crate::Texture::Ground,
				Texture::Sky => crate::Texture::Sky,
			},
			reflectivity: material.reflectivity,
			hardness: material.hardness,
			diffuse_factor: material.diffuse_factor,
			specular_factor: material.specular_factor,
		});
	}

	let mut objects = Vec::new();
	for object in scene.world.objects {
		let &material_index = mat2index.get(&object.material).expect("Material not found");
		objects.push(crate::Object {
			shape: object.shape,
			material: material_index as u32,
		});
	}

	let light = crate::Light {
		pos: scene.world.light.pos,
		color: scene.world.light.color,
		radius: scene.world.light.radius,
	};

	let world = crate::World {
		ambient_light: scene.world.ambient_light,
		light,
		materials,
		objects,
	};

	let image = crate::ImageSettings {
		width: scene.image.width,
		height: scene.image.height,
		nsamples: scene.image.nsamples,
		max_bounces: scene.image.max_bounces,
		use_rayon: scene.image.use_rayon,
	};

	let camera = crate::CameraSettings {
		origin: scene.camera.origin,
		target: scene.camera.target,
		ref_up: scene.camera.ref_up,
		fov_y: scene.camera.fov_y,
		dof_enabled: scene.camera.dof_enabled,
		aperture_radius: scene.camera.aperture_radius,
		focus_distance: scene.camera.focus_distance,
	};

	crate::Scene { image, camera, world }
}

pub fn to_string(scene: &crate::Scene) -> String {
	let mut materials = HashMap::new();
	for (i, material) in scene.world.materials.iter().enumerate() {
		let name = format!("material_{}", i);
		materials.insert(name, Material {
			texture: match material.texture {
				crate::Texture::None => Texture::None,
				crate::Texture::Color(color) => Texture::Color(color),
				crate::Texture::Ground => Texture::Ground,
				crate::Texture::Sky => Texture::Sky,
			},
			reflectivity: material.reflectivity,
			hardness: material.hardness,
			diffuse_factor: material.diffuse_factor,
			specular_factor: material.specular_factor,
		});
	}

	let objects: Vec<Object> = scene.world.objects.iter().map(|obj| Object {
		shape: obj.shape.clone(),
		material: format!("material_{}", obj.material),
	}).collect();

	let light = Light {
		pos: scene.world.light.pos,
		color: scene.world.light.color,
		radius: scene.world.light.radius,
	};

	let world = World {
		ambient_light: scene.world.ambient_light,
		light,
		materials,
		objects,
	};

	let image = ImageSettings {
		width: scene.image.width,
		height: scene.image.height,
		nsamples: scene.image.nsamples,
		max_bounces: scene.image.max_bounces,
		use_rayon: scene.image.use_rayon,
	};

	let camera = CameraSettings {
		origin: scene.camera.origin,
		target: scene.camera.target,
		ref_up: scene.camera.ref_up,
		fov_y: scene.camera.fov_y,
		dof_enabled: scene.camera.dof_enabled,
		aperture_radius: scene.camera.aperture_radius,
		focus_distance: scene.camera.focus_distance,
	};

	let scene = Scene { image, camera, world };

	let s = ron::ser::to_string_pretty(&scene, ron::ser::PrettyConfig::new().struct_names(true).indentor("\t")).expect("Failed to serialize scene");
	// let s = serde_json::to_string_pretty(&scene).expect("Failed to serialize scene");
	return s;
}
