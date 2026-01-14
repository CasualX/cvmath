use cvmath::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

#[path = "../../src/shape2/tests/svgwriter.rs"]
mod writer;

fn main() {
	let mut svg = writer::SvgWriter::new(WIDTH, HEIGHT);

	svg.rect(Bounds2 {
		mins: Point2(0.0, 0.0),
		maxs: Point2(WIDTH, HEIGHT),
	}).fill("#222");

	let position = Point3f(-100.0, 50.0, 20.0);
	let target = Point3f(0.0, 0.0, 0.0);
	let ref_up = Vec3f(0.0, 1.0, 0.0);
	let view = Transform3f::look_at(position, target, ref_up, Hand::RH);

	let fov_y = Angle::deg(60.0);
	let aspect_ratio = WIDTH / HEIGHT;
	let proj = Mat4f::perspective(fov_y, aspect_ratio, 0.1, 1000.0, (Hand::RH, Clip::ZO));

	let screen = Transform3f::screen(WIDTH, HEIGHT).mat4();

	// Combine the matrices
	let vp = screen * proj * view;

	let cube = Bounds3(target - Vec3(10.0, 1.0, 10.0), target + Vec3(10.0, 1.0, 10.0));

	let p1 = (vp * Vec4f(cube.mins.x, cube.mins.y, cube.mins.z, 1.0)).hdiv().xy();
	let p2 = (vp * Vec4f(cube.maxs.x, cube.mins.y, cube.mins.z, 1.0)).hdiv().xy();
	let p3 = (vp * Vec4f(cube.maxs.x, cube.maxs.y, cube.mins.z, 1.0)).hdiv().xy();
	let p4 = (vp * Vec4f(cube.mins.x, cube.maxs.y, cube.mins.z, 1.0)).hdiv().xy();
	let p5 = (vp * Vec4f(cube.mins.x, cube.mins.y, cube.maxs.z, 1.0)).hdiv().xy();
	let p6 = (vp * Vec4f(cube.maxs.x, cube.mins.y, cube.maxs.z, 1.0)).hdiv().xy();
	let p7 = (vp * Vec4f(cube.maxs.x, cube.maxs.y, cube.maxs.z, 1.0)).hdiv().xy();
	let p8 = (vp * Vec4f(cube.mins.x, cube.maxs.y, cube.maxs.z, 1.0)).hdiv().xy();

	svg.line(Line2(p1, p2)).stroke("#0f0").stroke_width(1.0);
	svg.line(Line2(p2, p3)).stroke("#0f0").stroke_width(1.0);
	svg.line(Line2(p3, p4)).stroke("#0f0").stroke_width(1.0);
	svg.line(Line2(p4, p1)).stroke("#0f0").stroke_width(1.0);

	svg.line(Line2(p5, p6)).stroke("#f0f").stroke_width(1.0);
	svg.line(Line2(p6, p7)).stroke("#f0f").stroke_width(1.0);
	svg.line(Line2(p7, p8)).stroke("#f0f").stroke_width(1.0);
	svg.line(Line2(p8, p5)).stroke("#f0f").stroke_width(1.0);

	svg.line(Line2(p1, p5)).stroke("#f00").stroke_width(1.0);
	svg.line(Line2(p2, p6)).stroke("#f00").stroke_width(1.0);
	svg.line(Line2(p3, p7)).stroke("#f00").stroke_width(1.0);
	svg.line(Line2(p4, p8)).stroke("#f00").stroke_width(1.0);

	svg.save("wireframe.svg").unwrap();
}
