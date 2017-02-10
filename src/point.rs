/*!
Affine point.
*/

use ::std::{mem, ops};

use ::num::{Cast};

use ::{Vec2, Vec3};

/// A point in 2-dimensional space.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Point2<T> {
	pub x: T,
	pub y: T,
}

/// A point in 3-dimensional space.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Point3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

macro_rules! point {
	($pt:ident $vec:ident $N:tt { $($field:ident $I:tt $T:ident),+ }) => {

		//----------------------------------------------------------------
		// Constructors

		impl<T> $pt<T> {
			pub fn new($($field: T),+) -> $pt<T> {
				$pt { $($field: $field),+ }
			}
			pub fn dup(u: T) -> $pt<T> where T: Copy {
				$pt { $($field: u),+ }
			}
		}
		
		//----------------------------------------------------------------
		// Conversions

		impl<T, U> Cast<$pt<U>> for $pt<T> where T: Cast<U> {
			fn cast(self) -> $pt<U> {
				$pt { $($field: self.$field.cast()),+ }
			}
		}

		impl<T> From<($($T,)+)> for $pt<T> {
			fn from(val: ($($T,)+)) -> $pt<T> {
				$pt { $($field: val.$I),+ }
			}
		}
		impl<T> Into<($($T,)+)> for $pt<T> {
			fn into(self) -> ($($T,)+) {
				($(self.$field,)+)
			}
		}

		impl<T: Copy> From<[T; $N]> for $pt<T> {
			fn from(val: [T; $N]) -> $pt<T> {
				// Can't move out of array...
				$pt { $($field: val[$I]),+ }
			}
		}
		impl<T> Into<[T; $N]> for $pt<T> {
			fn into(self) -> [T; $N] {
				[$(self.$field),+]
			}
		}

		impl<T> From<$vec<T>> for $pt<T> {
			fn from(vec: $vec<T>) -> $pt<T> {
				$pt { $($field: vec.$field),+ }
			}
		}
		impl<T> From<$pt<T>> for $vec<T> {
			fn from(pt: $pt<T>) -> $vec<T> {
				$vec { $($field: pt.$field),+ }
			}
		}

		//----------------------------------------------------------------
		// As references

		impl<T> AsRef<($($T,)+)> for $pt<T> {
			fn as_ref(&self) -> &($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T; $N]> for $pt<T> {
			fn as_ref(&self) -> &[T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T]> for $pt<T> {
			fn as_ref(&self) -> &[T] {
				<Self as AsRef<[T; $N]>>::as_ref(self)
			}
		}
		impl<T> AsRef<$vec<T>> for $pt<T> {
			fn as_ref(&self) -> &$vec<T> {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<$pt<T>> for $vec<T> {
			fn as_ref(&self) -> &$pt<T> {
				unsafe { mem::transmute(self) }
			}
		}
		
		impl<T> AsMut<($($T,)+)> for $pt<T> {
			fn as_mut(&mut self) -> &mut ($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T; $N]> for $pt<T> {
			fn as_mut(&mut self) -> &mut [T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T]> for $pt<T> {
			fn as_mut(&mut self) -> &mut [T] {
				<Self as AsMut<[T; $N]>>::as_mut(self)
			}
		}
		impl<T> AsMut<$vec<T>> for $pt<T> {
			fn as_mut(&mut self) -> &mut $vec<T> {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<$pt<T>> for $vec<T> {
			fn as_mut(&mut self) -> &mut $pt<T> {
				unsafe { mem::transmute(self) }
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T: ops::Add<Output = T>> ops::Add<$vec<T>> for $pt<T> {
			type Output = $pt<T>;
			fn add(self, rhs: $vec<T>) -> $pt<T> {
				$pt { $($field: self.$field + rhs.$field),+ }
			}
		}
		impl<T: ops::Sub<Output = T>> ops::Sub<$pt<T>> for $pt<T> {
			type Output = $vec<T>;
			fn sub(self, rhs: $pt<T>) -> $vec<T> {
				$vec { $($field: rhs.$field - self.$field),+ }
			}
		}

		//----------------------------------------------------------------
		// Operators

		fmt!($pt { $($field),+ });
	}
}

point!(Point2 Vec2 2 { x 0 T, y 1 T });
point!(Point3 Vec3 3 { x 0 T, y 1 T, z 2 T });
