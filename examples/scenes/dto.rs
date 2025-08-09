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
	pub roughness: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Object {
	pub shape: Shape3<f32>,
	pub material: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Light {
	pub pos: Point3<f32>,
	pub color: Vec3<f32>,
	pub radius: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Scene {
	pub settings: Settings,
	pub materials: HashMap<String, Material>,
	pub objects: Vec<Object>,
	pub light: Light,
}

pub fn from_str(s: &str) -> (crate::Settings, crate::Scene) {
	let scene: Scene = ron::from_str(s).expect("Failed to parse scene");

	let mut mat2index = HashMap::new();
	let mut materials = Vec::new();
	for (name, material) in scene.materials {
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
			roughness: material.roughness,
		});
	}

	let mut objects = Vec::new();
	for object in scene.objects {
		let &material_index = mat2index.get(&object.material).expect("Material not found");
		objects.push(crate::Object {
			shape: object.shape,
			material: material_index as u32,
		});
	}

	let light = crate::Light {
		pos: scene.light.pos,
		color: scene.light.color,
		radius: scene.light.radius,
	};

	let settings = crate::Settings {
		width: scene.settings.width,
		height: scene.settings.height,
		rayon: scene.settings.rayon,
		fov_y: scene.settings.fov_y,
		origin: scene.settings.origin,
		target: scene.settings.target,
		ref_up: scene.settings.ref_up,
		dof: scene.settings.dof,
		aa: scene.settings.aa,
		aa_samples: scene.settings.aa_samples,
		max_bounces: scene.settings.max_bounces,
		ambient_light: scene.settings.ambient_light,
	};

	(settings, crate::Scene { materials, objects, light })
}

pub fn to_string(settings: &crate::Settings, scene: &crate::Scene) -> String {
	let mut materials = HashMap::new();
	for (i, material) in scene.materials.iter().enumerate() {
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
			roughness: material.roughness,
		});
	}

	let objects: Vec<Object> = scene.objects.iter().map(|obj| Object {
		shape: obj.shape.clone(),
		material: format!("material_{}", obj.material),
	}).collect();

	let light = Light {
		pos: scene.light.pos,
		color: scene.light.color,
		radius: scene.light.radius,
	};

	let scene_data = Scene {
		settings: Settings {
			width: settings.width,
			height: settings.height,
			rayon: settings.rayon,
			fov_y: settings.fov_y,
			origin: settings.origin,
			target: settings.target,
			ref_up: settings.ref_up,
			dof: settings.dof,
			aa: settings.aa,
			aa_samples: settings.aa_samples,
			max_bounces: settings.max_bounces,
			ambient_light: settings.ambient_light,
		},
		materials,
		objects,
		light,
	};

	let s = ron::ser::to_string_pretty(&scene_data, ron::ser::PrettyConfig::new().struct_names(true)).expect("Failed to serialize scene");
	// let s = serde_json::to_string_pretty(&scene_data).expect("Failed to serialize scene");
	return s;
}
