/*!
Pack and unpack unsigned integers.

## Packed

`unpack32(v)`: Unpacks `u64` to 2×`u32`.

`unpack16(v)`: Unpacks `u64` to 4×`u16` or `u32` to 2×`u16`.

`unpack8(v)`: Unpacks `u32` to 4×`u8` or `u16` to 2×`u8`.

`pack(self)`: Packs back together as unsigned integer.

### Examples

```
# use cgm::prelude::{Vec2, Vec4};
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::unpack32(0x00000002_00000001));
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::unpack16(0x0002_0001));
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::unpack8(0x02_01));

assert_eq!(Vec4 { x: 1, y: 2, z: 3, w: 4 }, Vec4::unpack16(0x0004_0003_0002_0001));
assert_eq!(Vec4 { x: 1, y: 2, z: 3, w: 4 }, Vec4::unpack8(0x04_03_02_01));

// Example to unpack RGBA u32 where x: red, y: green, z: blue, w: alpha.
let color = Vec4::unpack8(0xFFC08040).cast::<f32>() / 255.0;
assert_eq!(Vec4 { x: 64.0/255.0, y: 128.0/255.0, z: 192.0/255.0, w: 1.0 }, color);
```

*/

use ::vec::{Vec2, Vec4};

//----------------------------------------------------------------
// Packed integers

impl Vec2<u32> {
	/// Unpack `u64` into `u32 u32`.
	#[inline]
	pub fn unpack32(v: u64) -> Vec2<u32> {
		Vec2 {
			x: ((v & 0x00000000FFFFFFFF) >> 0) as u32,
			y: ((v & 0xFFFFFFFF00000000) >> 32) as u32,
		}
	}
	/// Pack into `u64`.
	#[inline]
	pub fn pack(self) -> u64 {
		(self.y as u64) << 32 | (self.x as u64)
	}
}
impl Vec2<u16> {
	/// Unpack `u32` into `u16 u16`.
	#[inline]
	pub fn unpack16(v: u32) -> Vec2<u16> {
		Vec2 {
			x: ((v & 0x0000FFFF) >> 0) as u16,
			y: ((v & 0xFFFF0000) >> 16) as u16,
		}
	}
	/// Pack into `u32`.
	#[inline]
	pub fn pack(self) -> u32 {
		(self.y as u32) << 16 | (self.x as u32)
	}
}
impl Vec2<u8> {
	/// Unpack `u16` into `u8 u8`.
	#[inline]
	pub fn unpack8(v: u16) -> Vec2<u8> {
		Vec2 {
			x: ((v as u32 & 0x000000FF) >> 0) as u8,
			y: ((v as u32 & 0x0000FF00) >> 8) as u8,
		}
	}
	/// Pack into `u16`.
	#[inline]
	pub fn pack(self) -> u16 {
		((self.y as u32) << 8 | (self.x as u32)) as u16
	}
}
impl Vec4<u16> {
	/// Unpack `u64` into `u16 u16 u16 u16`.
	#[inline]
	pub fn unpack16(v: u64) -> Vec4<u16> {
		Vec4 {
			x: ((v & 0x000000000000FFFF) >> 0) as u16,
			y: ((v & 0x00000000FFFF0000) >> 16) as u16,
			z: ((v & 0x0000FFFF00000000) >> 32) as u16,
			w: ((v & 0xFFFF000000000000) >> 48) as u16,
		}
	}
	/// Pack into `u64`.
	#[inline]
	pub fn pack(self) -> u64 {
		(self.w as u64) << 48 | (self.z as u64) << 32 | (self.y as u64) << 16 | (self.x as u64)
	}
}
impl Vec4<u8> {
	/// Unpack `u32` into `u8 u8 u8 u8`.
	#[inline]
	pub fn unpack8(v: u32) -> Vec4<u8> {
		Vec4 {
			x: ((v & 0x000000FF) >> 0) as u8,
			y: ((v & 0x0000FF00) >> 8) as u8,
			z: ((v & 0x00FF0000) >> 16) as u8,
			w: ((v & 0xFF000000) >> 24) as u8,
		}
	}
	/// Pack into `u32`.
	#[inline]
	pub fn pack(self) -> u32 {
		(self.w as u32) << 24 | (self.z as u32) << 16 | (self.y as u32) << 8 | (self.x as u32)
	}
}
