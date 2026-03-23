use super::*;

const LEAF_CHILD_COUNT: usize = 8;
const EMPTY_LEAF_CHILD: u32 = u32::MAX;

#[derive(Copy, Clone, Debug, PartialEq)]
struct BvhBranch<T> {
	left_bounds: Bounds3<T>,
	right_bounds: Bounds3<T>,
	left_child: u32,
	right_child: u32,
}

impl<T: Float> BvhBranch<T> {
	const EMPTY: BvhBranch<T> = BvhBranch {
		left_bounds: Bounds3::EMPTY,
		right_bounds: Bounds3::EMPTY,
		left_child: 0,
		right_child: 0,
	};
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BvhLeaf {
	children: [u32; LEAF_CHILD_COUNT],
}

impl BvhLeaf {
	const EMPTY: BvhLeaf = BvhLeaf {
		children: [EMPTY_LEAF_CHILD; LEAF_CHILD_COUNT],
	};
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum BvhNode<T> {
	Branch(BvhBranch<T>),
	Leaf(BvhLeaf),
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ChildHit<T> {
	child: u32,
	distance: T,
}

#[inline]
fn encode_leaf_child(index: usize) -> u32 {
	u32::try_from(index).expect("BVH leaf index overflow")
}

#[inline]
fn decode_leaf_child(child: u32) -> Option<usize> {
	(child != EMPTY_LEAF_CHILD).then_some(child as usize)
}

#[inline]
fn encode_node_child(index: usize) -> u32 {
	u32::try_from(index).expect("BVH node index overflow")
}

/// 3D bounding volume hierarchy.
///
/// Accelerates point containment and ray intersection tests against a collection of shapes.
///
/// The BVH does not store the actual shapes, but rather indices and bounds for each shape.
/// The caller is responsible for providing the shape data and implementing the exact intersection logic in the callbacks.
///
/// ```
/// use cvmath::*;
/// struct Collection {
/// 	shapes: Vec<Shape3<f32>>,
/// 	bvh: Bvh3<f32>,
/// }
/// impl Collection {
/// 	pub fn build(shapes: Vec<Shape3<f32>>) -> Collection {
/// 		let bvh = Bvh3::build(shapes.iter().map(|shape| shape.bounds().unwrap()).enumerate());
/// 		Collection { shapes, bvh }
/// 	}
/// }
/// impl Trace3<f32> for Collection {
/// 	fn inside(&self, pt: Point3<f32>) -> bool {
/// 		self.bvh.inside(pt, |index, pt| self.shapes[index].inside(pt))
/// 	}
/// 	fn trace(&self, ray: &Ray3<f32>) -> Option<Hit3<f32>> {
/// 		self.bvh.trace(ray, |index, ray| {
/// 			self.shapes[index].trace(ray).map(|hit| Hit3 { index, ..hit })
/// 		})
/// 	}
/// }
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bvh3<T> {
	nodes: Vec<BvhNode<T>>,
}

impl<T> Bvh3<T> {
	/// Builds a BVH from leaf indices and bounds.
	///
	/// Each input item becomes one BVH leaf. The supplied index is passed back to the traversal callbacks.
	pub fn build<I>(items: I) -> Bvh3<T> where T: Float, I: IntoIterator<Item = (usize, Bounds3<T>)> {
		let items = items.into_iter()
			.map(|(index, bounds)| BuildItem { index, bounds, center: bounds.center() })
			.collect();
		build::<T>(items)
	}

	/// Reorders the input items to match the BVH leaf traversal order.
	///
	/// Items that end up in the same BVH leaf become contiguous in memory, which can improve cache locality
	/// during traversal. After this call, the indices passed to `inside` and `trace` callbacks refer to the
	/// reordered positions in the returned vector, not the original positions.
	pub fn optimize_reorder<U: Clone>(&mut self, items: Vec<U>) -> Vec<U> {
		optimize_reorder::<T, U>(self, items)
	}
}

impl<T: Float> Bvh3<T> {
	/// Visits branch child bounds in depth-first order.
	///
	/// The callback receives the child depth and its bounds. Returning `true` continues
	/// traversal into that child; returning `false` prunes that subtree.
	#[inline]
	pub fn visit<F>(&self, mut f: F) where F: FnMut(u32, &Bounds3<T>) -> bool {
		self.visit_node(0, 1, &mut f)
	}

	fn visit_node<F>(&self, node_index: u32, depth: u32, f: &mut F) where F: FnMut(u32, &Bounds3<T>) -> bool {
		let Some(node) = self.nodes.get(node_index as usize) else {
			return;
		};

		match node {
			BvhNode::Branch(node) => {
				if f(depth, &node.left_bounds) {
					self.visit_node(node.left_child, depth + 1, f);
				}
				if f(depth, &node.right_bounds) {
					self.visit_node(node.right_child, depth + 1, f);
				}
			}
			BvhNode::Leaf(_) => {}
		}
	}

	/// Returns whether the point lies inside any leaf accepted by the callback.
	#[inline]
	pub fn inside<F: FnMut(usize, Point3<T>) -> bool>(&self, pt: Point3<T>, mut f: F) -> bool {
		self.inside_node(0, pt, &mut f)
	}

	fn inside_node<F>(&self, node_index: u32, pt: Point3<T>, f: &mut F) -> bool where F: FnMut(usize, Point3<T>) -> bool {
		let Some(node) = self.nodes.get(node_index as usize) else {
			return false;
		};

		match node {
			BvhNode::Branch(node) => {
				if self.inside_child(node.left_bounds, node.left_child, pt, f) {
					return true;
				}
				self.inside_child(node.right_bounds, node.right_child, pt, f)
			}
			BvhNode::Leaf(leaf) => self.inside_leaf(leaf, pt, f),
		}
	}

	#[inline]
	fn inside_child<F>(&self, bounds: Bounds3<T>, child: u32, pt: Point3<T>, f: &mut F) -> bool where F: FnMut(usize, Point3<T>) -> bool {
		if !bounds.contains(pt) {
			return false;
		}
		self.inside_node(child, pt, f)
	}

	#[inline]
	fn inside_leaf<F>(&self, leaf: &BvhLeaf, pt: Point3<T>, f: &mut F) -> bool where F: FnMut(usize, Point3<T>) -> bool {
		leaf.children.into_iter()
			.filter_map(decode_leaf_child)
			.any(|child| f(child, pt))
	}

	/// Traces the BVH and delegates exact leaf testing to the callback.
	///
	/// The callback receives the leaf index and should return the closest hit in that leaf.
	#[inline]
	pub fn trace<F>(&self, ray: &Ray3<T>, mut f: F) -> Option<Hit3<T>> where F: FnMut(usize, &Ray3<T>) -> Option<Hit3<T>> {
		let mut ray = BvhRay::new(ray);
		self.trace_node(0, &mut ray, &mut f)
	}

	fn trace_node<F>(&self, node_index: u32, ray: &mut BvhRay<T>, f: &mut F) -> Option<Hit3<T>> where F: FnMut(usize, &Ray3<T>) -> Option<Hit3<T>> {
		let node = self.nodes.get(node_index as usize)?;

		match node {
			BvhNode::Branch(branch) => self.trace_branch(branch, ray, f),
			BvhNode::Leaf(leaf) => self.trace_leaf(leaf, ray, f),
		}
	}

	#[inline]
	fn trace_branch<F>(&self, branch: &BvhBranch<T>, ray: &mut BvhRay<T>, f: &mut F) -> Option<Hit3<T>> where F: FnMut(usize, &Ray3<T>) -> Option<Hit3<T>> {
		// Step 1: Intersect ray with both child bounds
		let left = ray_aabb(ray, &branch.left_bounds).map(|distance| ChildHit { child: branch.left_child, distance });
		let right = ray_aabb(ray, &branch.right_bounds).map(|distance| ChildHit { child: branch.right_child, distance });

		// Step 2: Sort children by near hit distance
		let (near, far) = order_child_hits(left, right)?;

		// Step 3: Trace near child first
		let result = self.trace_node(near.child, ray, f);
		let Some(far) = far else {
			return result;
		};

		// Step 4: Stop if the clipped ray cannot reach the far child
		if ray.ray.distance.max < far.distance {
			return result;
		}

		// Step 5: Any far-child hit must be at least as close as the current best
		self.trace_node(far.child, ray, f).or(result)
	}

	#[inline]
	fn trace_leaf<F>(&self, leaf: &BvhLeaf, ray: &mut BvhRay<T>, f: &mut F) -> Option<Hit3<T>> where F: FnMut(usize, &Ray3<T>) -> Option<Hit3<T>> {
		let mut best = None;

		for child in leaf.children.into_iter().filter_map(decode_leaf_child) {
			if let Some(hit) = f(child, &ray.ray) {
				ray.ray.distance.max = hit.distance;
				best = Some(hit);
			}
		}

		best
	}
}

#[inline]
fn ray_aabb<T: Float>(ray: &BvhRay<T>, bounds: &Bounds3<T>) -> Option<T> {
	#[inline]
	fn ray_axis<T: Float>(origin: T, inv_direction: T, min: T, max: T) -> Option<(T, T)> {
		if !inv_direction.is_finite() {
			if origin < min || origin > max {
				return None;
			}
			return Some((T::NEG_INFINITY, T::INFINITY));
		}

		Some(((min - origin) * inv_direction).min_max((max - origin) * inv_direction))
	}

	let (x0, x1) = ray_axis(ray.ray.origin.x, ray.inv_direction.x, bounds.mins.x, bounds.maxs.x)?;
	let (y0, y1) = ray_axis(ray.ray.origin.y, ray.inv_direction.y, bounds.mins.y, bounds.maxs.y)?;
	let (z0, z1) = ray_axis(ray.ray.origin.z, ray.inv_direction.z, bounds.mins.z, bounds.maxs.z)?;

	let near = T::max(T::max(T::max(x0, y0), z0), ray.ray.distance.min);
	let far = T::min(T::min(T::min(x1, y1), z1), ray.ray.distance.max);

	if near <= far && far > ray.ray.distance.min {
		Some(near)
	}
	else {
		None
	}
}

#[derive(Clone, Debug)]
struct BvhRay<T> {
	ray: Ray3<T>,
	inv_direction: Vec3<T>,
}

impl<T: Float> BvhRay<T> {
	#[inline]
	fn new(ray: &Ray3<T>) -> BvhRay<T> {
		BvhRay {
			ray: ray.clone(),
			inv_direction: ray.direction.map(|d| T::ONE / d),
		}
	}
}

#[inline]
fn order_child_hits<T: Float>(
	left: Option<ChildHit<T>>,
	right: Option<ChildHit<T>>,
) -> Option<(ChildHit<T>, Option<ChildHit<T>>)> {
	match (left, right) {
		(None, None) => None,
		(Some(hit), None) | (None, Some(hit)) => Some((hit, None)),
		(Some(left), Some(right)) => {
			if left.distance <= right.distance {
				Some((left, Some(right)))
			}
			else {
				Some((right, Some(left)))
			}
		}
	}
}

//----------------------------------------------------------------
// BVH building

#[derive(Copy, Clone, Debug)]
struct BuildItem<T> {
	index: usize,
	bounds: Bounds3<T>,
	center: Point3<T>,
}

fn build<T>(mut items: Vec<BuildItem<T>>) -> Bvh3<T> where T: Float {
	if items.is_empty() {
		return Bvh3 { nodes: Vec::new() };
	}

	let node_count = items.len().max(1);
	let mut nodes = Vec::with_capacity(node_count);
	build_node(&mut nodes, &mut items);
	Bvh3 { nodes }
}

fn build_node<T: Float>(nodes: &mut Vec<BvhNode<T>>, items: &mut [BuildItem<T>]) -> (usize, Bounds3<T>) {
	debug_assert!(!items.is_empty());

	if items.len() <= LEAF_CHILD_COUNT {
		let node_index = nodes.len();
		nodes.push(BvhNode::Leaf(build_leaf(items)));
		return (node_index, items.iter().map(|item| item.bounds).collect());
	}

	let node_index = nodes.len();
	nodes.push(BvhNode::Branch(BvhBranch::EMPTY));

	let split_axis = split_axis(items);
	let mid = items.len() / 2;
	items.select_nth_unstable_by(mid, |a, b| {
		a.center[split_axis].total_cmp(&b.center[split_axis])
	});
	let (left_items, right_items) = items.split_at_mut(mid);

	let (left_child, left_bounds) = build_node(nodes, left_items);
	let (right_child, right_bounds) = build_node(nodes, right_items);
	nodes[node_index] = BvhNode::Branch(BvhBranch {
		left_bounds,
		right_bounds,
		left_child: encode_node_child(left_child),
		right_child: encode_node_child(right_child),
	});

	(node_index, left_bounds.union(right_bounds))
}

fn split_axis<T: Float>(items: &[BuildItem<T>]) -> usize {
	let mut center_bounds = Bounds3::EMPTY;
	for item in items {
		center_bounds = center_bounds.include(item.center);
	}
	let size = center_bounds.size();
	if size.x >= size.y && size.x >= size.z { 0 } else if size.y >= size.z { 1 } else { 2 }
}

#[inline]
fn build_leaf<T>(items: &[BuildItem<T>]) -> BvhLeaf {
	let mut leaf = BvhLeaf::EMPTY;
	for (slot, item) in leaf.children.iter_mut().zip(items.iter()) {
		*slot = encode_leaf_child(item.index);
	}
	leaf
}

//----------------------------------------------------------------
// BVH optimization

fn optimize_reorder<T, U: Clone>(this: &mut Bvh3<T>, items: Vec<U>) -> Vec<U> {
	if this.nodes.is_empty() {
		debug_assert!(items.is_empty());
		return items;
	}

	let mut order = Vec::with_capacity(items.len());
	collect_leaf_order(&this.nodes, 0, &mut order);
	assert_eq!(order.len(), items.len(), "BVH leaf count does not match item count");

	let mut remap = vec![EMPTY_LEAF_CHILD; items.len()];
	for (new_index, old_index) in order.iter().copied().enumerate() {
		assert!(old_index < items.len(), "BVH leaf index out of range");
		assert_eq!(remap[old_index], EMPTY_LEAF_CHILD, "BVH leaf index appears multiple times");
		remap[old_index] = encode_leaf_child(new_index);
	}

	let reordered = order.into_iter().map(|old_index| items[old_index].clone()).collect();

	remap_leaf_indices(&mut this.nodes, &remap);
	reordered
}

fn collect_leaf_order<T>(nodes: &[BvhNode<T>], node_index: u32, order: &mut Vec<usize>) {
	let Some(node) = nodes.get(node_index as usize) else {
		return;
	};

	match node {
		BvhNode::Branch(node) => {
			collect_leaf_order(nodes, node.left_child, order);
			collect_leaf_order(nodes, node.right_child, order);
		}
		BvhNode::Leaf(leaf) => {
			order.extend(leaf.children.into_iter().filter_map(decode_leaf_child));
		}
	}
}

fn remap_leaf_indices<T>(nodes: &mut [BvhNode<T>], remap: &[u32]) {
	for node in nodes {
		if let BvhNode::Leaf(leaf) = node {
			for child in &mut leaf.children {
				if let Some(index) = decode_leaf_child(*child) {
					*child = remap[index];
				}
			}
		}
	}
}

#[cfg(test)]
mod tests;
