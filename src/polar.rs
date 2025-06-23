use super::*;

/// Polar coordinates.
#[derive(Copy, Clone, Default, PartialEq, Hash)]
#[repr(C)]
pub struct Polar<T> {
	pub radius: T,
	pub theta: Rad<T>,
}

/// Polar constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Polar<T>(radius: T, theta: Rad<T>) -> Polar<T> {
	Polar { radius, theta }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Polar<T> {}

impl<T: Float> Polar<T> {
	#[inline]
	pub fn complex(self) -> Complex<T> {
		let (re, im) = self.theta.sin_cos();
		Complex {
			re: self.radius * re,
			im: self.radius * im,
		}
	}
}

//----------------------------------------------------------------
// Operators

impl<T: ops::Mul<Output = T> + ops::Add<Output = T>> ops::Mul<Polar<T>> for Polar<T> {
	type Output = Polar<T>;

	#[inline]
	fn mul(self, rhs: Polar<T>) -> Polar<T> {
		Polar {
			radius: self.radius * rhs.radius,
			theta: self.theta + rhs.theta,
		}
	}
}

macro_rules! impl_fmt {
	($fmt:path) => {
		impl<T: $fmt> $fmt for Polar<T> where Rad<T>: $fmt {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.radius.fmt(f)?;
				let symbol = if f.alternate() { " angle " } else { " ∠ " };
				f.write_str(symbol)?;
				<Rad<T> as $fmt>::fmt(&self.theta, f)
			}
		}
	};
}

impl_fmt!(fmt::Display);
impl_fmt!(fmt::Debug);
impl_fmt!(fmt::Binary);
impl_fmt!(fmt::Octal);
impl_fmt!(fmt::LowerHex);
impl_fmt!(fmt::UpperHex);
impl_fmt!(fmt::LowerExp);
impl_fmt!(fmt::UpperExp);
