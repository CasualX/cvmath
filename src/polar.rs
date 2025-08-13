use super::*;

/// Polar coordinates.
#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Polar<T> {
	pub radius: T,
	pub theta: Angle<T>,
}

/// Polar constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Polar<T>(radius: T, theta: Angle<T>) -> Polar<T> {
	Polar { radius, theta }
}

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Polar<T> {}

impl<T> Polar<T> {
	/// Constructs a new polar coordinate from components.
	#[inline]
	pub const fn new(radius: T, theta: Angle<T>) -> Polar<T> {
		Polar { radius, theta }
	}
}

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
		impl<T: $fmt> $fmt for Polar<T> where Angle<T>: $fmt {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.radius.fmt(f)?;
				let symbol = if f.alternate() { " angle " } else { " ∠ " };
				f.write_str(symbol)?;
				<Angle<T> as $fmt>::fmt(&self.theta, f)
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

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Polar<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<T> + urandom::Distribution<Angle<T>>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Polar<T> {
		let radius = rand.sample(self);
		let theta = rand.sample(self);
		Polar { radius, theta }
	}
}

//----------------------------------------------------------------
// Serialization

#[cfg(feature = "serde")]
impl<T: serde::Serialize + 'static> serde::Serialize for Polar<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeTupleStruct;
		let mut state = serializer.serialize_tuple_struct("Polar", 2)?;
		state.serialize_field(&self.radius)?;
		state.serialize_field(&self.theta)?;
		state.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de> + 'static> serde::Deserialize<'de> for Polar<T> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let (radius, theta) = {
			#[derive(serde::Deserialize)]
			struct Polar<T: 'static>(T, Angle<T>);
			let Polar(radius, theta) = Polar::<T>::deserialize(deserializer)?;
			(radius, theta)
		};
		Ok(Polar { radius, theta })
	}
}
