use super::*;

#[derive(Clone, Debug, PartialEq)]
struct IndexedSphere {
	id: usize,
	sphere: Sphere<f32>,
}

fn trace_indexed_spheres(spheres: &[IndexedSphere], ray: &Ray3<f32>) -> Option<Hit3<f32>> {
	let mut best = None;
	for sphere in spheres {
		if let Some(hit) = sphere.sphere.trace(ray) {
			if best.as_ref().is_none_or(|best: &Hit3<f32>| hit.distance < best.distance) {
				best = Some(Hit3 { index: sphere.id, ..hit });
			}
		}
	}
	best
}

#[test]
fn test_bvh_optimize_reorder_reorders_sphere_items() {
	let mut spheres = vec![
		IndexedSphere { id: 0, sphere: Sphere { center: Point3(-9.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 1, sphere: Sphere { center: Point3(9.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 2, sphere: Sphere { center: Point3(-7.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 3, sphere: Sphere { center: Point3(7.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 4, sphere: Sphere { center: Point3(-5.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 5, sphere: Sphere { center: Point3(5.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 6, sphere: Sphere { center: Point3(-3.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 7, sphere: Sphere { center: Point3(3.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 8, sphere: Sphere { center: Point3(-1.0, 0.0, 0.0), radius: 0.75 } },
		IndexedSphere { id: 9, sphere: Sphere { center: Point3(1.0, 0.0, 0.0), radius: 0.75 } },
	];
	let original = spheres.clone();

	let mut bvh = Bvh3::build(spheres.iter().enumerate().map(|(index, item)| (index, sphere_bounds(item.sphere))));
	spheres = bvh.optimize_reorder(spheres);

	assert_ne!(spheres, original);

	for &(y, z) in &[(-0.4, -0.4), (0.0, 0.0), (0.4, 0.4)] {
		let ray = Ray3(Point3(-12.0, y, z), Vec3(1.0, 0.0, 0.0), Interval(0.0, 24.0));
		let linear = trace_indexed_spheres(&spheres, &ray);
		let traced = bvh.trace(&ray, |index, clipped| {
			spheres[index].sphere.trace(clipped).map(|hit| Hit3 { index: spheres[index].id, ..hit })
		});

		match (linear, traced) {
			(None, None) => (),
			(Some(linear), Some(traced)) => {
				assert_eq!(linear.index, traced.index);
				assert!(linear.distance.is_close(traced.distance));
			}
			(linear, traced) => panic!("linear={linear:?} traced={traced:?}"),
		}
	}
}

#[derive(Copy, Clone, Debug)]
struct SphereFieldConfig {
	region: Bounds3<f32>,
	sphere_count: usize,
	radius: f32,
	ray_count: usize,
	seed: u64,
}

#[derive(Clone, Debug)]
struct SphereField {
	spheres: Vec<Sphere<f32>>,
	bvh: Bvh3<f32>,
	rays: Vec<Ray3<f32>>,
}

impl SphereField {
	fn new(config: SphereFieldConfig) -> SphereField {
		let mut rng = urandom::seeded(config.seed);
		let radius = config.radius;
		let region = config.region;

		let mut spheres = Vec::with_capacity(config.sphere_count);
		for _ in 0..config.sphere_count {
			let center = Point3(
				rng.range(region.mins.x + radius..region.maxs.x - radius),
				rng.range(region.mins.y + radius..region.maxs.y - radius),
				rng.range(region.mins.z + radius..region.maxs.z - radius),
			);
			spheres.push(Sphere { center, radius });
		}

		let bvh = Bvh3::build(spheres.iter().map(Sphere::bounds).enumerate());

		let mut rays = Vec::with_capacity(config.ray_count);
		for _ in 0..config.ray_count {
			let origin = Point3(
				region.mins.x - radius * 4.0,
				rng.range(region.mins.y..region.maxs.y),
				rng.range(region.mins.z..region.maxs.z),
			);
			let target = Point3(
				region.maxs.x + radius * 4.0,
				rng.range(region.mins.y..region.maxs.y),
				rng.range(region.mins.z..region.maxs.z),
			);
			let direction = target - origin;
			rays.push(Ray3::new(origin, direction, Interval(0.0, origin.distance(target))));
		}

		SphereField { spheres, bvh, rays }
	}

	fn trace_linear(&self, ray: &Ray3<f32>) -> Option<Hit3<f32>> {
		ray.trace_collection(&self.spheres)
	}

	fn trace_bvh(&self, ray: &Ray3<f32>) -> Option<Hit3<f32>> {
		self.bvh.trace(ray, |index, clipped| {
			self.spheres[index].trace(clipped).map(|hit| Hit3 { index, ..hit })
		})
	}
}

#[inline]
fn sphere_bounds(sphere: Sphere<f32>) -> Bounds3<f32> {
	let radius = Vec3(sphere.radius, sphere.radius, sphere.radius);
	Bounds3(sphere.center - radius, sphere.center + radius)
}

#[test]
fn test_bvh_sphere_field_matches_linear() {
	let scene = SphereField::new(SphereFieldConfig {
		region: Bounds3(Point3(-10.0, -10.0, -10.0), Point3(10.0, 10.0, 10.0)),
		sphere_count: 128,
		radius: 0.8,
		ray_count: 4096,
		seed: 41,
	});

	let mut mismatches = Vec::new();
	for (ray_index, ray) in scene.rays.iter().enumerate() {
		let linear = scene.trace_linear(ray);
		let bvh = scene.trace_bvh(ray);

		match (linear, bvh) {
			(None, None) => (),
			(Some(linear), Some(bvh)) => {
				if !(linear.index == bvh.index && linear.distance.is_close(bvh.distance)) {
					mismatches.push(format!("ray {ray_index}: linear={linear:?} bvh={bvh:?}"));
				}
			}
			(linear, bvh) => mismatches.push(format!("ray {ray_index}: linear={linear:?} bvh={bvh:?}")),
		}
	}

	assert!(mismatches.is_empty(), "{} BVH mismatches\n{}", mismatches.len(), mismatches.join("\n"));
}
