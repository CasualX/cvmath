
pub trait SpatialOrd<Rhs = Self> {
	fn spatial_lt(&self, rhs: &Rhs) -> bool;
	fn spatial_le(&self, rhs: &Rhs) -> bool;
	fn spatial_gt(&self, rhs: &Rhs) -> bool;
	fn spatial_ge(&self, rhs: &Rhs) -> bool;
}
