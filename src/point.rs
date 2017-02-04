
use ::std::{mem, ops};

use ::num::{Cast};

use ::{Vec2, Vec3};

/// A point in 2-dimensional space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Point2<T> {
	pub x: T,
	pub y: T,
}

/// A point in 3-dimensional space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Point3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

macro_rules! point {
	($ty:ident $vec:ident { $($field:ident $I:tt $T:ident),+ } $N:expr) => {

		//----------------------------------------------------------------
		// Constructors

		impl<T> $ty<T> {
			pub fn new($($field: T),+) -> $ty<T> {
				$ty { $($field: $field),+ }
			}
			pub fn dup(u: T) -> $ty<T> where T: Copy {
				$ty { $($field: u),+ }
			}
		}
		
		//----------------------------------------------------------------
		// Conversions

		impl<T, U> Cast<$ty<U>> for $ty<T> where T: Cast<U> {
			fn cast(self) -> $ty<U> {
				$ty { $($field: self.$field.cast()),+ }
			}
		}

		impl<T> From<($($T,)+)> for $ty<T> {
			fn from(val: ($($T,)+)) -> $ty<T> {
				$ty { $($field: val.$I),+ }
			}
		}
		impl<T> Into<($($T,)+)> for $ty<T> {
			fn into(self) -> ($($T,)+) {
				($(self.$field,)+)
			}
		}

		impl<T: Copy> From<[T; $N]> for $ty<T> {
			fn from(val: [T; $N]) -> $ty<T> {
				// Can't move out of array...
				$ty { $($field: val[$I]),+ }
			}
		}
		impl<T> Into<[T; $N]> for $ty<T> {
			fn into(self) -> [T; $N] {
				[$(self.$field),+]
			}
		}

		impl<T> From<$vec<T>> for $ty<T> {
			fn from(vec: $vec<T>) -> $ty<T> {
				$ty { $($field: vec.$field),+ }
			}
		}
		impl<T> From<$ty<T>> for $vec<T> {
			fn from(pt: $ty<T>) -> $vec<T> {
				$vec { $($field: pt.$field),+ }
			}
		}

		impl<'a, T> From<&'a ($($T,)+)> for &'a $ty<T> {
			fn from(val: &'a ($($T,)+)) -> &'a $ty<T> {
				unsafe { mem::transmute(val) }
			}
		}
		impl<'a, T> From<&'a mut ($($T,)+)> for &'a mut $ty<T> {
			fn from(val: &'a mut ($($T,)+)) -> &'a mut $ty<T> {
				unsafe { mem::transmute(val) }
			}
		}

		impl<'a, T> From<&'a [T; $N]> for &'a $ty<T> {
			fn from(val: &'a [T; $N]) -> &'a $ty<T> {
				unsafe { mem::transmute(val) }
			}
		}
		impl<'a, T> From<&'a mut [T; $N]> for &'a mut $ty<T> {
			fn from(val: &'a mut [T; $N]) -> &'a mut $ty<T> {
				unsafe { mem::transmute(val) }
			}
		}

		impl<'a, T> From<&'a [T]> for &'a $ty<T> {
			fn from(val: &'a [T]) -> &'a $ty<T> {
				assert_eq!($N, val.len());
				unsafe { mem::transmute(val.as_ptr()) }
			}
		}
		impl<'a, T> From<&'a mut [T]> for &'a mut $ty<T> {
			fn from(val: &'a mut [T]) -> &'a mut $ty<T> {
				assert_eq!($N, val.len());
				unsafe { mem::transmute(val.as_mut_ptr()) }
			}
		}

		//----------------------------------------------------------------
		// As references

		impl<T> AsRef<($($T,)+)> for $ty<T> {
			fn as_ref(&self) -> &($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T; $N]> for $ty<T> {
			fn as_ref(&self) -> &[T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T]> for $ty<T> {
			fn as_ref(&self) -> &[T] {
				<Self as AsRef<[T; $N]>>::as_ref(self)
			}
		}
		impl<T> AsRef<$vec<T>> for $ty<T> {
			fn as_ref(&self) -> &$vec<T> {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<$ty<T>> for $vec<T> {
			fn as_ref(&self) -> &$ty<T> {
				unsafe { mem::transmute(self) }
			}
		}
		
		impl<T> AsMut<($($T,)+)> for $ty<T> {
			fn as_mut(&mut self) -> &mut ($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T; $N]> for $ty<T> {
			fn as_mut(&mut self) -> &mut [T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T]> for $ty<T> {
			fn as_mut(&mut self) -> &mut [T] {
				<Self as AsMut<[T; $N]>>::as_mut(self)
			}
		}
		impl<T> AsMut<$vec<T>> for $ty<T> {
			fn as_mut(&mut self) -> &mut $vec<T> {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<$ty<T>> for $vec<T> {
			fn as_mut(&mut self) -> &mut $ty<T> {
				unsafe { mem::transmute(self) }
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T: ops::Add<Output = T>> ops::Add<$vec<T>> for $ty<T> {
			type Output = $ty<T>;
			fn add(self, rhs: $vec<T>) -> $ty<T> {
				$ty { $($field: self.$field + rhs.$field),+ }
			}
		}
		impl<T: ops::Sub<Output = T>> ops::Sub<$ty<T>> for $ty<T> {
			type Output = $vec<T>;
			fn sub(self, rhs: $ty<T>) -> $vec<T> {
				$vec { $($field: rhs.$field - self.$field),+ }
			}
		}

	}
}

point!(Point2 Vec2 { x 0 T, y 1 T } 2);
point!(Point3 Vec3 { x 0 T, y 1 T, z 2 T } 3);
