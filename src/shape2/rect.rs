use super::*;

/// Rect shape.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Rect<T> {
	pub x: T,
	pub y: T,
	pub width: T,
	pub height: T,
}

/// Rect constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Rect<T>(x: T, y: T, width: T, height: T) -> Rect<T> {
	Rect { x, y, width, height }
}

/// Rect constructor.
///
/// ```
/// use cvmath::Rect;
///
/// let rect = cvmath::Rect!(1, 2, 3, 4);
/// let zero: Rect<i32> = cvmath::Rect!();
///
/// assert_eq!(rect, Rect(1, 2, 3, 4));
/// assert_eq!(zero, Rect::ZERO);
/// ```
#[macro_export]
macro_rules! Rect {
	($x:expr, $y:expr, $width:expr, $height:expr $(,)?) => {
		$crate::Rect { x: $x, y: $y, width: $width, height: $height }
	};
	() => {
		$crate::Rect::ZERO
	};
}

specialized_type!(Rect, Rectf, f32, x, y, width, height);
specialized_type!(Rect, Rectd, f64, x, y, width, height);
specialized_type!(Rect, Recti, i32, x, y, width, height);

#[cfg(feature = "dataview")]
unsafe impl<T: dataview::Pod> dataview::Pod for Rect<T> {}

impl<T: Zero> Rect<T> {
	/// Zero rect.
	pub const ZERO: Rect<T> = Rect { x: T::ZERO, y: T::ZERO, width: T::ZERO, height: T::ZERO };
}
impl<T: Zero + One> Rect<T> {
	/// Unit rect.
	pub const UNIT: Rect<T> = Rect { x: T::ZERO, y: T::ZERO, width: T::ONE, height: T::ONE };
}

impl<T> Rect<T> {
	/// Constructs a new rect.
	#[inline]
	pub const fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
		Rect { x, y, width, height }
	}

	/// Constructs a rect from its top-left point and size.
	#[inline]
	pub fn point(point: Point2<T>, size: Vec2<T>) -> Rect<T> {
		Rect { x: point.x, y: point.y, width: size.x, height: size.y }
	}

	/// Casts the rect to a different unit type.
	#[inline]
	pub fn cast<U>(self) -> Rect<U> where T: CastTo<U> {
		Rect {
			x: self.x.cast_to(),
			y: self.y.cast_to(),
			width: self.width.cast_to(),
			height: self.height.cast_to(),
		}
	}
}

impl<T: Copy> Rect<T> {
	/// Top-left point of the rect.
	#[inline]
	pub const fn position(&self) -> Point2<T> {
		Point2 { x: self.x, y: self.y }
	}

	/// Size of the rect.
	#[inline]
	pub const fn size(&self) -> Vec2<T> {
		Vec2 { x: self.width, y: self.height }
	}
}

impl<T> Rect<T> {
	/// Converts the rect to bounds.
	#[inline]
	pub fn bounds(&self) -> Bounds2<T> where T: Copy + ops::Add<Output = T> {
		Bounds2 {
			mins: Point2(self.x, self.y),
			maxs: Point2(self.x + self.width, self.y + self.height),
		}
	}

	/// Normalizes the rect so its width and height are non-negative.
	#[inline]
	pub fn norm(self) -> Rect<T> where T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + Extrema {
		self.bounds().norm().into()
	}
}

impl<T: Scalar> Rect<T> {
	/// X coordinate of the left side.
	#[inline]
	pub const fn left(&self) -> T {
		self.x
	}
	/// X coordinate of the right side.
	#[inline]
	pub fn right(&self) -> T {
		self.x + self.width
	}
	/// Y coordinate of the top side.
	#[inline]
	pub const fn top(&self) -> T {
		self.y
	}
	/// Y coordinate of the bottom side.
	#[inline]
	pub fn bottom(&self) -> T {
		self.y + self.height
	}
	/// Area of the rect.
	#[inline]
	pub fn area(&self) -> T {
		self.width * self.height
	}
	/// Center of the rect.
	#[inline]
	pub fn center(&self) -> Point2<T> {
		let two = T::ONE + T::ONE;
		Point2(self.x + self.width / two, self.y + self.height / two)
	}
	/// Top left corner of the rect.
	#[inline]
	pub const fn top_left(&self) -> Point2<T> {
		Point2 { x: self.x, y: self.y }
	}
	/// Top right corner of the rect.
	#[inline]
	pub fn top_right(&self) -> Point2<T> {
		Point2 { x: self.right(), y: self.y }
	}
	/// Bottom left corner of the rect.
	#[inline]
	pub fn bottom_left(&self) -> Point2<T> {
		Point2 { x: self.x, y: self.bottom() }
	}
	/// Bottom right corner of the rect.
	#[inline]
	pub fn bottom_right(&self) -> Point2<T> {
		Point2 { x: self.right(), y: self.bottom() }
	}
	/// Top side of the rect.
	#[inline]
	pub fn top_side(&self) -> Line2<T> {
		Line2 {
			start: self.top_left(),
			end: self.top_right(),
		}
	}
	/// Right side of the rect.
	#[inline]
	pub fn right_side(&self) -> Line2<T> {
		Line2 {
			start: self.top_right(),
			end: self.bottom_right(),
		}
	}
	/// Bottom side of the rect.
	#[inline]
	pub fn bottom_side(&self) -> Line2<T> {
		Line2 {
			start: self.bottom_right(),
			end: self.bottom_left(),
		}
	}
	/// Left side of the rect.
	#[inline]
	pub fn left_side(&self) -> Line2<T> {
		Line2 {
			start: self.bottom_left(),
			end: self.top_left(),
		}
	}
	/// Computes the aspect ratio _(width / height)_.
	#[inline]
	pub fn aspect_ratio(&self) -> T where T: Float {
		self.width / self.height
	}
	/// Transform of the unit square.
	#[inline]
	pub fn transform(self) -> Transform2<T> {
		Transform2::compose(
			Vec2(self.width, T::ZERO),
			Vec2(T::ZERO, self.height),
			Point2(self.x, self.y),
		)
	}
}

impl<T> Rect<T> {
	/// Returns whether the point `rhs` is contained within `self`.
	#[inline]
	pub fn contains(&self, rhs: Point2<T>) -> bool where T: Copy + ops::Add<Output = T> + PartialOrd {
		self.bounds().contains(rhs)
	}
	/// Returns whether the rect `rhs` is fully contained within `self`.
	#[inline]
	pub fn encloses(&self, rhs: Rect<T>) -> bool where T: Copy + ops::Add<Output = T> + PartialOrd + fmt::Debug {
		self.bounds().encloses(rhs.bounds())
	}
	/// Returns whether `rhs` is overlapped with `self`.
	#[inline]
	pub fn overlaps(&self, rhs: Rect<T>) -> bool where T: Copy + ops::Add<Output = T> + PartialOrd {
		self.bounds().overlaps(rhs.bounds())
	}
	/// Returns the overlapping area (if any) between `rhs` and `self`.
	#[inline]
	pub fn intersect(self, rhs: Rect<T>) -> Option<Rect<T>> where T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + PartialOrd + Extrema {
		self.bounds().intersect(rhs.bounds()).map(Into::into)
	}
}

impl<T: Copy + ops::Sub<Output = T>> From<Bounds2<T>> for Rect<T> {
	#[inline]
	fn from(bounds: Bounds2<T>) -> Rect<T> {
		Rect {
			x: bounds.mins.x,
			y: bounds.mins.y,
			width: bounds.maxs.x - bounds.mins.x,
			height: bounds.maxs.y - bounds.mins.y,
		}
	}
}
impl<T: Copy + ops::Add<Output = T>> From<Rect<T>> for Bounds2<T> {
	#[inline]
	fn from(rect: Rect<T>) -> Bounds2<T> {
		rect.bounds()
	}
}

impl<T: Copy + ops::Add<T, Output = T>> ops::Add<Vec2<T>> for Rect<T> {
	type Output = Rect<T>;
	#[inline]
	fn add(self, rhs: Vec2<T>) -> Rect<T> {
		Rect {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			..self
		}
	}
}
impl<T: Copy + ops::Sub<T, Output = T>> ops::Sub<Vec2<T>> for Rect<T> {
	type Output = Rect<T>;
	#[inline]
	fn sub(self, rhs: Vec2<T>) -> Rect<T> {
		Rect {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			..self
		}
	}
}
impl<T: Copy + ops::AddAssign<T>> ops::AddAssign<Vec2<T>> for Rect<T> {
	#[inline]
	fn add_assign(&mut self, rhs: Vec2<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}
impl<T: Copy + ops::SubAssign<T>> ops::SubAssign<Vec2<T>> for Rect<T> {
	#[inline]
	fn sub_assign(&mut self, rhs: Vec2<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T: Scalar> Lerp for Rect<T> {
	type T = T;

	#[inline]
	fn lerp(self, target: Rect<T>, t: T) -> Rect<T> {
		Rect {
			x: lerp(self.x, target.x, t),
			y: lerp(self.y, target.y, t),
			width: lerp(self.width, target.width, t),
			height: lerp(self.height, target.height, t),
		}
	}
}

impl<T> AsRef<[T; 4]> for Rect<T> {
	#[inline]
	fn as_ref(&self) -> &[T; 4] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> AsMut<[T; 4]> for Rect<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T; 4] {
		unsafe { core::mem::transmute(self) }
	}
}
impl<T> From<[T; 4]> for Rect<T> {
	#[inline]
	fn from([x, y, width, height]: [T; 4]) -> Rect<T> {
		Rect { x, y, width, height }
	}
}
impl<T> From<Rect<T>> for [T; 4] {
	#[inline]
	fn from(rect: Rect<T>) -> [T; 4] {
		[rect.x, rect.y, rect.width, rect.height]
	}
}

//----------------------------------------------------------------

#[cfg(feature = "urandom")]
impl<T> urandom::Distribution<Rect<T>> for urandom::distr::StandardUniform where
	urandom::distr::StandardUniform: urandom::Distribution<T>,
{
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Rect<T> {
		let distr = urandom::distr::StandardUniform;
		let x = distr.sample(rand);
		let y = distr.sample(rand);
		let width = distr.sample(rand);
		let height = distr.sample(rand);
		Rect { x, y, width, height }
	}
}

#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::SampleUniform for Rect<T> {
	type Sampler = Rect<urandom::distr::Uniform<T>>;
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::distr::UniformSampler<Rect<T>> for Rect<urandom::distr::Uniform<T>> {
	#[inline]
	fn try_new(low: Rect<T>, high: Rect<T>) -> Result<Self, urandom::distr::UniformError> {
		let x = urandom::distr::Uniform::try_new(low.x, high.x)?;
		let y = urandom::distr::Uniform::try_new(low.y, high.y)?;
		let width = urandom::distr::Uniform::try_new(low.width, high.width)?;
		let height = urandom::distr::Uniform::try_new(low.height, high.height)?;
		Ok(Rect { x, y, width, height })
	}
	#[inline]
	fn try_new_inclusive(low: Rect<T>, high: Rect<T>) -> Result<Self, urandom::distr::UniformError> where Self: Sized {
		let x = urandom::distr::Uniform::try_new_inclusive(low.x, high.x)?;
		let y = urandom::distr::Uniform::try_new_inclusive(low.y, high.y)?;
		let width = urandom::distr::Uniform::try_new_inclusive(low.width, high.width)?;
		let height = urandom::distr::Uniform::try_new_inclusive(low.height, high.height)?;
		Ok(Rect { x, y, width, height })
	}
}
#[cfg(feature = "urandom")]
impl<T: urandom::distr::SampleUniform> urandom::Distribution<Rect<T>> for Rect<urandom::distr::Uniform<T>> {
	#[inline]
	fn sample<R: urandom::Rng + ?Sized>(&self, rand: &mut urandom::Random<R>) -> Rect<T> {
		let x = self.x.sample(rand);
		let y = self.y.sample(rand);
		let width = self.width.sample(rand);
		let height = self.height.sample(rand);
		Rect { x, y, width, height }
	}
}

//----------------------------------------------------------------

impl<T: Float> Trace2<T> for Rect<T> {
	#[inline]
	fn inside(&self, pt: Point2<T>) -> bool {
		self.contains(pt)
	}

	#[inline]
	fn trace(&self, ray: &Ray2<T>) -> Option<Hit2<T>> {
		self.bounds().trace(ray)
	}
}
