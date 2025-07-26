/*!
Defines the vector types.

An overview of their implementations:

## Derived traits

`Copy`, `Clone` where T: `Copy`, `Clone`: For convenience all instances are passed by value, greatly simplifying usage.

`Debug`, `Display` where T: `Debug`, `Display`: Struct-based formatting and tuple-based formatting respectively.

`Eq`, `PartialEq` where T: `Eq`, `PartialEq`: Compares if _all_ the underlying components are equal.

`Ord`, `PartialOrd` where T: `Ord`, `PartialOrd`

`Hash` where T: `Hash`

### Examples

```
use cvmath::{Vec2, Vec3};

assert_eq!("Vec3(2, 3, 4)", format!("{}", Vec3(2, 3, 4)));
assert_eq!("Vec2(2.300, 2.142)", format!("{:.3}", Vec2(2.3, 2.14159278)));
assert_eq!("Vec2(16, 25)", format!("{:?}", Vec2(16, 25)));
assert_eq!("Vec3(  2,   3,  14)", format!("{:>3}", Vec3(2, 3, 14)));

assert_eq!(Vec2 { x: -5, y: 9 }, Vec2(-5, 9));
assert!(Vec2(1, 9) > Vec2(1, -2));
```

## Constructors

`new(x, y, ...)`: Constructs a new vector from components.

`dup(u)` where T: `Copy`: Constructs a new vector by splatting to its components.

`X`, `Y`, `Z`, `W` where T: `Zero` + `One`: Constructs a unit vector along the given axis (given that axis exists for the vector).

`set_x(self, x)`, `set_y(self, y)`, `set_z(self, z)`, `set_w(self, w)`: Note that these return new vectors with the respective component changed.

`get(self, c)`: Gets a component generically.

`shuffle(self, x, y, ..)`: Shuffles the components.

### Examples

```
use cvmath::{Vec2, Vec3, X, Y, Z};

assert_eq!(Vec2 { x: 1, y: 2 }, Vec2(1, 2));

assert_eq!(Vec3 { x: 42, y: 42, z: 42 }, Vec3::dup(42));

assert_eq!(Vec2 { x: 0.0, y: 1.0 }, Vec2::Y);
assert_eq!(Vec3 { x: 1, y: 0, z: 0 }, Vec3::X);

assert_eq!(Vec3 { x: -12, y: 0, z: 12 }, Vec3::default().set_x(-12).set_z(12));

assert_eq!(2, Vec2 { x: 13, y: 2}.get(Y));

assert_eq!(Vec3 { x: 5, y: -2, z: 5 }, Vec3(-2, 12, 5).shuffle(Z, X, Z));
```

## Extending and truncating

`vec3(self, T)`: Extends `Vec2` with a `z` component.

`vec4(self, T)`: Extends `Vec3` with a `w` component.

`xy(self)`: Drops the `z` component from `Vec3` and `w` from `Vec4`.

`xyz(self)`: Drops the `w` component from `Vec4`.

### Examples

```
use cvmath::{Vec2, Vec3, Vec4};

assert_eq!(Vec3 { x: 3, y: 4, z: 5 }, Vec2(3, 4).vec3(5));

assert_eq!(Vec4 { x: -1, y: -2, z: -3, w: -4 }, Vec3(-1, -2, -3).vec4(-4));

assert_eq!(Vec2 { x: 2, y: 1 }, Vec3(2, 1, 0).xy());
assert_eq!(Vec2 { x: 1, y: 2 }, Vec4(1, 2, 3, 4).xy());

assert_eq!(Vec3 { x: 1, y: 2, z: 3 }, Vec4(1, 2, 3, 4).xyz());
```

## Transformations

`cast<U>(self)` where T: `CastTo<U>`: Casts to a vector of type `U` with the same dimensions.

`map<U, F>(self, F)` where F: `FnMut(T) -> U`: Maps a callable over the components.

`zip<U, F>(self, rhs, F)` where F: `FnMut(T, T) -> U`: Zips two vectors together.

`reduce<F>(self, F)` where F: `Fn(T, T) -> T`: Reduces the vector. The `x` component is used as the initial value of the accumulator.

`fold<A, F>(self, acc, F)` where F: `Fn(A, T) -> A`: Folds the vector.

### Examples

```
use cvmath::{Vec2, Vec3};

assert_eq!(Vec2 { x: 2, y: 4 }, Vec2(2.2, 4.9).cast());

assert_eq!(Vec2 { x: 2, y: 4 }, Vec2(1, 2).map(|c| c * 2));

let left = Vec2(1, 2);
let right = Vec2(1, -1);
assert_eq!(Vec2 { x: 3, y: 3 }, Vec2::zip(left, right, |a, b| a * 2 + b));

let vec = Vec3(5, 3, 2);
assert_eq!(0, vec.reduce(|acc, c| acc - c));
assert_eq!(-10, vec.fold(0, |acc, c| acc - c));
```

## Conversions

`From`, `Into`: PointN, N-tuple and N-array conversions.

`AsRef`, `AsMut`: PointN, N-tuple, N-array and slice conversions.

### Examples

```
use cvmath::Vec2;

assert_eq!(Vec2::<i32>::from((2, 3)), Vec2::from([2, 3]));
```

## Operations where T is `Scalar`

`sqr(self)`: Squares the components.

`len_sqr(self)`: Calculates the squared length of the vector.

`len(self)` where T: `Float`: Calculates the length of the vector.

`distance_sqr(self, to)`: Calculates the squared euclidean distance to another vector.

`distance(self, to)` where T: `Float`: Calculates the euclidean distance to another vector.

`norm(self)` where T: `Float`: Normalizes the vector. The vector with length zero stays zero.

`resize(self, len)` where T: `Float`: Scales the vector such that its length equals the given value. The vector with length zero remains zero.

`project_scalar(self, v)` where T: `Float`: Scalar projection of `self` onto `v`.

`project(self, v)` where T: `Float`: Projection of `self` onto `v`.

`project_sat(self, v)` where T: `Float`: Saturated projection of `self` onto `v`.

`dot(self, rhs)`: Calculates the inner product.

`cos_angle(self, rhs)`: Calculates the cosine of the inner angle.

`angle(self, rhs)`: Calculates the inner angle.

`hadd(self)`: Horizontal adds all components.

`abs(self)`: Component-wise absolute value.

`min(self, rhs)`: Component-wise minimum value.

`vmin(self)`: Horizontal minimum value.

`max(self, rhs)`: Component-wise maximum value.

`vmax(self)`: Horizontal maximum value.

`mul_add(self, vec, scale)`: Adds the scaled value.

Exclusive to `Vec2`:

`polar_angle(self)`: Calculates the polar angle.

`signed_angle(self, rhs)`: Calculates the signed angle between two vectors.

`ccw(self)`: Rotates the vector counter-clockwise by 90°.

`cw(self)`: Rotates the vector clockwise by 90°.

`cross(self, rhs)`: Calculates the 3D cross product where the inputs are extended with `z = 0` and returns the magnitude of the result.

`hsub(self)`: Horizontal subtracts y from x.

Exclusive to `Vec3`:

`cross(self, rhs)`: Calculates the 3D cross product.

`signed_angle(self, rhs)`: Calculates the signed angle between two vectors in 3D space.

`any_perp(self)`: Returns a normalized vector perpendicular to the vector.

### Examples

```
use cvmath::{Vec2, Vec3};

assert_eq!(Vec2 { x: 9, y: 16 }, Vec2(3, 4).sqr());

assert_eq!(25, Vec2(3, 4).len_sqr());
assert_eq!(5.0, Vec2(3.0, 4.0).len());

assert_eq!(2, Vec2::distance_sqr(Vec2(1, 1), Vec2(2, 2)));
assert_eq!(5.0, Vec2::distance(Vec2(10.0, 10.0), Vec2(13.0, 14.0)));

assert_eq!(Vec2 { x: 0.6, y: 0.8 }, Vec2(3.0, 4.0).norm());
assert_eq!(Vec2 { x: 0.0, y: 0.0 }, Vec2(0.0, 0.0).norm());

assert_eq!(Vec2 { x: 1.5, y: 2.0 }, Vec2(3.0, 4.0).resize(2.5));
assert_eq!(Vec2 { x: 0.0, y: 0.0 }, Vec2(0.0, 0.0).resize(2.0));

assert_eq!(2.0, Vec2(2.0, 1.0).project_scalar(Vec2(3.0, 4.0)));
assert_eq!(2.0, Vec2(2.0, 1.0).project(Vec2(3.0, 4.0)).len());
assert_eq!(Vec2(-3.0, -4.0), Vec2(-5.0, -2.5).project(Vec2(3.0, 4.0)));
assert_eq!(Vec2(0.0, 0.0), Vec2(-5.0, -2.5).project_sat(Vec2(3.0, 4.0)));

assert_eq!(2.2, Vec2(1.0, 2.0).project_scalar(Vec2(3.0, 4.0)));
assert_eq!(2.0, Vec3(1.0, 4.0, 0.0).project_scalar(Vec3(4.0, 2.0, 4.0)));

assert_eq!(12, Vec3(3, 4, 5).hadd());
assert_eq!(-1, Vec2(3, 4).hsub());

assert_eq!(Vec2 { x: 4, y: -3 }, Vec2(3, 4).ccw());
assert_eq!(Vec2 { x: -4, y: 3 }, Vec2(3, 4).cw());
assert_eq!(10, Vec2::cross(Vec2(3, 4), Vec2(-1, 2)));

assert_eq!(12, Vec3::dot(Vec3(1, 2, 3), Vec3(4, -5, 6)));
assert_eq!(Vec3 { x: -12, y: 1, z: 39 }, Vec3::cross(Vec3(3, -3, 1), Vec3(4, 9, 1)));
```

## Operators

`Add`: Adds the vectors component-wise.

`Sub`: Subtracts the vectors component-wise.

`Neg`: Negates the vector component-wise.

`Mul`: Multiply by scalar or vector.

`Div`: Divide by scalar or vector.

`Rem`: Remainder by scalar or vector.

### Examples

*/

use super::*;

// /// A 1-dimensional vector.
// #[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
// #[repr(C)]
// pub struct Vec1<T> {
// 	pub x: T,
// }

/// Vec2 vector.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

/// Vec3 vector.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

/// Vec4 vector.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

/// X component.
pub struct X;
/// Y component.
pub struct Y;
/// Z component.
pub struct Z;
/// W component.
pub struct W;

/// Access the components of a vector generically.
///
/// Implementation helper for other functions.
pub trait ComponentImpl<T, C> {
	#[must_use]
	fn get(self) -> T;
}

impl<T> ComponentImpl<T, X> for Vec2<T> {
	#[inline]
	fn get(self) -> T { self.x }
}
impl<T> ComponentImpl<T, Y> for Vec2<T> {
	#[inline]
	fn get(self) -> T { self.y }
}

impl<T> ComponentImpl<T, X> for Vec3<T> {
	#[inline]
	fn get(self) -> T { self.x }
}
impl<T> ComponentImpl<T, Y> for Vec3<T> {
	#[inline]
	fn get(self) -> T { self.y }
}
impl<T> ComponentImpl<T, Z> for Vec3<T> {
	#[inline]
	fn get(self) -> T { self.z }
}

impl<T> ComponentImpl<T, X> for Vec4<T> {
	#[inline]
	fn get(self) -> T { self.x }
}
impl<T> ComponentImpl<T, Y> for Vec4<T> {
	#[inline]
	fn get(self) -> T { self.y }
}
impl<T> ComponentImpl<T, Z> for Vec4<T> {
	#[inline]
	fn get(self) -> T { self.z }
}
impl<T> ComponentImpl<T, W> for Vec4<T> {
	#[inline]
	fn get(self) -> T { self.w }
}

macro_rules! unit {
	(Vec1) => {
		/// Unit vector in the `x` direction.
		pub const X: Vec1<T> = Vec1 { x: T::ONE };
	};
	(Vec2) => {
		/// Unit vector in the `x` direction.
		pub const X: Vec2<T> = Vec2 { x: T::ONE, y: T::ZERO };
		/// Unit vector in the `y` direction.
		pub const Y: Vec2<T> = Vec2 { x: T::ZERO, y: T::ONE };
	};
	(Vec3) => {
		/// Unit vector in the `x` direction.
		pub const X: Vec3<T> = Vec3 { x: T::ONE, y: T::ZERO, z: T::ZERO };
		/// Unit vector in the `y` direction.
		pub const Y: Vec3<T> = Vec3 { x: T::ZERO, y: T::ONE, z: T::ZERO };
		/// Unit vector in the `z` direction.
		pub const Z: Vec3<T> = Vec3 { x: T::ZERO, y: T::ZERO, z: T::ONE };
	};
	(Vec4) => {
		/// Unit vector in the `x` direction.
		pub const X: Vec4<T> = Vec4 { x: T::ONE, y: T::ZERO, z: T::ZERO, w: T::ZERO };
		/// Unit vector in the `y` direction.
		pub const Y: Vec4<T> = Vec4 { x: T::ZERO, y: T::ONE, z: T::ZERO, w: T::ZERO };
		/// Unit vector in the `z` direction.
		pub const Z: Vec4<T> = Vec4 { x: T::ZERO, y: T::ZERO, z: T::ONE, w: T::ZERO };
		/// Unit vector in the `w` direction.
		pub const W: Vec4<T> = Vec4 { x: T::ZERO, y: T::ZERO, z: T::ZERO, w: T::ONE };
	};
}

macro_rules! with {
	(Vec1) => {
		/// Sets the `x` component.
		#[inline]
		#[must_use]
		pub fn set_x(self, x: T) -> Vec1<T> { Vec1 { x } }
	};
	(Vec2) => {
		/// Sets the `x` component.
		#[inline]
		#[must_use]
		pub fn set_x(self, x: T) -> Vec2<T> { Vec2 { x, y: self.y } }
		/// Sets the `y` component.
		#[inline]
		#[must_use]
		pub fn set_y(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y } }
	};
	(Vec3) => {
		/// Sets the `x` component.
		#[inline]
		#[must_use]
		pub fn set_x(self, x: T) -> Vec3<T> { Vec3 { x, y: self.y, z: self.z } }
		/// Sets the `y` component.
		#[inline]
		#[must_use]
		pub fn set_y(self, y: T) -> Vec3<T> { Vec3 { x: self.x, y, z: self.z } }
		/// Sets the `z` component.
		#[inline]
		#[must_use]
		pub fn set_z(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z } }
	};
	(Vec4) => {
		/// Sets the `x` component.
		#[inline]
		#[must_use]
		pub fn set_x(self, x: T) -> Vec4<T> { Vec4 { x, y: self.y, z: self.z, w: self.w } }
		/// Sets the `y` component.
		#[inline]
		#[must_use]
		pub fn set_y(self, y: T) -> Vec4<T> { Vec4 { x: self.x, y, z: self.z, w: self.w } }
		/// Sets the `z` component.
		#[inline]
		#[must_use]
		pub fn set_z(self, z: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z, w: self.w } }
		/// Sets the `w` component.
		#[inline]
		#[must_use]
		pub fn set_w(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w } }
	};
}

macro_rules! cvt {
	(Vec1) => {
		/// Extends the 1D vector with a `y` component.
		#[inline]
		#[must_use]
		pub fn vec2(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y } }
	};
	(Vec2) => {
		/// Extends the 2D vector with a `z` component.
		#[inline]
		#[must_use]
		pub fn vec3(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z } }
		/// Extends the 2D vector with a `z` and `w` component.
		#[inline]
		#[must_use]
		pub fn vec4(self, z: T, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z, w } }
	};
	(Vec3) => {
		/// Extends the 3D vector with a `w` component.
		#[inline]
		#[must_use]
		pub fn vec4(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w } }
		/// Drops the `z` component.
		#[inline]
		#[must_use]
		pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
	};
	(Vec4) => {
		/// Drops the `z` and `w` coordinates.
		#[inline]
		#[must_use]
		pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
		/// Drops the `w` component.
		#[inline]
		#[must_use]
		pub fn xyz(self) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: self.z } }
	};
}

macro_rules! fmt {
	($ty:ident { $($field:ident),+ }) => {
		fmt!($ty { $($field),+ } fmt::Display);
		fmt!($ty { $($field),+ } fmt::Debug);
		fmt!($ty { $($field),+ } fmt::Binary);
		fmt!($ty { $($field),+ } fmt::Octal);
		fmt!($ty { $($field),+ } fmt::LowerHex);
		fmt!($ty { $($field),+ } fmt::UpperHex);
		fmt!($ty { $($field),+ } fmt::LowerExp);
		fmt!($ty { $($field),+ } fmt::UpperExp);
	};
	($ty:ident { $($field:ident),+ } $fmt:path) => {
		impl<T: $fmt> $fmt for $ty<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.write_str(concat!(stringify!($ty), "("))?;
				instmt!(f.write_str(", ")?; $(self.$field.fmt(f)?;)+);
				f.write_str(")")
			}
		}
	};
}

macro_rules! parse_vec_elems {
	($s:ident, $iter:ident, $next:ident; $field:ident, $($tail:ident),+) => {{
		$field = {
			let start = $next;
			let end = $iter.next().ok_or(ParseVecError::DimMismatch)?;
			$next = end + 1;
			$s[start..end].trim_ascii().parse()?
		};
		parse_vec_elems!($s, $iter, $next; $($tail),+);
	}};
	($s:ident, $iter:ident, $next:ident; $field:ident) => {{
		$field = {
			if $iter.next().is_some() {
				return Err(ParseVecError::DimMismatch);
			}
			$s[$next..$s.len() - 1].trim_ascii().parse()?
		};
	}};
}

// This may or may not be horrible abuse of the `macro_rules!` system :)
macro_rules! vec {
	(
		$vec:ident $N:tt
		{ $($field:ident $I:tt $T:ident $U:ident $C:ident),+ }
		{ $($ops:tt)* }
	) => {

		#[cfg(feature = "dataview")]
		unsafe impl<T> dataview::Pod for $vec<T> where T: dataview::Pod {}

		//----------------------------------------------------------------
		// Constructors

		impl<T> $vec<T> {
			/// Constructs a new vector from components.
			#[inline]
			#[must_use]
			pub const fn new($($field: T),+) -> $vec<T> {
				$vec { $($field),+ }
			}
			/// Constructs a new vector by broadcasting to all its components.
			#[inline]
			#[must_use]
			pub const fn dup(u: T) -> $vec<T> where T: Copy {
				$vec { $($field: u),+ }
			}
		}

		impl<T: Zero> $vec<T> {
			/// Returns the origin for the vector space.
			#[inline]
			#[must_use]
			pub const fn zero() -> $vec<T> {
				$vec { $($field: T::ZERO),+ }
			}
			#[doc = stringify!($vec)]
			#[doc = " of all zero."]
			pub const ZERO: $vec<T> = $vec { $($field: T::ZERO),+ };
		}
		impl<T: One> $vec<T> {
			#[doc = stringify!($vec)]
			#[doc = " of all one."]
			pub const ONE: $vec<T> = $vec { $($field: T::ONE),+ };
		}
		impl<T: Zero> Zero for $vec<T> {
			const ZERO: $vec<T> = $vec::ZERO;
		}
		impl<T: One> One for $vec<T> {
			const ONE: $vec<T> = $vec::ONE;
		}
		impl<T: Zero + One> $vec<T> {
			unit!($vec);
		}

		#[doc = stringify!($vec)]
		#[doc = " constructor."]
		#[allow(non_snake_case)]
		#[inline]
		#[must_use]
		pub const fn $vec<T>($($field: T),+) -> $vec<T> {
			$vec { $($field),+ }
		}

		impl<T> $vec<T> {
			with!($vec);
			cvt!($vec);
		}

		impl<T: Copy> $vec<T> {
			/// Gets a component generically.
			#[inline]
			#[must_use]
			pub fn get<C>(self, _: C) -> T where Self: ComponentImpl<T, C> {
				<Self as ComponentImpl<T, C>>::get(self)
			}
			/// Shuffles the components.
			#[inline]
			#[must_use]
			#[allow(unused_variables)]
			pub fn shuffle<$($C),+>(self, $($field: $C),+) -> $vec<T> where Self: $(ComponentImpl<$T, $C> +)+ {
				$vec {
					$($field: <Self as ComponentImpl<$T, $C>>::get(self),)+
				}
			}
		}

		//----------------------------------------------------------------
		// Transformations

		impl<T> $vec<T> {
			/// Casts to a vector of different type with the same dimensions.
			#[inline]
			#[must_use]
			pub fn cast<U>(self) -> $vec<U> where T: CastTo<U> {
				$vec { $($field: self.$field.cast_to()),+ }
			}
			/// Maps a callable over the components.
			#[inline]
			#[must_use]
			pub fn map<U, F>(self, mut f: F) -> $vec<U> where F: FnMut(T) -> U {
				$vec { $($field: f(self.$field)),+ }
			}
			/// Zips two vectors together.
			#[inline]
			#[must_use]
			pub fn zip<U, F>(self, rhs: $vec<T>, mut f: F) -> $vec<U> where F: FnMut(T, T) -> U {
				$vec { $($field: f(self.$field, rhs.$field)),+ }
			}
			/// Reduces the vector.
			#[inline]
			#[must_use]
			pub fn reduce<F>(self, f: F) -> T where F: Fn(T, T) -> T {
				// These will end up nested without temporaries which won't work with `FnMut`...
				fold!(f, $(self.$field),+)
			}
			/// Folds the vector.
			#[inline]
			#[must_use]
			pub fn fold<A, F>(self, acc: A, f: F) -> A where F: Fn(A, T) -> A {
				// These will end up nested without temporaries which won't work with `FnMut`...
				fold!(f, acc, $(self.$field),+)
			}
		}

		//----------------------------------------------------------------
		// Conversions

		impl<T: Scalar> From<T> for $vec<T> {
			#[inline]
			fn from(val: T) -> $vec<T> {
				$vec { $($field: val),+ }
			}
		}

		impl<T> From<($($T,)+)> for $vec<T> {
			#[inline]
			fn from(val: ($($T,)+)) -> $vec<T> {
				$vec { $($field: val.$I),+ }
			}
		}
		impl<T> Into<($($T,)+)> for $vec<T> {
			#[inline]
			fn into(self) -> ($($T,)+) {
				($(self.$field,)+)
			}
		}

		impl<T> From<[T; $N]> for $vec<T> {
			#[inline]
			fn from(val: [T; $N]) -> $vec<T> {
				let [$($field),+] = val;
				$vec { $($field),+ }
			}
		}
		impl<T> Into<[T; $N]> for $vec<T> {
			#[inline]
			fn into(self) -> [T; $N] {
				[$(self.$field),+]
			}
		}

		//----------------------------------------------------------------
		// As references

		impl<T> AsRef<[T; $N]> for $vec<T> {
			#[inline]
			fn as_ref(&self) -> &[T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T]> for $vec<T> {
			#[inline]
			fn as_ref(&self) -> &[T] {
				<Self as AsRef<[T; $N]>>::as_ref(self)
			}
		}
		impl<T> $vec<T> {
			#[inline]
			#[must_use]
			pub fn as_bytes(&self) -> &[u8] {
				unsafe { slice::from_raw_parts(self as *const _ as *const u8, mem::size_of_val(self)) }
			}
		}

		impl<T> AsMut<[T; $N]> for $vec<T> {
			#[inline]
			fn as_mut(&mut self) -> &mut [T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T]> for $vec<T> {
			#[inline]
			fn as_mut(&mut self) -> &mut [T] {
				<Self as AsMut<[T; $N]>>::as_mut(self)
			}
		}

		impl<T> ops::Index<usize> for $vec<T> {
			type Output = T;
			#[inline]
			fn index(&self, i: usize) -> &T {
				let array: &[T; $N] = self.as_ref();
				&array[i]
			}
		}

		//----------------------------------------------------------------
		// Operations

		/// Operations on vectors of scalars.
		impl<T: Scalar> $vec<T> {
			/// Squares the components.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: -3, y: 4 };
			/// assert_eq!(Vec2(9, 16), this.sqr());
			///
			/// let this = Vec3 { x: 2, y: 3, z: -6 };
			/// assert_eq!(Vec3(4, 9, 36), this.sqr());
			/// ```
			#[inline]
			#[must_use]
			pub fn sqr(self) -> $vec<T> {
				$vec { $($field: self.$field * self.$field),+ }
			}
			/// Calculates the squared length of the vector.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: -3, y: 4 };
			/// assert_eq!(25, this.len_sqr());
			///
			/// let this = Vec3 { x: 2, y: -3, z: 6 };
			/// assert_eq!(49, this.len_sqr());
			/// ```
			#[inline]
			#[must_use]
			pub fn len_sqr(self) -> T {
				infix!(+ $(self.$field * self.$field),+)
			}
			/// Calculates the length of the vector.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: -3.0, y: 4.0 };
			/// assert_eq!(5.0, this.len());
			///
			/// let this = Vec3 { x: -2.0, y: 3.0, z: -6.0 };
			/// assert_eq!(7.0, this.len());
			/// ```
			#[inline]
			#[must_use]
			pub fn len(self) -> T where T: Float {
				self.len_sqr().sqrt()
			}
			/// Calculates the manhattan length of the vector.
			///
			/// <!--LEN_HAT--><svg width="400" height="120" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><path fill="none" d="M40 100 L360.5 20 M353.70688 25.818361 L360.5 20 L351.76944 18.056509" stroke="black" /><path fill="none" d="M40 100 L360.5 100 M352.5 104 L360.5 100 L352.5 96" stroke="grey" stroke-width="0.5" /><path fill="none" d="M360.5 100 L360.5 20 M364.5 28 L360.5 20 L356.5 28" stroke="grey" stroke-width="0.5" /><circle cx="40" cy="100" r="2" /><text x="365.5" y="20">this</text><text x="200.25" y="115" fill="grey">x</text><text x="365.5" y="60" fill="grey">y</text></svg>
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: 3, y: 4 };
			/// assert_eq!(7, this.len_hat());
			///
			/// let this = Vec3 { x: 2, y: -3, z: -6 };
			/// assert_eq!(11, this.len_hat());
			/// ```
			#[inline]
			#[must_use]
			pub fn len_hat(self) -> T {
				infix!(+ $(self.$field.abs()),+)
			}
			/// Calculates the squared euclidean distance to another vector.
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let this = Vec2 { x: 1, y: 1 };
			/// let to = Vec2 { x: 2, y: 2 };
			/// assert_eq!(2, this.distance_sqr(to));
			/// ```
			#[inline]
			#[must_use]
			pub fn distance_sqr(self, to: $vec<T>) -> T {
				infix!(+ $((to.$field - self.$field) * (to.$field - self.$field)),+)
			}
			/// Calculates the euclidean distance to another vector.
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let this = Vec2 { x: 10.0, y: 10.0 };
			/// let to = Vec2 { x: 13.0, y: 14.0 };
			/// assert_eq!(5.0, this.distance(to));
			/// ```
			#[inline]
			#[must_use]
			pub fn distance(self, to: $vec<T>) -> T where T: Float {
				self.distance_sqr(to).sqrt()
			}
			/// Calculates the manhattan distance to another vector.
			///
			/// <!--DISTANCE_HAT--><svg width="400" height="120" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><line x1="40" y1="100" x2="360.5" y2="20" stroke="black" /><path fill="none" d="M40 100 L360.5 100 M352.5 104 L360.5 100 L352.5 96" stroke="grey" stroke-width="0.5" /><path fill="none" d="M360.5 100 L360.5 20 M364.5 28 L360.5 20 L356.5 28" stroke="grey" stroke-width="0.5" /><circle cx="40" cy="100" r="2" /><circle cx="360.5" cy="20" r="2" /><text x="20" y="90">this</text><text x="365.5" y="20">to</text><text x="200.25" y="115" fill="grey">x</text><text x="365.5" y="60" fill="grey">y</text></svg>
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: 1.0, y: 5.0 };
			/// let to = Vec2 { x: 5.0, y: 2.0 };
			/// assert_eq!(7.0, this.distance_hat(to));
			///
			/// let this = Vec3 { x: 1.0, y: 5.0, z: -1.0 };
			/// let to = Vec3 { x: 2.0, y: 3.0, z: 1.0 };
			/// assert_eq!(5.0, this.distance_hat(to));
			/// ```
			#[inline]
			#[must_use]
			pub fn distance_hat(self, to: $vec<T>) -> T {
				infix!(+ $((to.$field - self.$field).abs()),+)
			}
			/// Normalizes the vector.
			///
			/// After normalizing the vector has the length `1.0` except the zero vector remains zero.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: 3.0, y: -4.0 };
			/// assert_eq!(Vec2(0.6, -0.8), this.norm());
			///
			/// let this = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
			/// assert_eq!(this, this.norm());
			/// ```
			#[inline]
			#[must_use]
			pub fn norm(self) -> $vec<T> where T: Float {
				self.norm_len().0
			}
			/// Calculates the normalized vector and its length.
			///
			/// After normalizing the vector has the length `1.0` except the zero vector remains zero.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this: Vec2<f32> = Vec2 { x: 3.0, y: -4.0 };
			/// assert_eq!((Vec2(0.6, -0.8), 5.0), this.norm_len());
			///
			/// let this = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
			/// assert_eq!((this, 0.0), this.norm_len());
			/// ```
			#[inline]
			#[must_use]
			pub fn norm_len(self) -> ($vec<T>, T) where T: Float {
				let self_len = self.len();
				if self_len != T::ZERO {
					(self * (T::ONE / self_len), self_len)
				}
				else {
					(self, self_len)
				}
			}
			/// Resizes the vector to the given length.
			///
			/// The zero vector remains zero.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: -3.0, y: -4.0 };
			/// assert_eq!(Vec2(-1.5, -2.0), this.resize(2.5));
			///
			/// let this = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
			/// assert_eq!(Vec3(0.0, 0.0, 0.0), this.resize(2.0));
			/// ```
			#[inline]
			#[must_use]
			pub fn resize(self, len: T) -> $vec<T> where T: Float {
				let self_len = self.len();
				if self_len == T::ZERO {
					return self;
				}
				self * (len / self_len)
			}
			/// Calculates the length of `self` projected onto `v`.
			///
			/// <!--PROJECT_SCALAR--><svg width="400" height="200" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><path fill="none" d="M40 160 L200 20 M196.6134 28.278343 L200 20 L191.34537 22.257729" stroke="black" /><path fill="none" d="M40 160 L360 120 M352.5579 124.96139 L360 120 L351.56564 117.02317" stroke="black" /><circle cx="40" cy="160" r="2" fill="black" /><line x1="214.76923" y1="138.15384" x2="200" y2="20" stroke="black" stroke-dasharray="5.0, 5.0" stroke-width="0.5" /><line x1="194.92368" y1="140.63454" x2="192.44298" y2="120.78898" stroke="black" stroke-width="0.5" /><line x1="192.44298" y1="120.78898" x2="212.28854" y2="118.30828" stroke="black" stroke-width="0.5" /><line x1="41.860523" y1="174.88417" x2="216.62975" y2="153.03801" stroke="red" stroke-width="1.5" /><line x1="41.395393" y1="171.16313" x2="42.325653" y2="178.60521" stroke="red" stroke-width="1.5" /><line x1="216.16461" y1="149.31697" x2="217.09488" y2="156.75905" stroke="red" stroke-width="1.5" /><text x="205" y="25" fill="black">self</text><text x="340" y="142" fill="black">v</text></svg>
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: 1.0, y: 2.0 };
			/// let v = Vec2 { x: 3.0, y: 4.0 };
			/// assert_eq!(2.2, this.project_scalar(v));
			///
			/// let this = Vec3 { x: 1.0, y: 4.0, z: 0.0 };
			/// let v = Vec3 { x: 4.0, y: 2.0, z: 4.0 };
			/// assert_eq!(2.0, this.project_scalar(v));
			/// ```
			#[inline]
			#[must_use]
			pub fn project_scalar(self, v: $vec<T>) -> T where T: Float {
				let len = v.len();
				if len == T::ZERO {
					return len;
				}
				v.dot(self) / len
			}
			/// Projection of `self` onto `v`.
			///
			/// <!--PROJECT-->
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: -5.0, y: -2.5 };
			/// let v = Vec2 { x: 3.0, y: 4.0 };
			/// assert_eq!(Vec2(-3.0, -4.0), this.project(v));
			///
			/// let this = Vec3 { x: -5.0, y: -2.5, z: 0.0 };
			/// let v = Vec3 { x: 3.0, y: 4.0, z: 0.0 };
			/// assert_eq!(Vec3(-3.0, -4.0, 0.0), this.project(v));
			/// ```
			#[inline]
			#[must_use]
			pub fn project(self, v: $vec<T>) -> $vec<T> where T: Float {
				let len_sqr = v.len_sqr();
				if len_sqr == T::ZERO {
					return v;
				}
				v * (v.dot(self) / len_sqr)
			}
			/// Projection of `self` onto `v` clamped to `v`.
			///
			/// <!--PROJECT_SAT-->
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let this = Vec2 { x: -5.0, y: -2.5 };
			/// let v = Vec2 { x: 3.0, y: 4.0 };
			/// assert_eq!(Vec2(0.0, 0.0), this.project_sat(v));
			/// ```
			#[inline]
			#[must_use]
			pub fn project_sat(self, v: $vec<T>) -> $vec<T> {
				let len_sqr = v.len_sqr();
				if len_sqr > T::ZERO {
					v * (v.dot(self) / len_sqr).clamp(T::ZERO, T::ONE)
				}
				else { v }
			}
			/// Reflects `self` around `v`.
			///
			/// <!--REFLECT_2D--><svg width="400" height="200" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><line x1="140" y1="20" x2="175.29413" y2="161.17647" stroke="black" stroke-width="0.5" stroke-dasharray="5.0, 5.0" /><line x1="157.64706" y1="90.588234" x2="57.647064" y2="190.58823" stroke="black" stroke-width="0.5" stroke-dasharray="5.0, 5.0" /><line x1="57.647064" y1="190.58823" x2="175.29413" y2="161.17647" stroke="black" stroke-width="0.5" stroke-dasharray="5.0, 5.0" /><path fill="none" d="M40 120 L290 57.5 M283.209 63.320854 L290 57.5 L281.2687 55.559715" stroke="black" /><path fill="none" d="M40 120 L140 20 M137.17157 28.485283 L140 20 L131.51471 22.828426" stroke="black" /><path fill="none" d="M40 120 L175.29413 161.17647 M166.47609 162.67386 L175.29413 161.17647 L168.80537 155.02048" stroke="red" /><circle cx="157.64706" cy="90.588234" r="2" fill="black" /><text x="290" y="57.5" fill="black">v</text><text x="140" y="20" fill="black">self</text><text x="165.64706" y="100.588234" fill="black">p</text><text x="175.29413" y="161.17647" fill="red">result</text><text x="52.647064" y="175.58823" fill="black">-self</text><text x="151.76471" y="182.05882" fill="black">+p</text></svg>
			///
			/// <!--REFLECT_3D-->
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let this = Vec2 { x: 1.0, y: 3.0 };
			/// let v = Vec2 { x: 4.0, y: 4.0 };
			/// assert_eq!(Vec2(3.0, 1.0), this.reflect(v));
			/// ```
			#[inline]
			#[must_use]
			pub fn reflect(self, v: $vec<T>) -> $vec<T> where T: Float {
				let p = self.project(v);
				p + p - self
			}
			$($ops)*
			/// Calculates the dot product.
			///
			/// <!--DOT-->
			///
			/// ```
			/// use cvmath::Vec3;
			///
			/// let lhs = Vec3 { x: 1, y: 2, z: 3 };
			/// let rhs = Vec3 { x: 4, y: -5, z: 6 };
			/// assert_eq!(12, Vec3::dot(lhs, rhs));
			/// ```
			#[inline]
			#[must_use]
			pub fn dot(self, rhs: $vec<T>) -> T {
				infix!(+ $(self.$field * rhs.$field),+)
			}
			/// Calculates the cosine of the angle between two vectors.
			///
			/// <!--COS_ANGLE-->
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let lhs = Vec2 { x: 1.0, y: 1.0 };
			/// let rhs = Vec2 { x: 1.0, y: 0.0 };
			/// let sqrt_2_div_2 = 1.0 / 2_f32.sqrt(); // √2 ÷ 2
			/// assert_eq!(sqrt_2_div_2, lhs.cos_angle(rhs));
			/// ```
			#[inline]
			#[must_use]
			pub fn cos_angle(self, rhs: $vec<T>) -> T where T: Float {
				// |self| * |rhs| <=> √(self ∙ self * rhs ∙ rhs)
				let d = (self.dot(self) * rhs.dot(rhs)).sqrt();
				self.dot(rhs) / d
			}
			/// Calculates the angle between two vectors.
			///
			/// <!--ANGLE-->
			///
			/// ```
			/// use cvmath::Vec2;
			/// # macro_rules! assert_eq { ($a:expr, $b:expr) => { assert!((f64::abs($a - $b) < 0.00001), "Expected {}, got {}", $b, $a); } }
			///
			/// let lhs = Vec2 { x: 1.0, y: 1.0 };
			/// let rhs = Vec2 { x: 1.0, y: 0.0 };
			/// assert_eq!(45.0, lhs.angle(rhs).to_deg());
			/// ```
			#[inline]
			#[must_use]
			pub fn angle(self, rhs: $vec<T>) -> Angle<T> where T: Float {
				Angle::acos(self.cos_angle(rhs))
			}
			/// Horizontal adds all components.
			///
			/// ```
			/// use cvmath::{Vec2, Vec3};
			///
			/// let this = Vec2 { x: -2, y: 7 };
			/// assert_eq!(5, this.hadd());
			///
			/// let this = Vec3 { x: 3, y: 4, z: 5 };
			/// assert_eq!(12, this.hadd());
			/// ```
			#[inline]
			#[must_use]
			pub fn hadd(self) -> T {
				infix!(+ $(self.$field),+)
			}
			/// Component-wise absolute value.
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let this = Vec2 { x: -3, y: 5 };
			/// assert_eq!(Vec2(3, 5), this.abs());
			/// ```
			#[inline]
			#[must_use]
			pub fn abs(self) -> $vec<T> {
				$vec { $($field: self.$field.abs()),+ }
			}
			/// Component-wise minimum value.
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let lhs = Vec2 { x: -3, y: 5 };
			/// let rhs = Vec2 { x: 0, y: 2 };
			/// assert_eq!(Vec2(-3, 2), lhs.min(rhs));
			/// ```
			#[inline]
			#[must_use]
			pub fn min(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: T::min(self.$field, rhs.$field)),+ }
			}
			/// Horizontal minimum value.
			#[inline]
			#[must_use]
			pub fn vmin(self) -> T {
				self.reduce(T::min)
			}
			/// Component-wise maximum value.
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// let lhs = Vec2 { x: -3, y: 5 };
			/// let rhs = Vec2 { x: 0, y: 2 };
			/// assert_eq!(Vec2(0, 5), lhs.max(rhs));
			/// ```
			#[inline]
			#[must_use]
			pub fn max(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: T::max(self.$field, rhs.$field)),+ }
			}
			/// Horizontal maximum value.
			#[inline]
			#[must_use]
			pub fn vmax(self) -> T {
				self.reduce(T::max)
			}
			/// Adds the scaled vector.
			///
			/// Equivalent to `self + (vec * scale)` with less rounding errors.
			#[inline]
			#[must_use]
			pub fn mul_add(self, vec: $vec<T>, scale: T) -> $vec<T> {
				$vec { $($field: T::mul_add(vec.$field, scale, self.$field)),+ }
			}
			/// Linear interpolation between the vectors.
			///
			/// <!--LERP--><svg width="400" height="120" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><line x1="40" y1="100" x2="104" y2="84" stroke="green" /><line x1="104" y1="84" x2="200" y2="60" stroke="blue" /><line x1="200" y1="60" x2="360" y2="20" stroke="black" /><circle cx="40" cy="100" r="2" fill="black" /><circle cx="360" cy="20" r="2" fill="black" /><circle cx="104" cy="84" r="2" fill="green" /><circle cx="200" cy="60" r="2" fill="blue" /><text x="20" y="90" fill="black">self</text><text x="345" y="40" fill="black">rhs</text><text x="84" y="104" fill="green">t = 0.2</text><text x="180" y="80" fill="blue">t = 0.5</text></svg>
			#[inline]
			#[must_use]
			pub fn lerp(self, rhs: $vec<T>, t: T) -> $vec<T> {
				self + (rhs - self) * t
			}
			/// Spherical interpolation between the vectors with constant velocity.
			///
			/// The result is linear interpolation of the angles between the vectors and their lengths.
			///
			/// This is fairly expensive to calculate requiring trigonometric functions.
			/// If constant velocity isn't required, see the less expensive [nlerp](#method.nlerp).
			///
			/// <!--SLERP--><svg width="400" height="140" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><path fill="none" d="M200 136.33249 L100 70 M108.87775 71.08883 L100 70 L104.45558 77.7555" stroke="black" stroke-width="0.5" /><path fill="none" d="M200 136.33249 L300 70 M295.54443 77.7555 L300 70 L291.12225 71.08883" stroke="black" stroke-width="0.5" /><path fill="none" d="M200 136.33249 L143.25452 30.597214 M150.56206 35.754715 L143.25452 30.597214 L143.51305 39.53775" stroke="green" stroke-width="0.25" /><path fill="none" d="M200 136.33249 L200 16.332481 M204 24.332481 L200 16.332481 L196 24.332481" stroke="green" stroke-width="0.25" /><path fill="none" d="M200 136.33249 L256.74548 30.597221 M256.48697 39.537758 L256.74548 30.597221 L249.43794 35.754723" stroke="green" /><path fill="none" d="M88.950035 90.85828 A120 120 0 0 1 100 70" stroke="black" stroke-width="0.5" /><path fill="none" d="M100 70 A120 120 0 0 1 256.74548 30.597221" stroke="green" /><path fill="none" d="M256.74548 30.597221 A120 120 0 0 1 300 70" stroke="black" /><path fill="none" d="M300 70 A120 120 0 0 1 311.05 90.85829" stroke="black" stroke-width="0.5" /><line x1="100" y1="70" x2="250" y2="70" stroke="blue" stroke-width="0.5" /><circle cx="100" cy="70" r="2" fill="black" /><circle cx="300" cy="70" r="2" fill="black" /><circle cx="250" cy="70" r="2" fill="blue" /><circle cx="256.74548" cy="30.597221" r="2" fill="green" /><text x="98.25452" y="25.597214" fill="green" font-size="10">t = 0.25</text><text x="180" y="11.332481" fill="green" font-size="10">t = 0.50</text><text x="256.74548" y="25.597221" fill="green" font-size="10">t = 0.75</text><text x="230" y="90" fill="blue">lerp</text><text x="196.74548" y="40.59722" fill="green">slerp</text><text x="50" y="70" fill="black">self</text><text x="310" y="70" fill="black">rhs</text></svg>
			#[inline]
			#[must_use]
			pub fn slerp(self, rhs: $vec<T>, t: T) -> $vec<T> where T: Float {
				let (v0, len0) = self.norm_len();
				let (v1, len1) = rhs.norm_len();
				let len = len0 + (len1 - len0) * t;

				let dot = v0.dot(v1);
				let theta = Angle::acos(dot) * t;
				let (sin, cos) = theta.sin_cos();

				let v2 = (v1 - v0 * dot).norm();
				(v0 * cos + v2 * sin) * len
			}
			/// Cheap spherical interpolation between the vectors without constant velocity.
			///
			/// <!--NLERP--><svg width="400" height="140" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><path fill="none" d="M200 136.33249 L100 70 M108.87775 71.08883 L100 70 L104.45558 77.7555" stroke="black" stroke-width="0.5" /><path fill="none" d="M200 136.33249 L300 70 M295.54443 77.7555 L300 70 L291.12225 71.08883" stroke="black" stroke-width="0.5" /><path fill="none" d="M200 136.33249 L127.768486 40.50657 M135.77812 44.487244 L127.768486 40.50657 L129.38972 49.30268" stroke="green" stroke-width="0.25" /><path fill="none" d="M200 136.33249 L200 16.332497 M204 24.332497 L200 16.332497 L196 24.332497" stroke="green" stroke-width="0.25" /><path fill="none" d="M200 136.33249 L272.2315 40.50657 M270.61026 49.30268 L272.2315 40.50657 L264.2219 44.487244" stroke="green" /><path fill="none" d="M94.97722 78.27897 A120 120 0 0 1 100 70" stroke="black" stroke-width="0.5" /><path fill="none" d="M100 70 A120 120 0 0 1 272.2315 40.50657" stroke="green" /><path fill="none" d="M272.2315 40.50657 A120 120 0 0 1 300 70" stroke="black" /><path fill="none" d="M300 70 A120 120 0 0 1 305.02277 78.27897" stroke="black" stroke-width="0.5" /><line x1="100" y1="70" x2="250" y2="70" stroke="blue" stroke-width="0.5" /><circle cx="100" cy="70" r="2" fill="black" /><circle cx="300" cy="70" r="2" fill="black" /><circle cx="250" cy="70" r="2" fill="blue" /><circle cx="272.2315" cy="40.50657" r="2" fill="green" /><text x="82.768486" y="35.50657" fill="green" font-size="10">t = 0.25</text><text x="180" y="11.332497" fill="green" font-size="10">t = 0.50</text><text x="272.2315" y="35.50657" fill="green" font-size="10">t = 0.75</text><text x="230" y="90" fill="blue">lerp</text><text x="212.2315" y="50.50657" fill="green">nlerp</text><text x="50" y="70" fill="black">self</text><text x="310" y="70" fill="black">rhs</text></svg>
			#[inline]
			#[must_use]
			pub fn nlerp(self, rhs: $vec<T>, t: T) -> $vec<T> where T: Float {
				let self_len = self.len();
				let rhs_len = rhs.len();
				let len = self_len + (rhs_len - self_len) * t;
				self.lerp(rhs, t).resize(len)
			}
			/// Exponential decay smoothing.
			///
			/// Also known as lerp smoothing.
			/// Useful decay values range from approx 1.0 to 25.0, slow to fast.
			///
			/// ```
			/// use cvmath::Vec2;
			///
			/// struct Entity {
			/// 	pos: Vec2<f32>,
			/// 	target: Vec2<f32>,
			/// }
			/// impl Entity {
			/// 	fn update(&mut self, dt: f32) {
			/// 		// Smoothly move towards the target.
			/// 		self.pos = self.pos.exp_decay(self.target, 5.0, dt);
			/// 	}
			/// }
			/// ```
			#[inline]
			#[must_use]
			pub fn exp_decay(self, rhs: $vec<T>, decay: T, dt: T) -> $vec<T> where T: Float {
				rhs + (self - rhs) * (-decay * dt).exp()
			}
		}

		// Float ops
		impl<T: Float> $vec<T> {
			/// Component-wise floor.
			#[inline]
			#[must_use]
			pub fn floor(self) -> $vec<T> {
				$vec { $($field: self.$field.floor()),+ }
			}
			/// Component-wise ceil.
			#[inline]
			#[must_use]
			pub fn ceil(self) -> $vec<T> {
				$vec { $($field: self.$field.ceil()),+ }
			}
			/// Component-wise round.
			#[inline]
			#[must_use]
			pub fn round(self) -> $vec<T> {
				$vec { $($field: self.$field.round()),+ }
			}
			/// Component-wise fract.
			#[inline]
			#[must_use]
			pub fn fract(self) -> $vec<T> {
				$vec { $($field: self.$field.fract()),+ }
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T: Extrema> Extrema<$vec<T>> for $vec<T> {
			#[inline]
			fn min(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: T::min(self.$field, rhs.$field)),+ }
			}
			#[inline]
			fn max(self, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: T::max(self.$field, rhs.$field)),+ }
			}
			#[inline]
			fn min_max(self, rhs: $vec<T>) -> ($vec<T>, $vec<T>) {
				let temp = $vec { $($field: self.$field.min_max(rhs.$field)),+ };
				($vec { $($field: temp.$field.0),+ }, $vec { $($field: temp.$field.1),+ })
			}
		}
		impl<T: PartialOrd> SpatialOrd<$vec<T>> for $vec<T> {
			#[inline] fn spatial_lt(&self, rhs: &$vec<T>) -> bool { $(self.$field < rhs.$field &&)+ true }
			#[inline] fn spatial_le(&self, rhs: &$vec<T>) -> bool { $(self.$field <= rhs.$field &&)+ true }
			#[inline] fn spatial_gt(&self, rhs: &$vec<T>) -> bool { $(self.$field > rhs.$field &&)+ true }
			#[inline] fn spatial_ge(&self, rhs: &$vec<T>) -> bool { $(self.$field >= rhs.$field &&)+ true }
		}

		// Vector addition, subtraction and negation
		impl<U, T: ops::Add<U>> ops::Add<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn add(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field + rhs.$field),+ }
			}
		}
		impl<U, T: ops::Sub<U>> ops::Sub<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn sub(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field - rhs.$field),+ }
			}
		}
		impl<T: ops::Neg> ops::Neg for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn neg(self) -> $vec<T::Output> {
				$vec { $($field: -self.$field),+ }
			}
		}
		impl<U, T: ops::AddAssign<U>> ops::AddAssign<$vec<U>> for $vec<T> {
			#[inline]
			fn add_assign(&mut self, rhs: $vec<U>) {
				$(self.$field += rhs.$field;)+
			}
		}
		impl<U, T: ops::SubAssign<U>> ops::SubAssign<$vec<U>> for $vec<T> {
			#[inline]
			fn sub_assign(&mut self, rhs: $vec<U>) {
				$(self.$field -= rhs.$field;)+
			}
		}

		// Scalar multiplication, division and remainder
		impl<U: Scalar, T: ops::Mul<U>> ops::Mul<U> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn mul(self, rhs: U) -> $vec<T::Output> {
				$vec { $($field: self.$field * rhs),+ }
			}
		}
		impl<U: Scalar, T: ops::Div<U>> ops::Div<U> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn div(self, rhs: U) -> $vec<T::Output> {
				$vec { $($field: self.$field / rhs),+ }
			}
		}
		impl<U: Scalar, T: ops::Rem<U>> ops::Rem<U> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn rem(self, rhs: U) -> $vec<T::Output> {
				$vec { $($field: self.$field % rhs),+ }
			}
		}
		impl<U: Scalar, T: ops::MulAssign<U>> ops::MulAssign<U> for $vec<T> {
			#[inline]
			fn mul_assign(&mut self, rhs: U) {
				$(self.$field *= rhs;)+
			}
		}
		impl<U: Scalar, T: ops::DivAssign<U>> ops::DivAssign<U> for $vec<T> {
			#[inline]
			fn div_assign(&mut self, rhs: U) {
				$(self.$field /= rhs;)+
			}
		}
		impl<U: Scalar, T: ops::RemAssign<U>> ops::RemAssign<U> for $vec<T> {
			#[inline]
			fn rem_assign(&mut self, rhs: U) {
				$(self.$field %= rhs;)+
			}
		}

		// Vector multiplication, division and remainder
		impl<U, T: ops::Mul<U>> ops::Mul<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn mul(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field * rhs.$field),+ }
			}
		}
		impl<U, T: ops::Div<U>> ops::Div<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn div(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field / rhs.$field),+ }
			}
		}
		impl<U, T: ops::Rem<U>> ops::Rem<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn rem(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field % rhs.$field),+ }
			}
		}
		impl<U, T: ops::MulAssign<U>> ops::MulAssign<$vec<U>> for $vec<T> {
			#[inline]
			fn mul_assign(&mut self, rhs: $vec<U>) {
				$(self.$field *= rhs.$field;)+
			}
		}
		impl<U, T: ops::DivAssign<U>> ops::DivAssign<$vec<U>> for $vec<T> {
			#[inline]
			fn div_assign(&mut self, rhs: $vec<U>) {
				$(self.$field /= rhs.$field;)+
			}
		}
		impl<U, T: ops::RemAssign<U>> ops::RemAssign<$vec<U>> for $vec<T> {
			#[inline]
			fn rem_assign(&mut self, rhs: $vec<U>) {
				$(self.$field %= rhs.$field;)+
			}
		}

		//----------------------------------------------------------------
		// Formatting

		fmt!($vec { $($field),+ });

		//----------------------------------------------------------------
		// Parsing

		impl<T: FromStr> FromStr for $vec<T> {
			type Err = ParseVecError<T::Err>;
			fn from_str(s: &str) -> Result<$vec<T>, Self::Err> {
				let bytes = s.as_bytes();
				// Must be surrounded by parenthesis
				if bytes.len() < 2 || bytes[0] != b'(' || bytes[bytes.len() - 1] != b')' {
					return Err(ParseVecError::SyntaxError);
				}
				// Comma separated list of values
				let mut iter = s.bytes().enumerate().filter_map(|(i, v)| if v == b',' { Some(i) } else { None });
				let mut next = 1;
				$(let $field;)+
				parse_vec_elems!(s, iter, next; $($field),+);
				Ok($vec { $($field),+ })
			}
		}
	}
}

// vec!(Vec1 1 { x 0 T U X });
vec!(Vec2 2 { x 0 T U X, y 1 T U Y } {
	/// Calculates the polar angle.
	///
	/// <!--POLAR_ANGLE-->
	///
	/// ```
	/// use cvmath::Vec2;
	/// # macro_rules! assert_eq { ($a:expr, $b:expr) => { assert!((f64::abs($a - $b) < 0.00001), "Expected {}, got {}", $b, $a); } }
	///
	/// let this = Vec2 { x: 1.0, y: 1.0 };
	/// assert_eq!(45.0, this.polar_angle().to_deg());
	/// ```
	#[inline]
	pub fn polar_angle(self) -> Angle<T> where T: Float {
		Angle::atan2(self.y, self.x)
	}
	/// Returns the signed angle between two 2D vectors.
	///
	/// The sign of the angle follows the right-hand rule (counter-clockwise is positive).
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let a = Vec2::X;
	/// let b = Vec2::Y;
	/// assert_eq!(90.0, a.signed_angle(b).to_deg());
	/// assert_eq!(-90.0, b.signed_angle(a).to_deg());
	/// ```
	#[inline]
	#[must_use]
	pub fn signed_angle(self, rhs: Vec2<T>) -> Angle<T> where T: Float {
		Angle::atan2(self.cross(rhs), self.dot(rhs))
	}
	/// Rotates the vector counter-clockwise by 90°.
	///
	/// The resulting vector is perpendicular to the given vector.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let this = Vec2 { x: 3.0, y: 4.0 };
	/// assert_eq!(Vec2(4.0, -3.0), this.ccw());
	/// ```
	#[inline]
	pub fn ccw(self) -> Vec2<T> {
		Vec2 { x: self.y, y: -self.x }
	}
	/// Rotates the vector clockwise by 90°.
	///
	/// The resulting vector is perpendicular to the given vector.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let this = Vec2 { x: 3.0, y: 4.0 };
	/// assert_eq!(Vec2(-4.0, 3.0), this.cw());
	/// ```
	#[inline]
	pub fn cw(self) -> Vec2<T> {
		Vec2 { x: -self.y, y: self.x }
	}
	/// Calculates the magnitude of the 3D cross product where the inputs are extended with `z = 0`.
	///
	/// This result is equal to the area of the parallelogram between the two vectors.
	/// This result is equal to twice the area of the triangle between the two vectors.
	///
	/// Furthermore this area is signed; a positive value means the `rhs` is on the left side of `self` and a negative value means `rhs` is on the right side.
	///
	/// <!--CROSS_2D-->
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let lhs = Vec2 { x: -3, y: -4 };
	/// let rhs = Vec2 { x: -1, y: 2 };
	/// assert_eq!(-10, lhs.cross(rhs));
	/// // Area under the parallelogram defined by (origin, lhs, rhs, lhs + rhs) equals 10
	/// // Area under the triangle defined by (origin, lhs, rhs) equals 5
	/// ```
	#[inline]
	pub fn cross(self, rhs: Vec2<T>) -> T {
		self.x * rhs.y - self.y * rhs.x
	}
	/// Horizontal subtracts the components.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let this = Vec2 { x: 3, y: 4 };
	/// assert_eq!(-1, this.hsub());
	/// ```
	#[inline]
	pub fn hsub(self) -> T {
		self.x - self.y
	}
	/// Intercepts the vector with `x = constant` returning the y.
	#[inline]
	pub fn y_intercept(self, x: T) -> Option<T> {
		if self.x != T::ZERO {
			Some((self.y * x) / self.x)
		}
		else {
			None
		}
	}
	/// Intercepts the vector with `y = constant` returning the x.
	#[inline]
	pub fn x_intercept(self, y: T) -> Option<T> {
		if self.y != T::ZERO {
			Some((y * self.x) / self.y)
		}
		else {
			None
		}
	}
});
vec!(Vec3 3 { x 0 T U X, y 1 T U Y, z 2 T U Z } {
	/// Calculates the 3D cross product.
	///
	/// Effectively calculates the vector perpendicular to both inputs with direction according to the [right-hand rule](https://en.wikipedia.org/wiki/Right-hand_rule).
	///
	/// <!--CROSS_3D-->
	///
	/// ```
	/// use cvmath::Vec3;
	///
	/// let lhs = Vec3 { x: 3, y: -3, z: 1 };
	/// let rhs = Vec3 { x: 4, y: 9, z: 1 };
	/// assert_eq!(Vec3(-12, 1, 39), lhs.cross(rhs));
	/// ```
	#[inline]
	pub fn cross(self, rhs: Vec3<T>) -> Vec3<T> {
		Vec3 {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x,
		}
	}
	/// Returns the signed angle between two 3D vectors around a given axis.
	///
	/// The sign of the angle is determined by the right-hand rule around the `axis`.
	///
	/// ```
	/// use cvmath::Vec3;
	///
	/// let a = Vec3::X;
	/// let b = Vec3::Y;
	/// let axis = Vec3::Z;
	///
	/// assert_eq!(90.0, a.signed_angle(b, axis).to_deg());
	/// ```
	#[inline]
	#[must_use]
	pub fn signed_angle(self, rhs: Vec3<T>, axis: Vec3<T>) -> Angle<T> where T: Float {
		Angle::atan2(axis.dot(self.cross(rhs)), self.dot(rhs))
	}
	/// Returns a normalized vector perpendicular to `self`.
	///
	/// ```
	/// use cvmath::Vec3;
	///
	/// let v: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);
	/// let perp = v.any_perp();
	///
	/// // Dot product is (close to) zero, meaning perpendicular:
	/// assert_eq!(v.dot(perp).abs(), 0.0);
	/// assert_eq!(perp.len(), 1.0);
	/// ```
	#[inline]
	#[must_use]
	pub fn any_perp(self) -> Vec3<T> where T: Float {
		let v = if self.x != T::ZERO || self.y != T::ZERO { Vec3::new(-self.y, self.x, T::ZERO) }
		else { Vec3::new(T::ZERO, -self.z, self.y) };
		v.norm()
	}
	/// Homogeneous divide.
	#[inline]
	pub fn hdiv(self) -> Vec2<T> {
		if self.z != T::ZERO {
			Vec2 {
				x: self.x / self.z,
				y: self.x / self.z,
			}
		}
		else { self.xy() }
	}
});
vec!(Vec4 4 { x 0 T U X, y 1 T U Y, z 2 T U Z, w 3 T U W } {
	/// Homogeneous divide.
	#[inline]
	pub fn hdiv(self) -> Vec3<T> {
		if self.w != T::ZERO {
			let inv_w = T::ONE / self.w;
			Vec3 {
				x: self.x * inv_w,
				y: self.y * inv_w,
				z: self.z * inv_w,
			}
		}
		else { self.xyz() }
	}
});

specialized_type!(Vec2, Vec2f, f32, x, y);
specialized_type!(Vec3, Vec3f, f32, x, y, z);
specialized_type!(Vec4, Vec4f, f32, x, y, z, w);

specialized_type!(Vec2, Vec2d, f64, x, y);
specialized_type!(Vec3, Vec3d, f64, x, y, z);
specialized_type!(Vec4, Vec4d, f64, x, y, z, w);

specialized_type!(Vec2, Vec2i, i32, x, y);
specialized_type!(Vec3, Vec3i, i32, x, y, z);
specialized_type!(Vec4, Vec4i, i32, x, y, z, w);

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Vec2<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let slice = <Vec2<T> as AsRef<[T; 2]>>::as_ref(self).as_slice();
		serializer.collect_seq(slice)
	}
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Vec3<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let slice = <Vec3<T> as AsRef<[T; 3]>>::as_ref(self).as_slice();
		serializer.collect_seq(slice)
	}
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Vec4<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let slice = <Vec4<T> as AsRef<[T; 4]>>::as_ref(self).as_slice();
		serializer.collect_seq(slice)
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Vec2<T> {
	fn deserialize<D>(deserializer: D) -> Result<Vec2<T>, D::Error> where D: serde::Deserializer<'de> {
		let [x, y] = <[T; 2]>::deserialize(deserializer)?;
		Ok(Vec2 { x, y })
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Vec3<T> {
	fn deserialize<D>(deserializer: D) -> Result<Vec3<T>, D::Error> where D: serde::Deserializer<'de> {
		let [x, y, z] = <[T; 3]>::deserialize(deserializer)?;
		Ok(Vec3 { x, y, z })
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Vec4<T> {
	fn deserialize<D>(deserializer: D) -> Result<Vec4<T>, D::Error> where D: serde::Deserializer<'de> {
		let [x, y, z, w] = <[T; 4]>::deserialize(deserializer)?;
		Ok(Vec4 { x, y, z, w })
	}
}

//----------------------------------------------------------------

/// An error which can be returned when parsing a VecN<T>.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseVecError<E> {
	/// Missing parentheses surrounding the vector elements.
	SyntaxError,
	/// The number of vector elements doesn't match the vector dimensions.
	DimMismatch,
	/// Error parsing the vector elements.
	ParseValue(E),
}
impl<E> From<E> for ParseVecError<E> {
	#[inline]
	fn from(err: E) -> ParseVecError<E> {
		ParseVecError::ParseValue(err)
	}
}
impl<E: Error + 'static> fmt::Display for ParseVecError<E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#[allow(deprecated)]
		self.description().fmt(f)
	}
}
impl<E: Error + 'static> Error for ParseVecError<E> {
	fn description(&self) -> &str {
		#[allow(deprecated)]
		match *self {
			ParseVecError::SyntaxError => "syntax error",
			ParseVecError::DimMismatch => "dim mismatch",
			ParseVecError::ParseValue(ref inner) => inner.description(),
		}
	}
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			ParseVecError::SyntaxError => None,
			ParseVecError::DimMismatch => None,
			ParseVecError::ParseValue(ref inner) => Some(inner),
		}
	}
}
