use core::arch::x86_64::*;

#[inline]
pub fn min_f32(a: f32, b: f32) -> f32 {
	unsafe {
		let va = _mm_set_ss(a);
		let vb = _mm_set_ss(b);
		let min = _mm_min_ss(va, vb);
		_mm_cvtss_f32(min)
	}
}

#[inline]
pub fn max_f32(a: f32, b: f32) -> f32 {
	unsafe {
		let va = _mm_set_ss(a);
		let vb = _mm_set_ss(b);
		let max = _mm_max_ss(va, vb);
		_mm_cvtss_f32(max)
	}
}

#[inline]
pub fn min_f64(a: f64, b: f64) -> f64 {
	unsafe {
		let va = _mm_set_sd(a);
		let vb = _mm_set_sd(b);
		let min = _mm_min_sd(va, vb);
		_mm_cvtsd_f64(min)
	}
}

#[inline]
pub fn max_f64(a: f64, b: f64) -> f64 {
	unsafe {
		let va = _mm_set_sd(a);
		let vb = _mm_set_sd(b);
		let max = _mm_max_sd(va, vb);
		_mm_cvtsd_f64(max)
	}
}
