use std::ops;

pub trait One where Self: Sized + ops::Mul<Output = Self> {
	fn one() -> Self;
}

//----------------------------------------------------------------
// Implementation

impl One for u8 { fn one() -> u8 { 1 } }
impl One for u16 { fn one() -> u16 { 1 } }
impl One for u32 { fn one() -> u32 { 1 } }
impl One for u64 { fn one() -> u64 { 1 } }

impl One for i8 { fn one() -> i8 { 1 } }
impl One for i16 { fn one() -> i16 { 1 } }
impl One for i32 { fn one() -> i32 { 1 } }
impl One for i64 { fn one() -> i64 { 1 } }

impl One for f32 { fn one() -> f32 { 1.0 } }
impl One for f64 { fn one() -> f64 { 1.0 } }
