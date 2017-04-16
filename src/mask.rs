/*!
Boolean vectors.

## Comparison masks

Comparison masks are boolean vectors to be consumed by `select`.

Map or Zip vectors to a boolean vector for a custom mask.

`is_finite(self)`: Masks if the components are finite.

`is_infinite(self)`: Masks if the components are infinite.

`eq(self, rhs)`: Masks if the components are equal.

`ne(self, rhs)`: Masks if the components are not equal.

`lt(self, rhs)`: Masks if the left-hand side components are less than the right-hand side.

`le(self, rhs)`: Masks if the left-hand side components are less than or equal the right-hand side.

`gt(self, rhs)`: Masks if the left-hand side components are greater than the right-hand side.

`ge(self, rhs)`: Masks if the left-hand side components are greater than or equal the right-hand side.

`select(self, rhs, mask)`: Combines two vectors based on the mask, selecting components from the left-hand side if `true` and right-hand side if `false`.

### Examples

```
# use cgm::prelude::{Vec2, Mask2};
assert_eq!(Mask2 { x: true, y: false }, Vec2::new(1, 2).eq(Vec2::new(1, -2)));
```

## Comparison operators

`any(self)`: Returns `true` if any of the components is `true`.

`all(self)`: Returns `true` if all the components are `true`.

`none(self)`: Returns `true` if none of the components are `true`.

### Examples

```
# use cgm::prelude::{Mask2};
assert!(Mask2 { x: true, y: false }.any());
assert!(Mask2 { x: true, y: true }.all());
assert!(Mask2 { x: false, y: false }.none());
```

*/

use ::vec::{Vec2, Vec3, Vec4};
use ::num::{Float};

pub type Mask2 = Vec2<bool>;
pub type Mask3 = Vec3<bool>;
pub type Mask4 = Vec4<bool>;

macro_rules! mask {
	($mask:ident $vec:ident { $($field:ident),+ }) => {
		//----------------------------------------------------------------
		// Comparison masks

		impl<T> $vec<T> {
			/// Masks if the components are finite.
			pub fn is_finite(self) -> $mask where T: Float {
				$vec { $($field: self.$field.is_finite()),+ }
			}
			/// Masks if the components are infinite.
			pub fn is_infinite(self) -> $mask where T: Float {
				$vec { $($field: self.$field.is_infinite()),+ }
			}
			/// Masks if the components are equal.
			pub fn eq(self, rhs: $vec<T>) -> $mask where T: PartialEq {
				$vec { $($field: self.$field == rhs.$field),+ }
			}
			/// Masks if the components are not equal.
			pub fn ne(self, rhs: $vec<T>) -> $mask where T: PartialEq {
				$vec { $($field: self.$field != rhs.$field),+ }
			}
			/// Masks if the left-hand side components are less than the right-hand side.
			pub fn lt(self, rhs: $vec<T>) -> $mask where T: PartialOrd {
				$vec { $($field: self.$field < rhs.$field),+ }
			}
			/// Masks if the left-hand side components are less than or equal the right-hand side.
			pub fn le(self, rhs: $vec<T>) -> $mask where T: PartialOrd {
				$vec { $($field: self.$field <= rhs.$field),+ }
			}
			/// Masks if the left-hand side components are greater than the right-hand side.
			pub fn gt(self, rhs: $vec<T>) -> $mask where T: PartialOrd {
				$vec { $($field: self.$field > rhs.$field),+ }
			}
			/// Masks if the left-hand side components are greater than or equal the right-hand side.
			pub fn ge(self, rhs: $vec<T>) -> $mask where T: PartialOrd {
				$vec { $($field: self.$field >= rhs.$field),+ }
			}
			/// Combines two vectors based on the mask, selecting components from the left-hand side if `true` and right-hand side if `false`.
			pub fn select(self, rhs: $vec<T>, mask: $mask) -> $vec<T> {
				$vec { $($field: if mask.$field { self.$field } else { rhs.$field }),+ }
			}
		}

		//----------------------------------------------------------------
		// Comparison operators

		impl $mask {
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
	};
}

mask!(Mask2 Vec2 { x, y });
mask!(Mask3 Vec3 { x, y, z });
mask!(Mask4 Vec4 { x, y, z, w });
