
pub trait SpatialOrd<Rhs = Self> {
	#[must_use]
	fn spatial_lt(&self, rhs: &Rhs) -> bool;
	#[must_use]
	fn spatial_le(&self, rhs: &Rhs) -> bool;
	#[must_use]
	fn spatial_gt(&self, rhs: &Rhs) -> bool;
	#[must_use]
	fn spatial_ge(&self, rhs: &Rhs) -> bool;
}
