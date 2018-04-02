use std::ops;

pub trait Zero where Self: Sized + ops::Add<Output = Self> + ops::Mul<Output = Self> {
	fn zero() -> Self;
}

//----------------------------------------------------------------
// Implementation

impl Zero for u8 { fn zero() -> u8 { 0 } }
impl Zero for u16 { fn zero() -> u16 { 0 } }
impl Zero for u32 { fn zero() -> u32 { 0 } }
impl Zero for u64 { fn zero() -> u64 { 0 } }

impl Zero for i8 { fn zero() -> i8 { 0 } }
impl Zero for i16 { fn zero() -> i16 { 0 } }
impl Zero for i32 { fn zero() -> i32 { 0 } }
impl Zero for i64 { fn zero() -> i64 { 0 } }

impl Zero for f32 { fn zero() -> f32 { 0.0 } }
impl Zero for f64 { fn zero() -> f64 { 0.0 } }
