
/// Ray hit side.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HitSide {
	/// The ray is entering the surface.
	#[default]
	Entry,
	/// The ray is exiting the surface.
	Exit,
}
