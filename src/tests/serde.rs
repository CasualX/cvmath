use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
struct State {
	vec2: Vec2<i32>,
	vec3: Vec3<f64>,
	vec4: Vec4<f32>,
	bool2: Bool2,
	bool3: Bool3,
	bool4: Bool4,
	angle: Angle<f32>,
	complex: Complex<f64>,
	polar: Polar<f64>,
	quat: Quat<f32>,
	mat2: Mat2<f32>,
	mat3: Mat3<f32>,
	mat4: Mat4<f32>,
	transform2: Transform2<f32>,
	transform3: Transform3<f32>,
	point2: Point2<i32>,
	point3: Point3<f32>,
	bounds2: Bounds2<i32>,
	bounds3: Bounds3<f32>,
	line2: Line2<f64>,
	line3: Line3<f32>,
	circle: Circle<f64>,
	sphere: Sphere<f32>,
	plane2: Plane2<f64>,
	plane3: Plane3<f32>,
	triangle2: Triangle2<f64>,
	triangle3: Triangle3<f32>,
}

trait Value: Copy {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self;
}
impl Value for i32 {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self {
		rng.range(-1000..1000)
	}
}
impl Value for f32 {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self {
		(rng.range(-100.0..100.0) * 64.0).round() / 64.0
	}
}
impl Value for f64 {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self {
		(rng.range(-1000.0..1000.0) * 1024.0).round() / 1024.0
	}
}
impl Value for Angle<f32> {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self {
		Angle::deg(rng.range(-360.0..360.0).round())
	}
}
impl Value for Angle<f64> {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self {
		Angle::deg(rng.range(-360.0..360.0).round())
	}
}
impl Value for bool {
	fn next<R: urandom::Rng>(rng: &mut urandom::Random<R>) -> Self {
		rng.coin_flip()
	}
}

trait RandomValue {
	fn value<T: Value>(&mut self) -> T;
}
impl<R: urandom::Rng> RandomValue for urandom::Random<R> {
	fn value<T: Value>(&mut self) -> T {
		T::next(self)
	}
}

#[test]
fn main() {
	let mut rng = urandom::new();

	let state = State {
		vec2: Vec2(rng.value(), rng.value()),
		vec3: Vec3(rng.value(), rng.value(), rng.value()),
		vec4: Vec4(rng.value(), rng.value(), rng.value(), rng.value()),
		bool2: Bool2(rng.value(), rng.value()),
		bool3: Bool3(rng.value(), rng.value(), rng.value()),
		bool4: Bool4(rng.value(), rng.value(), rng.value(), rng.value()),
		angle: rng.value(),
		complex: Complex(rng.value(), rng.value()),
		polar: Polar::new(rng.value(), rng.value()),
		quat: Quat::new(rng.value(), rng.value(), rng.value(), rng.value()),
		mat2: Mat2::new(
			rng.value(), rng.value(),
			rng.value(), rng.value(),
		),
		mat3: Mat3::new(
			rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(),
		),
		mat4: Mat4::new(
			rng.value(), rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(), rng.value(),
		),
		transform2: Transform2::new(
			rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(),
		),
		transform3: Transform3::new(
			rng.value(), rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(), rng.value(),
			rng.value(), rng.value(), rng.value(), rng.value(),
		),
		point2: Point2(rng.value(), rng.value()),
		point3: Point3(rng.value(), rng.value(), rng.value()),
		bounds2: Bounds2::new(
			Point2(rng.value(), rng.value()),
			Point2(rng.value(), rng.value()),
		),
		bounds3: Bounds3::new(
			Point3(rng.value(), rng.value(), rng.value()),
			Point3(rng.value(), rng.value(), rng.value()),
		),
		line2: Line2::new(
			Point2(rng.value(), rng.value()),
			Point2(rng.value(), rng.value()),
		),
		line3: Line3::new(
			Point3(rng.value(), rng.value(), rng.value()),
			Point3(rng.value(), rng.value(), rng.value()),
		),
		circle: Circle::new(
			Point2(rng.value(), rng.value()),
			rng.value(),
		),
		sphere: Sphere::new(
			Point3(rng.value(), rng.value(), rng.value()),
			rng.value(),
		),
		plane2: Plane2::new(
			Vec2(rng.value(), rng.value()),
			rng.value(),
		),
		plane3: Plane3::new(
			Vec3(rng.value(), rng.value(), rng.value()),
			rng.value(),
		),
		triangle2: Triangle2::new(
			Point2(rng.value(), rng.value()),
			Point2(rng.value(), rng.value()),
			Point2(rng.value(), rng.value()),
		),
		triangle3: Triangle3::new(
			Point3(rng.value(), rng.value(), rng.value()),
			Point3(rng.value(), rng.value(), rng.value()),
			Point3(rng.value(), rng.value(), rng.value()),
		),
	};

	{
		let serialized = serde_json::to_string_pretty(&state).unwrap();
		// std::fs::write("state.json", &serialized).unwrap();
		let deserialized: State = serde_json::from_str(&serialized).unwrap();
		assert_eq!(state.vec2, deserialized.vec2);
		assert_eq!(state.vec3, deserialized.vec3);
		assert_eq!(state.vec4, deserialized.vec4);
		assert_eq!(state.bool2, deserialized.bool2);
		assert_eq!(state.bool3, deserialized.bool3);
		assert_eq!(state.bool4, deserialized.bool4);
		assert_eq!(state.angle, deserialized.angle);
		assert_eq!(state.complex, deserialized.complex);
		assert_eq!(state.polar, deserialized.polar);
		assert_eq!(state.quat, deserialized.quat);
		assert_eq!(state.mat2, deserialized.mat2);
		assert_eq!(state.mat3, deserialized.mat3);
		assert_eq!(state.mat4, deserialized.mat4);
		assert_eq!(state.transform2, deserialized.transform2);
		assert_eq!(state.transform3, deserialized.transform3);
		assert_eq!(state.point2, deserialized.point2);
		assert_eq!(state.point3, deserialized.point3);
		assert_eq!(state.bounds2, deserialized.bounds2);
		assert_eq!(state.bounds3, deserialized.bounds3);
		assert_eq!(state.line2, deserialized.line2);
		assert_eq!(state.line3, deserialized.line3);
		assert_eq!(state.circle, deserialized.circle);
		assert_eq!(state.sphere, deserialized.sphere);
		assert_eq!(state.plane2, deserialized.plane2);
		assert_eq!(state.plane3, deserialized.plane3);
		assert_eq!(state.triangle2, deserialized.triangle2);
		assert_eq!(state.triangle3, deserialized.triangle3);
	}

	// {
	// 	let serialized = ron::ser::to_string_pretty(&state, ron::ser::PrettyConfig::new().struct_names(true)).unwrap();
	// 	std::fs::write("state.ron", &serialized).unwrap();
	// 	let deserialized: State = ron::from_str(&serialized).unwrap();
	// 	assert_eq!(state, deserialized);
	// }
}
