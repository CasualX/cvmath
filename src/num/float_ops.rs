
pub trait FloatOps: Copy {
	fn is_finite(self) -> bool;
	fn is_infinite(self) -> bool;
	fn sqrt(self) -> Self;
	fn remainder(self, Self) -> Self;
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn sin_cos(self) -> (Self, Self);
	fn asin(self) -> Self;
	fn acos(self) -> Self;
	fn atan(self) -> Self;
	fn atan2(self, Self) -> Self;
}

//----------------------------------------------------------------
// Implementation

macro_rules! impl_float_ops {
	($ty:ty) => {

impl FloatOps for $ty {
	fn is_finite(self) -> bool { self.is_finite() }
	fn is_infinite(self) -> bool { self.is_infinite() }
	fn sqrt(self) -> $ty { self.sqrt() }
	fn remainder(self, y: $ty) -> $ty {
		self - ((self / y).round() * y)
	}
	fn sin(self) -> $ty { self.sin() }
	fn cos(self) -> $ty { self.cos() }
	fn tan(self) -> $ty { self.tan() }
	fn sin_cos(self) -> ($ty, $ty) { self.sin_cos() }
	fn asin(self) -> $ty { self.asin() }
	fn acos(self) -> $ty { self.acos() }
	fn atan(self) -> $ty { self.atan() }
	fn atan2(self, x: $ty) -> $ty { self.atan2(x) }
}

	}
}

impl_float_ops!(f32);
impl_float_ops!(f64);
