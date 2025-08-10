
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Union<S1, S2> {
	pub shape1: S1,
	pub shape2: S2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<S1, S2> {
	pub shape1: S1,
	pub shape2: S2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Difference<S1, S2> {
	pub shape1: S1,
	pub shape2: S2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Xor<S1, S2> {
	pub shape1: S1,
	pub shape2: S2,
}
