/*!
Defines the vector types.

All vectors implement the following interfaces:

* Derived traits

  `Copy`, `Clone`, `Debug`, `Default`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`.

  `Display`: Formats the vector as an N-tuple.

* Constructors

  `new(x, y, ...)`: Constructs a new vector from components.

  `dup(u)`: Constructs a new vector by broadcasting all its components.

  `zero()`: Constructs a new zero vector.

* Unit vectors

  `unit_x()`: A unit vector in the `x` direction.

  `unit_y()`: A unit vector in the `y` direction. `Vec2` and up.

  `unit_z()`: A unit vector in the `z` direction. `Vec3` and up.

  `unit_w()`: A unit vector in the `w` direction. `Vec4` only.

* Setting individual components

  Note that these return new vectors with the respective component changed.

  `set_x(self, x)`: Sets the `x` component.

  `set_y(self, y)`: Sets the `y` component. `Vec2` and up.

  `set_z(self, z)`: Sets the `z` component. `Vec3` and up.

  `set_w(self, w)`: Sets the `w` component. `Vec4` only.

* Extending and truncating

  `vec3(self, T)`: Extends `Vec2` with a `z` component.

  `vec4(self, T)`: Extends `Vec3` with a `w` component.

  `xy(self)`: Drops the `z` component from `Vec3` and `w` from `Vec4`.

  `xyz(self)`: Drops the `w` component from `Vec4`.

* Casting vector component types

  `cast<U>(self)`: Casts to a vector of type `U` with the same dimensions.

* Conversions

  `From`: PointN, N-tuple and N-array conversions.

  `Into`: N-tuple and N-array conversions.

  `AsRef`: PointN, N-tuple, N-array and slice conversions.

  `AsMut`: PointN, N-tuple, N-array and slice conversions.

* Operations where T is `Scalar`

  `sqr(self)`: Squares the components.

  `len_sqr(self)`: Calculates the squared length of the vector.

  `len(self)`: Calculates the length of the vector given `Float` components.

  `dist_sqr(self, to)`: Calculates the squared euclidean distance to another vector.

  `dist(self, to)`: Calculates the euclidean distance to another vector given `Float` components.

  `norm(self)`: Normalizes the vector given `Float` components. Undefined for zero length vectors.

  `resize(self, len)`: Scales the vector such that its length equals the given value given `Float` components. Undefined for zero length vectors.

  `hadd(self)`: Horizontal adds all components.

  `dot(self, rhs)`: Calculates the inner product.

  * Exclusive to `Vec2`

     `hsub(self)`: Horizontal subtracts the components.

     `ccw(self)`: Rotates the vector counter-clockwise by 90°.

     `cw(self)`: Rotates the vector clockwise by 90°.

     `cross(self, rhs)`: Calculates the 3D cross product where the inputs are extended with `z = 0` and returns the magnitude of the result.

  * Exclusive to `Vec3`

     `cross(self, rhs)`: Calculates the 3D cross product.

* Operators

  `abs(self)`: Component wise absolute value.

  `min(self, rhs)`: Component wise minimum value.

  `max(self, rhs)`: Component wise maximum value.

  `mul_add(self, vec, scale)`: Adds the scaled value.

  `Add`: Adds the vectors component wise.

  `Sub`: Subtracts the vectors component wise.

  `Neg`: Negates the vector component wise.

  `Mul`: Multiply by scalar.

  `Div`: Divide by scalar.

  `Rem`: Remainder by scalar.
*/

use ::std::{fmt, mem, ops};

use ::num::{Scalar, Zero, One, Abs, Min, Max, Float, Cast};

/// A 2-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

/// A 3-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

/// A 4-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

macro_rules! unit {
	(Vec1) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec1<T> where T: Zero + One { Vec1 { x: Scalar::one() } }
	};
	(Vec2) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec2<T> where T: Zero + One { Vec2 { x: T::one(), y: T::zero() } }
		/// A unit vector in the `y` direction.
		pub fn unit_y() -> Vec2<T> where T: Zero + One { Vec2 { x: T::zero(), y: T::one() } }
	};
	(Vec3) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec3<T> where T: Zero + One { Vec3 { x: T::one(), y: T::zero(), z: T::zero() } }
		/// A unit vector in the `y` direction.
		pub fn unit_y() -> Vec3<T> where T: Zero + One { Vec3 { x: T::zero(), y: T::one(), z: T::zero() } }
		/// A unit vector in the `z` direction.
		pub fn unit_z() -> Vec3<T> where T: Zero + One { Vec3 { x: T::zero(), y: T::zero(), z: T::one() } }
	};
	(Vec4) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec4<T> where T: Zero + One { Vec4 { x: T::one(), y: T::zero(), z: T::zero(), w: T::zero() } }
		/// A unit vector in the `y` direction.
		pub fn unit_y() -> Vec4<T> where T: Zero + One { Vec4 { x: T::zero(), y: T::one(), z: T::zero(), w: T::zero() } }
		/// A unit vector in the `z` direction.
		pub fn unit_z() -> Vec4<T> where T: Zero + One { Vec4 { x: T::zero(), y: T::zero(), z: T::one(), w: T::zero() } }
		/// A unit vector in the `w` direction.
		pub fn unit_w() -> Vec4<T> where T: Zero + One { Vec4 { x: T::zero(), y: T::zero(), z: T::zero(), w: T::one() } }
	};
}

macro_rules! set {
	(Vec1) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec1<T> { Vec1 { x: x } }
	};
	(Vec2) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec2<T> { Vec2 { x: x, y: self.y } }
		/// Sets the `y` component.
		pub fn set_y(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y: y } }
	};
	(Vec3) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec3<T> { Vec3 { x: x, y: self.y, z: self.z } }
		/// Sets the `y` component.
		pub fn set_y(self, y: T) -> Vec3<T> { Vec3 { x: self.x, y: y, z: self.z } }
		/// Sets the `z` component.
		pub fn set_z(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: z } }
	};
	(Vec4) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec4<T> { Vec4 { x: x, y: self.y, z: self.z, w: self.w } }
		/// Sets the `y` component.
		pub fn set_y(self, y: T) -> Vec4<T> { Vec4 { x: self.x, y: y, z: self.z, w: self.w } }
		/// Sets the `z` component.
		pub fn set_z(self, z: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: z, w: self.w } }
		/// Sets the `w` component.
		pub fn set_w(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w: w } }
	};
}

macro_rules! cvt {
	(Vec1) => {
		/// Extends the 1D vector with a `y` component.
		pub fn vec2(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y: y } }
	};
	(Vec2) => {
		/// Extends the 2D vector with a `z` component.
		pub fn vec3(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: z } }
	};
	(Vec3) => {
		/// Extends the 3D vector with a `w` component.
		pub fn vec4(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w: w } }
		/// Drops the `z` component.
		pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
	};
	(Vec4) => {
		/// Drops the `z` and `w` coordinates.
		pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
		/// Drops the `w` component.
		pub fn xyz(self) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: self.z } }
	};
}

macro_rules! ops {
	(Vec1) => {};
	(Vec2) => {
		/// Horizontal subtracts the components.
		pub fn hsub(self) -> T { self.x - self.y }
		/// Rotates the vector counter-clockwise by 90°.
		pub fn ccw(self) -> Vec2<T> { Vec2 { x: self.y, y: -self.x } }
		/// Rotates the vector clockwise by 90°.
		pub fn cw(self) -> Vec2<T> { Vec2 { x: -self.y, y: self.x } }
		/// Calculates the 3D cross product where the inputs are extended with `z = 0` and returns the magnitude of the result.
		pub fn cross(self, rhs: Vec2<T>) -> T { self.x * rhs.y - self.y * rhs.x }
	};
	(Vec3) => {
		/// Calculates the 3D cross product.
		pub fn cross(self, rhs: Vec3<T>) -> Vec3<T> {
			Vec3 {
				x: self.y * rhs.z - self.z * rhs.y,
				y: self.z * rhs.x - self.x * rhs.z,
				z: self.x * rhs.y - self.y * rhs.x,
			}
		}
	};
	(Vec4) => {};
}

macro_rules! display {
	(Vec1) => {
		impl<T: fmt::Display> fmt::Display for Vec1<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "({})", self.x)
			}
		}
	};
	(Vec2) => {
		impl<T: fmt::Display> fmt::Display for Vec2<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "({}, {})", self.x, self.y)
			}
		}
	};
	(Vec3) => {
		impl<T: fmt::Display> fmt::Display for Vec3<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "({}, {}, {})", self.x, self.y, self.z)
			}
		}
	};
	(Vec4) => {
		impl<T: fmt::Display> fmt::Display for Vec4<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
			}
		}
	};
}

// This may or may not be horrible abuse of the `macro_rules!` system :)
macro_rules! vec {
	($ty:ident { $($field:ident $I:tt $T:ident),+ } $N:expr) => {

		//----------------------------------------------------------------
		// Constructors

		impl<T> $ty<T> {
			/// Constructs a new vector from components.
			pub fn new($($field: T),+) -> $ty<T> {
				$ty { $($field: $field),+ }
			}
			/// Constructs a new vector by broadcasting to all its components.
			pub fn dup(u: T) -> $ty<T> where T: Copy {
				$ty { $($field: u),+ }
			}
			/// Constructs a new zero vector.
			pub fn zero() -> $ty<T> where T: Zero {
				$ty { $($field: Zero::zero()),+ }
			}
		}

		impl<T> $ty<T> {
			unit!($ty);
		}
		impl<T> $ty<T> {
			set!($ty);
		}
		impl<T> $ty<T> {
			cvt!($ty);
		}
		
		//----------------------------------------------------------------
		// Conversions

		impl<T> $ty<T> {
			/// Casts to a vector of different type with the same dimensions.
			pub fn cast<U>(self) -> $ty<U> where T: Cast<U> {
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

		impl<T: Scalar> $ty<T> {
			/// Squares the components.
			pub fn sqr(self) -> $ty<T> {
				$ty { $($field: self.$field * self.$field),+ }
			}
			/// Calculates the squared length of the vector.
			pub fn len_sqr(self) -> T {
				infix!(+ $(self.$field * self.$field),+)
			}
			/// Calculates the length of the vector.
			pub fn len(self) -> T where T: Float {
				self.len_sqr().sqrt()
			}
			/// Calculates the squared euclidean distance to another vector.
			pub fn dist_sqr(self, to: $ty<T>) -> T {
				infix!(+ $((to.$field - self.$field) * (to.$field - self.$field)),+)
			}
			/// Calculates the euclidean distance to another vector.
			pub fn dist(self, to: $ty<T>) -> T where T: Float {
				self.dist_sqr(to).sqrt()
			}
			/// Normalizes the vector.
			pub fn norm(self) -> $ty<T> where T: Float {
				let self_len = self.len();
				if self_len == T::zero() {
					Self::zero()
				}
				else {
					self / self_len
				}
			}
			/// Scales the vector such that its length equals the given value.
			pub fn resize(self, len: T) -> $ty<T> where T: Float {
				let self_len = self.len();
				if self_len == T::zero() {
					Self::zero()
				}
				else {
					self * (len / self_len)
				}
			}
			/// Horizontal adds all components.
			pub fn hadd(self) -> T {
				infix!(+ $(self.$field),+)
			}
			ops!($ty);
			/// Calculates the inner product.
			pub fn dot(self, rhs: $ty<T>) -> T {
				infix!(+ $(self.$field * rhs.$field),+)
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T> $ty<T> {
			/// Component wise absolute value.
			pub fn abs(self) -> $ty<T> where T: Abs<Output = T> {
				$ty { $($field: self.$field.abs()),+ }
			}
			/// Component wise minimum value.
			pub fn min(self, rhs: $ty<T>) -> $ty<T> where T: Min<Output = T> {
				$ty { $($field: T::min(self.$field, rhs.$field)),+ }
			}
			/// Component wise maximum value.
			pub fn max(self, rhs: $ty<T>) -> $ty<T> where T: Max<Output = T> {
				$ty { $($field: T::max(self.$field, rhs.$field)),+ }
			}
			/// Adds the scaled vector.
			pub fn mul_add(self, vec: $ty<T>, scale: T) -> $ty<T> where T: Scalar {
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

		//----------------------------------------------------------------
		// Display

		display!($ty);
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
