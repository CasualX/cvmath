
/// A 2-dimensional bools result.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Bools2 {
	pub x: bool,
	pub y: bool,
}

/// A 3-dimensional bools result.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Bools3 {
	pub x: bool,
	pub y: bool,
	pub z: bool,
}

/// A 4-dimensional bools result.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Bools4 {
	pub x: bool,
	pub y: bool,
	pub z: bool,
	pub w: bool,
}

macro_rules! bools {
	($ty:ident { $($field:ident $I:tt $T:ident),+ } $N:expr) => {
		impl $ty {
			pub fn any(self) -> bool {
				infix!(|| $(self.$field),+)
			}
			pub fn all(self) -> bool {
				infix!(&& $(self.$field),+)
			}
		}
	}
}

bools!(Bools2 { x 0 T, y 1 T } 2);
bools!(Bools3 { x 0 T, y 1 T, z 2 T } 3);
bools!(Bools4 { x 0 T, y 1 T, z 2 T, w 3 T } 4);
