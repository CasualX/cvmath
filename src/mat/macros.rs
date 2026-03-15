
macro_rules! impl_mat_add_mat {
	($mat:ident) => {
		// mat + mat
		impl<T> ops::Add<&$mat<T>> for $mat<T> where $mat<T>: ops::Add<$mat<T>, Output=$mat<T>>, $mat<T>: Copy {
			type Output = $mat<T>;
			#[inline]
			fn add(self, rhs: &$mat<T>) -> $mat<T> {
				ops::Add::add(self, *rhs)
			}
		}
		impl<T> ops::Add<&$mat<T>> for &$mat<T> where $mat<T>: Copy + ops::Add<$mat<T>, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn add(self, rhs: &$mat<T>) -> $mat<T> {
				ops::Add::add(*self, *rhs)
			}
		}
		impl<T> ops::Add<$mat<T>> for &$mat<T> where $mat<T>: Copy + ops::Add<$mat<T>, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn add(self, rhs: $mat<T>) -> $mat<T> {
				ops::Add::add(*self, rhs)
			}
		}
		// mat += mat
		impl<T> ops::AddAssign<&$mat<T>> for $mat<T> where $mat<T>: ops::AddAssign<$mat<T>>, $mat<T>: Copy {
			#[inline]
			fn add_assign(&mut self, rhs: &$mat<T>) {
				ops::AddAssign::add_assign(self, *rhs)
			}
		}
	};
}

macro_rules! impl_mat_sub_mat {
	($mat:ident) => {
		// mat - mat
		impl<T> ops::Sub<&$mat<T>> for $mat<T> where $mat<T>: ops::Sub<$mat<T>, Output=$mat<T>>, $mat<T>: Copy {
			type Output = $mat<T>;
			#[inline]
			fn sub(self, rhs: &$mat<T>) -> $mat<T> {
				ops::Sub::sub(self, *rhs)
			}
		}
		impl<T> ops::Sub<&$mat<T>> for &$mat<T> where $mat<T>: Copy + ops::Sub<$mat<T>, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn sub(self, rhs: &$mat<T>) -> $mat<T> {
				ops::Sub::sub(*self, *rhs)
			}
		}
		impl<T> ops::Sub<$mat<T>> for &$mat<T> where $mat<T>: Copy + ops::Sub<$mat<T>, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn sub(self, rhs: $mat<T>) -> $mat<T> {
				ops::Sub::sub(*self, rhs)
			}
		}
		// mat -= mat
		impl<T> ops::SubAssign<&$mat<T>> for $mat<T> where $mat<T>: ops::SubAssign<$mat<T>>, $mat<T>: Copy {
			#[inline]
			fn sub_assign(&mut self, rhs: &$mat<T>) {
				ops::SubAssign::sub_assign(self, *rhs)
			}
		}
	};
}

macro_rules! impl_mat_neg {
	($mat:ident) => {
		impl<T> ops::Neg for &$mat<T> where $mat<T>: Copy + ops::Neg<Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn neg(self) -> $mat<T> {
				ops::Neg::neg(*self)
			}
		}
	};
}

macro_rules! impl_mat_mul_scalar {
	($mat:ident) => {
		// mat * scalar
		impl<T> ops::Mul<&T> for $mat<T> where $mat<T>: ops::Mul<T, Output=$mat<T>>, T: Copy {
			type Output = $mat<T>;
			#[inline]
			fn mul(self, rhs: &T) -> $mat<T> {
				ops::Mul::mul(self, *rhs)
			}
		}
		impl<T> ops::Mul<&T> for &$mat<T> where $mat<T>: Copy + ops::Mul<T, Output=$mat<T>>, T: Copy {
			type Output = $mat<T>;
			#[inline]
			fn mul(self, rhs: &T) -> $mat<T> {
				ops::Mul::mul(*self, *rhs)
			}
		}
		impl<T> ops::Mul<T> for &$mat<T> where $mat<T>: Copy + ops::Mul<T, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn mul(self, rhs: T) -> $mat<T> {
				ops::Mul::mul(*self, rhs)
			}
		}
		// mat *= T
		impl<T> ops::MulAssign<&T> for $mat<T> where $mat<T>: ops::MulAssign<T>, T: Copy {
			#[inline]
			fn mul_assign(&mut self, rhs: &T) {
				ops::MulAssign::mul_assign(self, *rhs)
			}
		}
	}
}

macro_rules! impl_mat_mul_vec {
	($mat:ident, $vec:ident) => {
		// mat * vec
		impl<T> ops::Mul<&$vec<T>> for $mat<T> where $mat<T>: ops::Mul<$vec<T>, Output=$vec<T>>, $vec<T>: Copy {
			type Output = $vec<T>;
			#[inline]
			fn mul(self, rhs: &$vec<T>) -> $vec<T> {
				ops::Mul::mul(self, *rhs)
			}
		}
		impl<T> ops::Mul<&$vec<T>> for &$mat<T> where $mat<T>: Copy + ops::Mul<$vec<T>, Output=$vec<T>>, $vec<T>: Copy {
			type Output = $vec<T>;
			#[inline]
			fn mul(self, rhs: &$vec<T>) -> $vec<T> {
				ops::Mul::mul(*self, *rhs)
			}
		}
		impl<T> ops::Mul<$vec<T>> for &$mat<T> where $mat<T>: Copy + ops::Mul<$vec<T>, Output=$vec<T>> {
			type Output = $vec<T>;
			#[inline]
			fn mul(self, rhs: $vec<T>) -> $vec<T> {
				ops::Mul::mul(*self, rhs)
			}
		}
	};
}

macro_rules! impl_mat_mul_mat {
	($mat:ident) => {
		// mat * mat
		impl<T> ops::Mul<&$mat<T>> for $mat<T> where $mat<T>: ops::Mul<$mat<T>, Output=$mat<T>>, $mat<T>: Copy {
			type Output = $mat<T>;
			#[inline]
			fn mul(self, rhs: &$mat<T>) -> $mat<T> {
				ops::Mul::mul(self, *rhs)
			}
		}
		impl<T> ops::Mul<&$mat<T>> for &$mat<T> where $mat<T>: Copy + ops::Mul<$mat<T>, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn mul(self, rhs: &$mat<T>) -> $mat<T> {
				ops::Mul::mul(*self, *rhs)
			}
		}
		impl<T> ops::Mul<$mat<T>> for &$mat<T> where $mat<T>: Copy + ops::Mul<$mat<T>, Output=$mat<T>> {
			type Output = $mat<T>;
			#[inline]
			fn mul(self, rhs: $mat<T>) -> $mat<T> {
				ops::Mul::mul(*self, rhs)
			}
		}
		// mat *= mat
		impl<T> ops::MulAssign<&$mat<T>> for $mat<T> where $mat<T>: ops::MulAssign<$mat<T>>, $mat<T>: Copy {
			#[inline]
			fn mul_assign(&mut self, rhs: &$mat<T>) {
				ops::MulAssign::mul_assign(self, *rhs)
			}
		}
	};
}
