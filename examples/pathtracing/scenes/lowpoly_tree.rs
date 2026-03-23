use std::mem;

use super::*;

macro_rules! include_static {
	($vis:vis $name:ident: [$ty:ty] = $file:expr) => {
		$vis static $name: [$ty; include_bytes!($file).len() / mem::size_of::<$ty>()] = unsafe {
			mem::transmute(*include_bytes!($file))
		};
	};
}

include_static!(VERTICES: [Point3f] = "lowpoly_tree/vertices.bin");

#[repr(C)]
struct Face {
	verts: [u32; 3],
	surface: u32,
}

include_static!(FACES: [Face] = "lowpoly_tree/triangles.bin");

#[repr(usize)]
enum MaterialIndex {
	Red = 0,
	Blue = 1,
	Gray = 2,
	DarkGray = 3,
	White = 4,
	Green = 5,
	Light = 6,
	TreeBark = 7,
	TreeLeaves = 8,
	Mirror = 9,
	Gold = 10,
}

const DIFFUSE: Material = Material {
	color: Vec3::ONE,
	emissive: Vec3::ZERO,
	roughness: 1.0,
	metallic: 0.0,
};

static MATERIALS: [Material; 11] = [
	// Red
	Material {
		color: Vec3(0.8, 0.0, 0.0),
		..DIFFUSE
	},
	// Blue
	Material {
		color: Vec3(0.15, 0.40, 0.7),
		..DIFFUSE
	},
	// Gray
	Material {
		color: Vec3(0.5, 0.5, 0.5),
		..DIFFUSE
	},
	// DarkGray
	Material {
		color: Vec3(0.2, 0.2, 0.2),
		..DIFFUSE
	},
	// White
	Material {
		color: Vec3(0.9, 0.9, 0.9),
		..DIFFUSE
	},
	// Green
	Material {
		color: Vec3(0.13, 0.5, 0.11),
		..DIFFUSE
	},
	// Light
	Material {
		color: Vec3f::ONE,
		emissive: Vec3f(10.0, 10.0, 10.0),
		roughness: 0.0,
		metallic: 0.0,
	},
	// TreeBark
	Material {
		color: Vec3(0.207595, 0.138513, 0.055181),
		emissive: Vec3::ZERO,
		roughness: 1.0,
		metallic: 0.0,
	},
	// TreeLeaves
	Material {
		color: Vec3(0.256861, 0.440506, 0.110769),
		emissive: Vec3::ZERO,
		roughness: 1.0,
		metallic: 0.0,
	},
	// Mirror
	Material {
		color: Vec3f::ONE,
		emissive: Vec3f::ZERO,
		roughness: 0.0,
		metallic: 1.0,
	},
	// Gold
	Material {
		color: Vec3(0.8, 0.6, 0.2),
		emissive: Vec3::ZERO,
		roughness: 0.1,
		metallic: 0.5,
	},
];

fn add_tree(objects: &mut Vec<Object>) {
	// Compute bounding box of the tree vertices
	let source_bounds: Bounds3f = VERTICES.iter().cloned().collect();
	// Use the base center as the pivot for scaling and placement
	let source_origin = source_bounds.center().set_y(source_bounds.mins.y);

	let target_origin = Vec3(5.0, 0.0, 4.0); // Position in the Cornell box
	let target_scale = 8.5 / source_bounds.size().vmax(); // Uniform scale to fit
	let transform = Transform3f::translation(target_origin) // Move tree to final location
		// * Transform3f::rotation(Vec3::Y, Angle::deg(-45.0)) // Rotate tree for better view
		* Transform3f::scaling(Vec3::dup(target_scale)) // Apply uniform scale
		* Transform3f::translation(-source_origin); // Move pivot to origin for scaling

	// Converts a localspace Face to a worlspace Object
	let face_fn = |face: &Face| Object {
		shape: Shape3::Triangle(Triangle3::points(
			transform * VERTICES[face.verts[0] as usize],
			transform * VERTICES[face.verts[1] as usize],
			transform * VERTICES[face.verts[2] as usize],
		)),
		material: match face.surface {
			0 => MaterialIndex::TreeBark,
			_ => MaterialIndex::TreeLeaves,
		} as u32,
	};

	// Adds all faces as objects to the scene
	objects.extend(FACES.iter().map(face_fn));
}

pub fn scene() -> (&'static str, Scene) {
	let image = ImageSettings {
		width: 1080,
		height: 1080,
		nsamples: 128,//*32*4,
		max_bounces: 4,
		use_rayon: true,
	};
	let camera = CameraSettings {
		origin: Vec3(5.0, 5.0, -5.0),
		target: Vec3(5.0, 5.0, 5.0),
		ref_up: Vec3(0.0, 1.0, 0.0),
		fov_y: Angle::deg(90.0),
		dof_enabled: true,
		aperture_radius: 0.08,
		focus_distance: 10.0,
	};
	let mut objects = Vec::new();
	let mut add_object = |shape: Shape3<f32>, material: MaterialIndex| {
		let object = Object { shape, material: material as u32 };
		objects.push(object);
	};

	add_object(Shape3::Plane(Plane3( Vec3f::X,  0.0)), MaterialIndex::Red);
	add_object(Shape3::Plane(Plane3(-Vec3f::X, 10.0)), MaterialIndex::Blue);
	add_object(Shape3::Plane(Plane3(-Vec3f::Z, 10.0)), MaterialIndex::Gray);
	add_object(Shape3::Plane(Plane3( Vec3f::Z,  0.0)), MaterialIndex::DarkGray);
	add_object(Shape3::Plane(Plane3(-Vec3f::Y, 10.0)), MaterialIndex::White);
	add_object(Shape3::Plane(Plane3( Vec3f::Y,  0.0)), MaterialIndex::Green);

	add_object(
		Shape3::Bounds(Bounds3::point(Vec3(5.0, 10.0, 5.0), Vec3(2.0, 0.1, 2.0))),
		MaterialIndex::Light,
	);

	// Two metallic spheres
	add_object(
		Shape3::Sphere(Sphere { center: Vec3(1.0, 1.0, 9.0), radius: 2.0 }),
		MaterialIndex::Gold,
	);
	add_object(
		Shape3::Sphere(Sphere { center: Vec3(9.0, 1.0, 9.0), radius: 4.0 }),
		MaterialIndex::Mirror,
	);

	add_tree(&mut objects);

	let materials = MATERIALS.to_vec();
	let scene = Scene { image, camera, world: WorldBuilder { env_light: None, materials, objects }.build() };
	("lowpoly_tree.ppm", scene)
}
