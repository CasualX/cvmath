use std::ops;

pub trait One where Self: Sized + ops::Mul<Output = Self> {
	const ONE: Self;
}

//----------------------------------------------------------------
// Implementation

impl One for u8 {
	const ONE: u8 = 1;
}
impl One for u16 {
	const ONE: u16 = 1;
}
impl One for u32 {
	const ONE: u32 = 1;
}
impl One for u64 {
	const ONE: u64 = 1;
}

impl One for i8 {
	const ONE: i8 = 1;
}
impl One for i16 {
	const ONE: i16 = 1;
}
impl One for i32 {
	const ONE: i32 = 1;
}
impl One for i64 {
	const ONE: i64 = 1;
}

impl One for f32 {
	const ONE: f32 = 1.0;
}
impl One for f64 {
	const ONE: f64 = 1.0;
}
