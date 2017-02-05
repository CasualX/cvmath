/*!
Vector masks.
*/

/// A 2-dimensional mask.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mask2 {
	pub x: bool,
	pub y: bool,
}

/// A 3-dimensional mask.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mask3 {
	pub x: bool,
	pub y: bool,
	pub z: bool,
}

/// A 4-dimensional mask.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Mask4 {
	pub x: bool,
	pub y: bool,
	pub z: bool,
	pub w: bool,
}

macro_rules! mask {
	($mask:ident { $($field:ident $I:tt $T:ident),+ } $N:expr) => {
		impl $mask {
			pub fn any(self) -> bool {
				infix!(|| $(self.$field),+)
			}
			pub fn all(self) -> bool {
				infix!(&& $(self.$field),+)
			}
		}
	}
}

mask!(Mask2 { x 0 T, y 1 T } 2);
mask!(Mask3 { x 0 T, y 1 T, z 2 T } 3);
mask!(Mask4 { x 0 T, y 1 T, z 2 T, w 3 T } 4);
