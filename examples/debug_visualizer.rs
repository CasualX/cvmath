//! Example for checking embedded Natvis debugger visualizers.
//! Set a breakpoint on `zzz();` and inspect the local variables in a Windows MSVC debugger.

use cvmath::*;

#[allow(unused_variables)]
fn main() {
	let angle_f32 = Angle::deg(33.333f32);
	let angle_f64 = Angle::deg(123.456f64);

	let complex_f32 = Complex(0.75f32, -1.25f32);
	let complex_f64 = Complex(-2.5f64, 3.75f64);

	let polar_f32 = Polar(2.5f32, Angle::deg(30.0f32));
	let polar_f64 = Polar(4.0f64, Angle::deg(-60.0f64));

	let quat_f32 = Quat::from_axis_angle(Vec3(0.0f32, 1.0, 0.0), Angle::deg(45.0f32));
	let quat_f64 = Quat::from_axis_angle(Vec3(1.0f64, 0.0, 0.0), Angle::deg(120.0f64));

	let rotvec_f32 = RotationVector::from((Vec3(0.0f32, 0.0, 1.0), Angle::deg(135.0f32)));
	let rotvec_f64 = RotationVector::from((Vec3(0.0f64, 1.0, 0.0), Angle::deg(22.5f64)));
	let rotvec_i32 = RotationVector(Vec3(1, 2, 3));

	let vec2_f32 = Vec2(3.0f32, 4.0f32);
	let vec2_f64 = Vec2(-3.5f64, 8.25f64);
	let vec2_i32 = Vec2(7, -9);

	let vec3_f32 = Vec3(1.0f32, 2.0f32, 2.0f32);
	let vec3_f64 = Vec3(-1.5f64, 4.0f64, -2.5f64);
	let vec3_i32 = Vec3(3, 4, 5);

	let vec4_f32 = Vec4(1.0f32, -2.0f32, 3.0f32, -4.0f32);
	let vec4_f64 = Vec4(-1.0f64, 0.5f64, 2.5f64, -3.5f64);
	let vec4_i32 = Vec4(1, 2, 3, 4);

	let mat2 = Mat2(
		1.0f32, 2.0f32,
		3.0f32, 4.0f32,
	);
	let mat3 = Mat3(
		1.0f32, 2.0f32, 3.0f32,
		4.0f32, 5.0f32, 6.0f32,
		7.0f32, 8.0f32, 9.0f32,
	);
	let mat4 = Mat4(
		1.0f32, 0.0f32, 0.0f32, 10.0f32,
		0.0f32, 1.0f32, 0.0f32, 20.0f32,
		0.0f32, 0.0f32, 1.0f32, 30.0f32,
		0.0f32, 0.0f32, 0.0f32, 1.0f32,
	);

	let transform2 = Transform2::translation(Vec2(5.0f32, -2.0f32)) * Transform2::rotation(Angle::deg(15.0f32));
	let transform3 = Transform3::translation(Vec3(1.0f32, 2.0f32, 3.0f32)) * Transform3::rotation(Vec3::Y, Angle::deg(25.0f32));

	zzz();
}

#[inline(never)]
fn zzz() {}