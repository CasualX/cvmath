use super::*;

fn leaf(index: u32) -> BvhLeaf {
	let mut leaf = BvhLeaf::EMPTY;
	leaf.children[0] = index;
	leaf
}

#[test]
fn trace_returns_far_hit_when_near_child_misses() {

	let bvh = Bvh3 {
		nodes: vec![
			BvhNode::Branch(BvhBranch {
				left_bounds: Bounds3(Point3(-5.0, -1.0, -1.0), Point3(-1.0, 1.0, 1.0)),
				right_bounds: Bounds3(Point3(-4.0, -1.0, -1.0), Point3(2.0, 1.0, 1.0)),
				left_child: 1,
				right_child: 2,
			}),
			BvhNode::Leaf(leaf(0)),
			BvhNode::Leaf(leaf(1)),
		],
	};

	let ray = Ray3(Point3(-10.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Interval(0.0, 20.0));
	let hit = bvh.trace(&ray, |index, _| match index {
		0 => None,
		1 => Some(Hit3 {
			point: Point3(-2.0, 0.0, 0.0),
			distance: 8.0,
			normal: Vec3(-1.0, 0.0, 0.0),
			index,
			side: HitSide::Entry,
		}),
		_ => None,
	}).unwrap();

	assert_eq!(hit.index, 1);
	assert_eq!(hit.distance, 8.0);
}

#[test]
fn trace_prefers_closest_hit_when_ray_starts_inside_overlapping_bounds() {
	let bvh = Bvh3 {
		nodes: vec![
			BvhNode::Branch(BvhBranch {
				left_bounds: Bounds3(Point3(-10.0, -10.0, -10.0), Point3(10.0, 10.0, 10.0)),
				right_bounds: Bounds3(Point3(-5.0, -5.0, -5.0), Point3(20.0, 20.0, 20.0)),
				left_child: 1,
				right_child: 2,
			}),
			BvhNode::Leaf(leaf(0)),
			BvhNode::Leaf(leaf(1)),
		],
	};

	let ray = Ray3(Point3(0.0, 0.0, 0.0), Vec3(1.0, 0.0, 0.0), Interval(0.0, 100.0));
	let hit = bvh.trace(&ray, |index, _| match index {
		0 => Some(Hit3 {
			point: Point3(5.0, 0.0, 0.0),
			distance: 5.0,
			normal: Vec3(-1.0, 0.0, 0.0),
			index,
			side: HitSide::Entry,
		}),
		1 => Some(Hit3 {
			point: Point3(1.0, 0.0, 0.0),
			distance: 1.0,
			normal: Vec3(-1.0, 0.0, 0.0),
			index,
			side: HitSide::Entry,
		}),
		_ => None,
	}).unwrap();

	assert_eq!(hit.index, 1);
	assert_eq!(hit.distance, 1.0);
}

#[test]
fn inspect_visits_bounds_depth_first() {
	let bvh = Bvh3 {
		nodes: vec![
			BvhNode::Branch(BvhBranch {
				left_bounds: Bounds3(Point3(-4.0, -4.0, -4.0), Point3(0.0, 4.0, 4.0)),
				right_bounds: Bounds3(Point3(0.0, -2.0, -2.0), Point3(4.0, 2.0, 2.0)),
				left_child: 1,
				right_child: 2,
			}),
			BvhNode::Branch(BvhBranch {
				left_bounds: Bounds3(Point3(-4.0, 0.0, 0.0), Point3(-2.0, 4.0, 4.0)),
				right_bounds: Bounds3(Point3(-2.0, -4.0, -4.0), Point3(0.0, 0.0, 0.0)),
				left_child: 3,
				right_child: 4,
			}),
			BvhNode::Leaf(leaf(0)),
			BvhNode::Leaf(leaf(1)),
			BvhNode::Leaf(leaf(2)),
		],
	};

	let mut visited = Vec::new();
	bvh.visit(|depth, bounds| {
		visited.push((depth, *bounds));
		true
	});

	assert_eq!(visited, vec![
		(1, Bounds3(Point3(-4.0, -4.0, -4.0), Point3(0.0, 4.0, 4.0))),
		(2, Bounds3(Point3(-4.0, 0.0, 0.0), Point3(-2.0, 4.0, 4.0))),
		(2, Bounds3(Point3(-2.0, -4.0, -4.0), Point3(0.0, 0.0, 0.0))),
		(1, Bounds3(Point3(0.0, -2.0, -2.0), Point3(4.0, 2.0, 2.0))),
	]);
}
