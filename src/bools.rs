/*!
Boolean vectors.

## Comparison masks

Comparison masks are boolean vectors to be consumed by `select`.

`is_finite(self)`: Creates a mask for finite components.

`is_infinite(self)`: Creates a mask for infinite components.

`eq(self, rhs)`: Creates a mask for equal components.

`ne(self, rhs)`: Creates a mask for unequal components.

`lt(self, rhs)`: Creates a mask for left-hand side components are less than the right-hand side.

`le(self, rhs)`: Creates a mask for left-hand side components are less than or equal the right-hand side.

`gt(self, rhs)`: Creates a mask for left-hand side components are greater than the right-hand side.

`ge(self, rhs)`: Creates a mask for left-hand side components are greater than or equal the right-hand side.

`is_close(self, rhs)`: Creates a mask for approximately equal components.

`all_close(self, rhs)`: Equivalent to `self.is_close(rhs).all()`.

### Examples

```
use cvmath::{Vec2, Bool2};

assert_eq!(Bool2 { x: true, y: false }, Vec2(1, 2).eq(Vec2(1, -2)));
```

## Comparison operators

`any(self)`: Returns `true` if any of the components are `true`.

`all(self)`: Returns `true` if all the components are `true`.

`none(self)`: Returns `true` if none of the components are `true`.

`select(self, lhs, rhs)`: Combines two vectors based on the bools, selecting components from the left-hand side if `true` and right-hand side if `false`.

### Examples

```
use cvmath::{Bool2};

assert!(Bool2 { x: true, y: false }.any());
assert!(Bool2 { x: true, y: true }.all());
assert!(Bool2 { x: false, y: false }.none());
```

*/

use super::*;

/// Bool2 mask.
pub type Bool2 = Vec2<bool>;
/// Bool3 mask.
pub type Bool3 = Vec3<bool>;
/// Bool4 mask.
pub type Bool4 = Vec4<bool>;

macro_rules! bools {
	($bools:ident $vec:ident { $($field:ident),+ }) => {

		#[doc = stringify!($bools)]
		#[doc = " constructor."]
		#[allow(non_snake_case)]
		#[inline]
		pub const fn $bools($($field: bool),+) -> $bools {
			$bools { $($field),+ }
		}

		//----------------------------------------------------------------
		// Comparison masks

		impl<T> $vec<T> {
			/// Creates a mask for finite components.
			#[inline]
			pub fn is_finite(self) -> $bools where T: Float {
				$vec { $($field: self.$field.is_finite()),+ }
			}
			/// Creates a mask for infinite components.
			#[inline]
			pub fn is_infinite(self) -> $bools where T: Float {
				$vec { $($field: self.$field.is_infinite()),+ }
			}
			/// Creates a mask for equal components.
			#[inline]
			pub fn eq(self, rhs: $vec<T>) -> $bools where T: PartialEq {
				$vec { $($field: self.$field == rhs.$field),+ }
			}
			/// Creates a mask for inequal components.
			#[inline]
			pub fn ne(self, rhs: $vec<T>) -> $bools where T: PartialEq {
				$vec { $($field: self.$field != rhs.$field),+ }
			}
			/// Creates a mask for left-hand side components are less than the right-hand side.
			#[inline]
			pub fn lt(self, rhs: $vec<T>) -> $bools where T: PartialOrd {
				$vec { $($field: self.$field < rhs.$field),+ }
			}
			/// Creates a mask for left-hand side components are less than or equal the right-hand side.
			#[inline]
			pub fn le(self, rhs: $vec<T>) -> $bools where T: PartialOrd {
				$vec { $($field: self.$field <= rhs.$field),+ }
			}
			/// Creates a mask for left-hand side components are greater than the right-hand side.
			#[inline]
			pub fn gt(self, rhs: $vec<T>) -> $bools where T: PartialOrd {
				$vec { $($field: self.$field > rhs.$field),+ }
			}
			/// Creates a mask for left-hand side components are greater than or equal the right-hand side.
			#[inline]
			pub fn ge(self, rhs: $vec<T>) -> $bools where T: PartialOrd {
				$vec { $($field: self.$field >= rhs.$field),+ }
			}
		}

		impl<T> $vec<T> {
			/// Creates a mask for approximately equal components.
			#[inline]
			pub fn is_close(self, rhs: $vec<T>) -> $bools where T: Float {
				$vec { $($field: self.$field.is_close(rhs.$field)),+ }
			}
			/// Returns true if the values are approximately equal.
			#[inline]
			pub fn all_close(self, rhs: $vec<T>) -> bool where T: Float {
				self.is_close(rhs).all()
			}
		}

		//----------------------------------------------------------------
		// Comparison operators

		impl $bools {
			/// Returns `true` if any of the components are `true`.
			#[inline]
			pub const fn any(self) -> bool {
				infix!(| $(self.$field),+)
			}
			/// Returns `true` if all the components are `true`.
			#[inline]
			pub const fn all(self) -> bool {
				infix!(& $(self.$field),+)
			}
			/// Returns `true` if none of the components are `true`.
			#[inline]
			pub const fn none(self) -> bool {
				!self.any()
			}
			/// Combines two vectors based on the bools, selecting components from the left-hand side if `true` and right-hand side if `false`.
			#[inline]
			pub fn select<T>(self, lhs: $vec<T>, rhs: $vec<T>) -> $vec<T> {
				$vec { $($field: if self.$field { lhs.$field } else { rhs.$field }),+ }
			}
		}

		//----------------------------------------------------------------
		// Bitwise operators

		impl<U, T: ops::BitAnd<U>> ops::BitAnd<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn bitand(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field & rhs.$field),+ }
			}
		}
		impl<U, T: ops::BitOr<U>> ops::BitOr<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn bitor(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field | rhs.$field),+ }
			}
		}
		impl<U, T: ops::BitXor<U>> ops::BitXor<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn bitxor(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field ^ rhs.$field),+ }
			}
		}
		impl<T: ops::Not> ops::Not for $vec<T> {
			type Output = $vec<T::Output>;
			#[inline]
			fn not(self) -> $vec<T::Output> {
				$vec { $($field: !self.$field),+ }
			}
		}
	};
}

bools!(Bool2 Vec2 { x, y });
bools!(Bool3 Vec3 { x, y, z });
bools!(Bool4 Vec4 { x, y, z, w });
