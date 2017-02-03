/*! Define the vector types.

This may or may not be horrible abuse of the `macro_rules!` system.
*/

use ::std::{mem, ops};

use ::{Unit, Float, Cast};

macro_rules! infix {
	($op:tt $e:expr) => ($e);
	($op:tt $e:expr, $($tail:expr),*) => ($e $op infix!($op $($tail),*));
}

macro_rules! hsub {
	(Vec1) => ();
	(Vec2) => (pub fn hsub(self) -> T { self.x - self.y });
	(Vec3) => ();
	(Vec4) => ();
}

macro_rules! vec {
	($ty:ident { $($field:ident $I:tt $T:ident),+ } $N:expr) => {
		#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
		#[repr(C)]
		pub struct $ty<T> {
			$(pub $field: T),+
		}
		
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

		//----------------------------------------------------------------
		// Operations

		impl<T: Unit> $ty<T> {
			/// Horizontal add all components together.
			pub fn hadd(self) -> T {
				infix!(+ $(self.$field),+)
			}
			hsub!($ty);
			pub fn len_sqr(self) -> T {
				infix!(+ $(self.$field * self.$field),+)
			}
			pub fn len(self) -> T where T: Float {
				self.len_sqr().sqrt()
			}
			pub fn dist_sqr(self, rhs: $ty<T>) -> T {
				infix!(+ $((rhs.$field - self.$field) * (rhs.$field - self.$field)),+)
			}
			pub fn dist(self, to: $ty<T>) -> T where T: Float {
				self.dist_sqr(to).sqrt()
			}
			pub fn norm(self) -> $ty<T> where T: Float {
				self / self.len()
			}
			pub fn resize(self, len: T) -> $ty<T> where T: Float {
				self * (len / self.len())
			}
			pub fn dot(self, rhs: $ty<T>) -> T {
				infix!(+ $(self.$field * rhs.$field),+)
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T: Unit> $ty<T> {
			/// Component wise absolute value.
			pub fn abs(self) -> $ty<T> {
				$ty { $($field: self.$field.abs()),+ }
			}
			pub fn sqr(self) -> $ty<T> {
				$ty { $($field: self.$field * self.$field),+ }
			}
			pub fn min(self, rhs: $ty<T>) -> $ty<T> {
				$ty { $($field: Unit::min(self.$field, rhs.$field)),+ }
			}
			pub fn max(self, rhs: $ty<T>) -> $ty<T> {
				$ty { $($field: Unit::max(self.$field, rhs.$field)),+ }
			}
			pub fn mul_add(self, vec: $ty<T>, scale: T) -> $ty<T> {
				$ty { $($field: self.$field + vec.$field * scale),+ }
			}
		}

		impl<T: ops::Add<Output = T>> ops::Add<$ty<T>> for $ty<T> {
			type Output = $ty<T>;
			fn add(self, rhs: $ty<T>) -> $ty<T> {
				$ty { $($field: self.$field + rhs.$field),+ }
			}
		}
		impl<T: ops::Sub<Output = T>> ops::Sub<$ty<T>> for $ty<T> {
			type Output = $ty<T>;
			fn sub(self, rhs: $ty<T>) -> $ty<T> {
				$ty { $($field: self.$field - rhs.$field),+ }
			}
		}
		impl<T: ops::Neg<Output = T>> ops::Neg for $ty<T> {
			type Output = $ty<T>;
			fn neg(self) -> $ty<T> {
				$ty { $($field: -self.$field),+ }
			}
		}

		impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for $ty<T> {
			type Output = $ty<T>;
			fn mul(self, rhs: T) -> $ty<T> {
				$ty { $($field: self.$field * rhs),+ }
			}
		}
		impl<T: Copy + ops::Div<Output = T>> ops::Div<T> for $ty<T> {
			type Output = $ty<T>;
			fn div(self, rhs: T) -> $ty<T> {
				$ty { $($field: self.$field / rhs),+ }
			}
		}
		impl<T: Copy + ops::Rem<Output = T>> ops::Rem<T> for $ty<T> {
			type Output = $ty<T>;
			fn rem(self, rhs: T) -> $ty<T> {
				$ty { $($field: self.$field % rhs),+ }
			}
		}
	}
}

// vec!(Vec1 { x 0 T } 1);
vec!(Vec2 { x 0 T, y 1 T } 2);
vec!(Vec3 { x 0 T, y 1 T, z 2 T } 3);
vec!(Vec4 { x 0 T, y 1 T, z 2 T, w 3 T } 4);

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn units() {
		let _ = Vec2::new(1, 2);
	}
}

// impl<T: Unit> Vec1<T> {
// 	pub fn unit_x() -> Vec2<T> { Vec2 { x: Unit::one() } }
//	pub fn extend(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y: y } }
// }
impl<T: Unit> Vec2<T> {
	pub fn unit_x() -> Vec2<T> { Vec2 { x: Unit::one(), y: Unit::zero() } }
	pub fn unit_y() -> Vec2<T> { Vec2 { x: Unit::zero(), y: Unit::one() } }
	pub fn vec3(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: z } }
	pub fn cross(self, rhs: Vec2<T>) -> T { self.x * rhs.y - self.y * rhs.x }
	pub fn ccw(self) -> Vec2<T> { Vec2 { x: self.y, y: -self.x } }
	pub fn cw(self) -> Vec2<T> { Vec2 { x: -self.y, y: self.x } }
}
impl<T: Unit> Vec3<T> {
	pub fn unit_x() -> Vec3<T> { Vec3 { x: Unit::one(), y: Unit::zero(), z: Unit::zero() } }
	pub fn unit_y() -> Vec3<T> { Vec3 { x: Unit::zero(), y: Unit::one(), z: Unit::zero() } }
	pub fn unit_z() -> Vec3<T> { Vec3 { x: Unit::zero(), y: Unit::zero(), z: Unit::one() } }
	pub fn vec4(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w: w } }
	pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
	pub fn cross(self, rhs: Vec3<T>) -> Vec3<T> {
		Vec3 {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x,
		}
	}
}
impl<T: Unit> Vec4<T> {
	pub fn unit_x() -> Vec4<T> { Vec4 { x: Unit::one(), y: Unit::zero(), z: Unit::zero(), w: Unit::zero() } }
	pub fn unit_y() -> Vec4<T> { Vec4 { x: Unit::zero(), y: Unit::one(), z: Unit::zero(), w: Unit::zero() } }
	pub fn unit_z() -> Vec4<T> { Vec4 { x: Unit::zero(), y: Unit::zero(), z: Unit::one(), w: Unit::zero() } }
	pub fn unit_w() -> Vec4<T> { Vec4 { x: Unit::zero(), y: Unit::zero(), z: Unit::zero(), w: Unit::one() } }
	pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
	pub fn xyz(self) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: self.z } }
}
