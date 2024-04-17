/*!
Packs and unpacks unsigned integers.
*/

use super::*;

//----------------------------------------------------------------
// Packed integers

impl Vec2<u32> {
	/// Unpacks `u64` into `u32 u32`.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// assert_eq!(
	/// 	Vec2 { x: 0x01010101, y: 0xFEFEFEFE },
	/// 	Vec2::unpack32(0xFEFEFEFE_01010101)
	/// );
	/// ```
	#[inline]
	pub const fn unpack32(v: u64) -> Vec2<u32> {
		Vec2 {
			x: ((v & 0x00000000FFFFFFFF) >> 0) as u32,
			y: ((v & 0xFFFFFFFF00000000) >> 32) as u32,
		}
	}
	/// Packs into `u64`.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let this: Vec2<u32> = Vec2 { x: 0x01010101, y: 0xFEFEFEFE };
	/// assert_eq!(0xFEFEFEFE_01010101, this.pack());
	/// ```
	#[inline]
	pub const fn pack(self) -> u64 {
		(self.y as u64) << 32 | (self.x as u64)
	}
}
impl Vec2<u16> {
	/// Unpacks `u32` into `u16 u16`.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// assert_eq!(
	/// 	Vec2 { x: 0x0101, y: 0xFEFE },
	/// 	Vec2::unpack16(0xFEFE_0101)
	/// );
	/// ```
	#[inline]
	pub const fn unpack16(v: u32) -> Vec2<u16> {
		Vec2 {
			x: ((v & 0x0000FFFF) >> 0) as u16,
			y: ((v & 0xFFFF0000) >> 16) as u16,
		}
	}
	/// Packs into `u32`.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let this: Vec2<u16> = Vec2 { x: 0x0101, y: 0xFEFE };
	/// assert_eq!(0xFEFE_0101, this.pack());
	/// ```
	#[inline]
	pub const fn pack(self) -> u32 {
		(self.y as u32) << 16 | (self.x as u32)
	}
}
impl Vec2<u8> {
	/// Unpacks `u16` into `u8 u8`.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// assert_eq!(
	/// 	Vec2 { x: 0x01, y: 0xFE },
	/// 	Vec2::unpack8(0xFE_01)
	/// );
	/// ```
	#[inline]
	pub const fn unpack8(v: u16) -> Vec2<u8> {
		Vec2 {
			x: ((v as u32 & 0x000000FF) >> 0) as u8,
			y: ((v as u32 & 0x0000FF00) >> 8) as u8,
		}
	}
	/// Packs into `u16`.
	///
	/// ```
	/// use cvmath::Vec2;
	///
	/// let this: Vec2<u8> = Vec2 { x: 0x01, y: 0xFE };
	/// assert_eq!(0xFE_01, this.pack());
	/// ```
	#[inline]
	pub const fn pack(self) -> u16 {
		((self.y as u32) << 8 | (self.x as u32)) as u16
	}
}
impl Vec4<u16> {
	/// Unpacks `u64` into `u16 u16 u16 u16`.
	///
	/// ```
	/// use cvmath::Vec4;
	///
	/// assert_eq!(
	/// 	Vec4 { x: 0x0101, y: 0x5656, z: 0x9A9A, w: 0xFEFE },
	/// 	Vec4::unpack16(0xFEFE_9A9A_5656_0101)
	/// );
	/// ```
	#[inline]
	pub const fn unpack16(v: u64) -> Vec4<u16> {
		Vec4 {
			x: ((v & 0x000000000000FFFF) >> 0) as u16,
			y: ((v & 0x00000000FFFF0000) >> 16) as u16,
			z: ((v & 0x0000FFFF00000000) >> 32) as u16,
			w: ((v & 0xFFFF000000000000) >> 48) as u16,
		}
	}
	/// Packs into `u64`.
	///
	/// ```
	/// use cvmath::Vec4;
	///
	/// let this: Vec4<u16> = Vec4 { x: 0x0101, y: 0x5656, z: 0x9A9A, w: 0xFEFE };
	/// assert_eq!(0xFEFE_9A9A_5656_0101, this.pack());
	/// ```
	#[inline]
	pub const fn pack(self) -> u64 {
		(self.w as u64) << 48 | (self.z as u64) << 32 | (self.y as u64) << 16 | (self.x as u64)
	}
}
impl Vec4<u8> {
	/// Unpacks `u32` into `u8 u8 u8 u8`.
	///
	/// ```
	/// use cvmath::Vec4;
	///
	/// assert_eq!(
	/// 	Vec4 { x: 0x01, y: 0x56, z: 0x9A, w: 0xFE },
	/// 	Vec4::unpack8(0xFE_9A_56_01)
	/// );
	/// ```
	///
	/// Unpacks an RGBA color value into `Vec4<f32>` where `x`: red, `y`: green, `z`: blue and `w`: alpha.
	///
	/// ```
	/// use cvmath::Vec4;
	///
	/// // 0xAABBGGRR in little endian results in RR GG BB AA bytes in memory.
	/// let rgba = 0xFF_C0_80_40;
	/// let color = Vec4::unpack8(rgba).cast::<f32>() / 255_f32;
	/// assert_eq!(Vec4 { x: 64.0/255.0, y: 128.0/255.0, z: 192.0/255.0, w: 1.0 }, color);
	/// ```
	#[inline]
	pub const fn unpack8(v: u32) -> Vec4<u8> {
		Vec4 {
			x: ((v & 0x000000FF) >> 0) as u8,
			y: ((v & 0x0000FF00) >> 8) as u8,
			z: ((v & 0x00FF0000) >> 16) as u8,
			w: ((v & 0xFF000000) >> 24) as u8,
		}
	}
	/// Packs into `u32`.
	///
	/// ```
	/// use cvmath::Vec4;
	///
	/// let this: Vec4<u8> = Vec4 { x: 0x01, y: 0x56, z: 0x9A, w: 0xFE };
	/// assert_eq!(0xFE_9A_56_01, this.pack());
	/// ```
	///
	/// Packs `Vec4<f32>` color components into an RGBA color value.
	///
	/// ```
	/// use cvmath::Vec4;
	///
	/// let color = Vec4 { x: 64.0/255.0, y: 128.0/255.0, z: 192.0/255.0, w: 1.0 };
	/// let rgba = (color * 255_f32).cast::<u8>().pack();
	/// assert_eq!(0xFF_C0_80_40, rgba);
	/// ```
	#[inline]
	pub const fn pack(self) -> u32 {
		(self.w as u32) << 24 | (self.z as u32) << 16 | (self.y as u32) << 8 | (self.x as u32)
	}
}
