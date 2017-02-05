/*!
Defines the vector types.

All vectors implement the following interfaces:

* Derived traits

  `Copy`, `Clone`, `Debug`, `Default`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`.

  `Display`: Formats the vector as an N-tuple.

* Constructors

  `new(x, y, ...)`: Constructs a new vector from components.

  `dup(u)`: Constructs a new vector by splatting to its components.

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

* Transformations

  `cast<U>(self)`: Casts to a vector of type `U` with the same dimensions.

  `map<F>(self, F)`: Maps a callable over the components.

  `map2<F>(self, rhs, F)`: Maps a callable over the components side by side.

  `reduce<F>(self, F)`: Reduces the vector.

  `fold<F>(self, acc, F)`: Folds the vector.

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

  `norm(self)`: Normalizes the vector given `Float` components.

  `resize(self, len)`: Scales the vector such that its length equals the given value given `Float` components.

  `hadd(self)`: Horizontal adds all components.

  `dot(self, rhs)`: Calculates the inner product.

  * Exclusive to `Vec2`

     `hsub(self)`: Horizontal subtracts the components.

     `ccw(self)`: Rotates the vector counter-clockwise by 90°.

     `cw(self)`: Rotates the vector clockwise by 90°.

     `cross(self, rhs)`: Calculates the 3D cross product where the inputs are extended with `z = 0` and returns the magnitude of the result.

  * Exclusive to `Vec3`

     `cross(self, rhs)`: Calculates the 3D cross product.

* Comparison masks

  `mask<F>(self, F)`: Creates a mask by applying the callable `F` to each component.

  `masked<F>(self, rhs, F)`: Creates a mask by applying the callable `F` to each component on the left and righthand side.

  `is_finite(self)`: Masks if the component is finite.

  `eq(self, rhs)`: Masks if the components are equal.

  `ne(self, rhs)`: Masks if the components are not equal.

  `lt(self, rhs)`: Masks if the lefthand side components are less than the righthand side.

  `le(self, rhs)`: Masks if the lefthand side components are less than or equal the righthand side.

  `gt(self, rhs)`: Masks if the lefthand side components are greater than the righthand side.

  `ge(self, rhs)`: Masks if the lefthand side components are greater than or equal the righthand side.

  `select(self, rhs, mask)`: Combines two vectors based on the mask, selecting components from the lefthand side if `true` and righthand side if `false`.

* Operators

  `abs(self)`: Component wise absolute value.

  `min(self, rhs)`: Component wise minimum value.

  `max(self, rhs)`: Component wise maximum value.

  `mul_add(self, vec, scale)`: Adds the scaled value.

  `Add`: Adds the vectors component wise.

  `Sub`: Subtracts the vectors component wise.

  `Neg`: Negates the vector component wise.

  `Mul`: Multiply by scalar or vector.

  `Div`: Divide by scalar or vector.

  `Rem`: Remainder by scalar or vector.
*/

use ::std::{fmt, mem, ops};

use ::num::{Scalar, Zero, One, Abs, Min, Max, Float, Cast};

// /// A 1-dimensional vector.
// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
// #[repr(C)]
// pub struct Vec1<T> {
// 	pub x: T,
// }

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
		pub fn unit_x() -> Vec1<T> where T: Zero + One { Vec1 { x: T::one() } }
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

macro_rules! fmt_str {
	(Vec1) => ("({})");
	(Vec2) => ("({}, {})");
	(Vec3) => ("({}, {}, {})");
	(Vec4) => ("({}, {}, {}, {})");
	(Vec1 .*) => ("({:.*})");
	(Vec2 .*) => ("({:.*}, {:.*})");
	(Vec3 .*) => ("({:.*}, {:.*}, {:.*})");
	(Vec4 .*) => ("({:.*}, {:.*}, {:.*}, {:.*})");
}

// This may or may not be horrible abuse of the `macro_rules!` system :)
macro_rules! vec {
	($vec:ident $N:tt { $($field:ident $I:tt $T:ident),+ }) => {

		//----------------------------------------------------------------
		// Constructors

		impl<T> $vec<T> {
			/// Constructs a new vector from components.
			pub fn new($($field: T),+) -> $vec<T> {
				$vec { $($field: $field),+ }
			}
			/// Constructs a new vector by broadcasting to all its components.
			pub fn dup(u: T) -> $vec<T> where T: Copy {
				$vec { $($field: u),+ }
			}
			/// Constructs a new zero vector.
			pub fn zero() -> $vec<T> where T: Zero {
				$vec { $($field: Zero::zero()),+ }
			}
		}

		impl<T> $vec<T> {
			unit!($vec);
		}
		impl<T> $vec<T> {
			set!($vec);
		}
		impl<T> $vec<T> {
			cvt!($vec);
		}

		//----------------------------------------------------------------
		// Transformations

		impl<T> $vec<T> {
			/// Casts to a vector of different type with the same dimensions.
			pub fn cast<U>(self) -> $vec<U> where T: Cast<U> {
				$vec { $($field: self.$field.cast()),+ }
			}
			/// Maps a callable over the components.
			pub fn map<F: FnMut(T) -> T>(self, mut f: F) -> $vec<T> {
				$vec { $($field: f(self.$field)),+ }
			}
			/// Maps a callable over the components side by side.
			pub fn map2<F: FnMut(T, T) -> T>(self, rhs: $vec<T>, mut f: F) -> $vec<T> {
				$vec { $($field: f(self.$field, rhs.$field)),+ }
			}
			/// Reduces the vector.
			pub fn reduce<F: Fn(T, T) -> T>(self, f: F) -> T {
				// These will end up nested without temporaries which won't work with `FnMut`...
				fold!(f, $(self.$field),+)
			}
			/// Folds the vector.
			pub fn fold<F: Fn(T, T) -> T>(self, acc: T, f: F) -> T {
				// These will end up nested without temporaries which won't work with `FnMut`...
				fold!(f, acc, $(self.$field),+)
			}
		}

		//----------------------------------------------------------------
		// Conversions

		impl<T> From<($($T,)+)> for $vec<T> {
			fn from(val: ($($T,)+)) -> $vec<T> {
				$vec { $($field: val.$I),+ }
			}
		}
		impl<T> Into<($($T,)+)> for $vec<T> {
			fn into(self) -> ($($T,)+) {
				($(self.$field,)+)
			}
		}

		impl<T: Copy> From<[T; $N]> for $vec<T> {
			fn from(val: [T; $N]) -> $vec<T> {
				// Can't move out of array...
				$vec { $($field: val[$I]),+ }
			}
		}
		impl<T> Into<[T; $N]> for $vec<T> {
			fn into(self) -> [T; $N] {
				[$(self.$field),+]
			}
		}

		//----------------------------------------------------------------
		// As references

		impl<T> AsRef<($($T,)+)> for $vec<T> {
			fn as_ref(&self) -> &($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T; $N]> for $vec<T> {
			fn as_ref(&self) -> &[T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T]> for $vec<T> {
			fn as_ref(&self) -> &[T] {
				<Self as AsRef<[T; $N]>>::as_ref(self)
			}
		}
		
		impl<T> AsMut<($($T,)+)> for $vec<T> {
			fn as_mut(&mut self) -> &mut ($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T; $N]> for $vec<T> {
			fn as_mut(&mut self) -> &mut [T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T]> for $vec<T> {
			fn as_mut(&mut self) -> &mut [T] {
				<Self as AsMut<[T; $N]>>::as_mut(self)
			}
		}

		//----------------------------------------------------------------
		// Operations

		impl<T: Scalar> $vec<T> {
			/// Squares the components.
			pub fn sqr(self) -> $vec<T> {
				$vec { $($field: self.$field * self.$field),+ }
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
			pub fn dist_sqr(self, to: $vec<T>) -> T {
				infix!(+ $((to.$field - self.$field) * (to.$field - self.$field)),+)
			}
			/// Calculates the euclidean distance to another vector.
			pub fn dist(self, to: $vec<T>) -> T where T: Float {
				self.dist_sqr(to).sqrt()
			}
			/// Normalizes the vector.
			pub fn norm(self) -> $vec<T> where T: Float {
				let self_len = self.len();
				if self_len == T::zero() {
					Self::zero()
				}
				else {
					self / self_len
				}
			}
			/// Scales the vector such that its length equals the given value.
			pub fn resize(self, len: T) -> $vec<T> where T: Float {
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
			ops!($vec);
			/// Calculates the inner product.
			pub fn dot(self, rhs: $vec<T>) -> T {
				infix!(+ $(self.$field * rhs.$field),+)
			}
		}

		//----------------------------------------------------------------
		// Comparison masks

		impl<T> $vec<T> {
			/// Creates a mask by applying the callable `F` to each component.
			pub fn mask<F: FnMut(T) -> bool>(self, mut f: F) -> $vec<bool> {
				$vec { $($field: f(self.$field)),+ }
			}
			/// Creates a mask by applying the callable `F` to each component on the left and righthand side.
			pub fn masked<F: FnMut(T, T) -> bool>(self, rhs: $vec<T>, mut f: F) -> $vec<bool> {
				$vec { $($field: f(self.$field, rhs.$field)),+ }
			}
			/// Masks if the component is finite.
			pub fn is_finite(self) -> $vec<bool> where T: Float {
				$vec { $($field: self.$field.is_finite()),+ }
			}
			/// Masks if the components are equal.
			pub fn eq(self, rhs: $vec<T>) -> $vec<bool> where T: PartialEq {
				$vec { $($field: self.$field == rhs.$field),+ }
			}
			/// Masks if the components are not equal.
			pub fn ne(self, rhs: $vec<T>) -> $vec<bool> where T: PartialEq {
				$vec { $($field: self.$field != rhs.$field),+ }
			}
			/// Masks if the lefthand side components are less than the righthand side.
			pub fn lt(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field < rhs.$field),+ }
			}
			/// Masks if the lefthand side components are less than or equal the righthand side.
			pub fn le(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field <= rhs.$field),+ }
			}
			/// Masks if the lefthand side components are greater than the righthand side.
			pub fn gt(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field > rhs.$field),+ }
			}
			/// Masks if the lefthand side components are greater than or equal the righthand side.
			pub fn ge(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field >= rhs.$field),+ }
			}
			/// Combines two vectors based on the mask, selecting components from the lefthand side if `true` and righthand side if `false`.
			pub fn select(self, rhs: $vec<T>, mask: $vec<bool>) -> $vec<T> {
				$vec { $($field: if mask.$field { self.$field } else { rhs.$field }),+ }
			}
		}

		//----------------------------------------------------------------
		// Comparison Operators

		impl $vec<bool> {
			/// Returns `true` if any of the components is `true`.
			pub fn any(self) -> bool {
				infix!(|| $(self.$field),+)
			}
			/// Returns `true` if all the components are `true`.
			pub fn all(self) -> bool {
				infix!(&& $(self.$field),+)
			}
			/// Returns `true` if none of the components are `true`.
			pub fn none(self) -> bool {
				!self.any()
			}
		}

		impl ops::BitAnd<$vec<bool>> for $vec<bool> {
			type Output = $vec<bool>;
			fn bitand(self, rhs: $vec<bool>) -> $vec<bool> {
				$vec { $($field: self.$field && rhs.$field),+ }
			}
		}
		impl ops::BitOr<$vec<bool>> for $vec<bool> {
			type Output = $vec<bool>;
			fn bitor(self, rhs: $vec<bool>) -> $vec<bool> {
				$vec { $($field: self.$field || rhs.$field),+ }
			}
		}
		impl ops::BitXor<$vec<bool>> for $vec<bool> {
			type Output = $vec<bool>;
			fn bitxor(self, rhs: $vec<bool>) -> $vec<bool> {
				$vec { $($field: self.$field != rhs.$field),+ }
			}
		}
		impl ops::Not for $vec<bool> {
			type Output = $vec<bool>;
			fn not(self) -> $vec<bool> {
				$vec { $($field: !self.$field),+ }
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T> $vec<T> {
			/// Component wise absolute value.
			pub fn abs(self) -> $vec<T> where T: Abs<Output = T> {
				$vec { $($field: self.$field.abs()),+ }
			}
			/// Component wise minimum value.
			pub fn min(self, rhs: $vec<T>) -> $vec<T> where T: Min<Output = T> {
				$vec { $($field: T::min(self.$field, rhs.$field)),+ }
			}
			/// Component wise maximum value.
			pub fn max(self, rhs: $vec<T>) -> $vec<T> where T: Max<Output = T> {
				$vec { $($field: T::max(self.$field, rhs.$field)),+ }
			}
			/// Adds the scaled vector.
			pub fn mul_add(self, vec: $vec<T>, scale: T) -> $vec<T> where T: Scalar {
				$vec { $($field: self.$field + vec.$field * scale),+ }
			}
		}

		impl<T: ops::Add<Output = T>> ops::Add<$vec<T>> for $vec<T> {
			type Output = $vec<T>;
			fn add(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: self.$field + rhs.$field),+ }
			}
		}
		impl<T: ops::Sub<Output = T>> ops::Sub<$vec<T>> for $vec<T> {
			type Output = $vec<T>;
			fn sub(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: self.$field - rhs.$field),+ }
			}
		}
		impl<T: ops::Neg<Output = T>> ops::Neg for $vec<T> {
			type Output = $vec<T>;
			fn neg(self) -> $vec<T> {
				$vec { $($field: -self.$field),+ }
			}
		}

		impl<T: Copy + ops::Mul<Output = T>> ops::Mul<T> for $vec<T> {
			type Output = $vec<T>;
			fn mul(self, rhs: T) -> $vec<T> {
				$vec { $($field: self.$field * rhs),+ }
			}
		}
		impl<T: Copy + ops::Div<Output = T>> ops::Div<T> for $vec<T> {
			type Output = $vec<T>;
			fn div(self, rhs: T) -> $vec<T> {
				$vec { $($field: self.$field / rhs),+ }
			}
		}
		impl<T: Copy + ops::Rem<Output = T>> ops::Rem<T> for $vec<T> {
			type Output = $vec<T>;
			fn rem(self, rhs: T) -> $vec<T> {
				$vec { $($field: self.$field % rhs),+ }
			}
		}

		impl<T: ops::Mul<Output = T>> ops::Mul<$vec<T>> for $vec<T> {
			type Output = $vec<T>;
			fn mul(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: self.$field * rhs.$field),+ }
			}
		}
		impl<T: ops::Div<Output = T>> ops::Div<$vec<T>> for $vec<T> {
			type Output = $vec<T>;
			fn div(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: self.$field / rhs.$field),+ }
			}
		}
		impl<T: ops::Rem<Output = T>> ops::Rem<$vec<T>> for $vec<T> {
			type Output = $vec<T>;
			fn rem(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: self.$field % rhs.$field),+ }
			}
		}

		//----------------------------------------------------------------
		// Formatting

		impl<T: fmt::Display> fmt::Display for $vec<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				// This is unfortunate but floats really want the precision argument...
				match f.precision() {
					Some(p) => write!(f, fmt_str!($vec .*), $(p, self.$field),+),
					None => write!(f, fmt_str!($vec), $(self.$field),+),
				}
			}
		}
	}
}

// vec!(Vec1 1 { x 0 T });
vec!(Vec2 2 { x 0 T, y 1 T });
vec!(Vec3 3 { x 0 T, y 1 T, z 2 T });
vec!(Vec4 4 { x 0 T, y 1 T, z 2 T, w 3 T });

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn constructors() {
		let _ = Vec2::new(1, 2);
	}
	#[test]
	fn formatting() {
		assert_eq!("(3, 4, 5)", format!("{}", Vec3::new(3, 4, 5)));
		assert_eq!("(1.300, 3.142)", format!("{:.3}", Vec2::new(1.3, 3.1415927)));
	}
}
