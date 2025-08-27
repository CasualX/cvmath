// Generates the SVG for the documentation and serves as a simple example.
// Running this example will modify the `src/vec.rs` file by adding SVG images to the doc comments.

#![allow(dead_code)]

use cvmath::*;

#[path = "../src/shape2/tests/svgwriter.rs"]
mod writer;
use writer::SvgWriter;

const ARROW_SIZE: f32 = 8.0;

//----------------------------------------------------------------

fn main() {
	write_svg("src/vec.rs:LEN_HAT", &len_hat());
	write_svg("src/vec.rs:DISTANCE_HAT", &distance_hat());
	write_svg("src/vec.rs:LERP", &lerp());
	write_svg("src/vec.rs:SLERP", &slerp());
	write_svg("src/vec.rs:NLERP", &nlerp());
	write_svg("src/vec.rs:PROJECT_SCALAR", &project_scalar());
	write_svg("src/vec.rs:REFLECT_2D", &reflect_2d());
	write_svg("src/scalar.rs:SCALAR_LERP", &scalar_lerp());
	write_svg("src/scalar.rs:SCALAR_STEP", &scalar_step());
	write_svg("src/scalar.rs:SCALAR_SMOOTHSTEP", &scalar_smoothstep());
	write_svg("src/scalar.rs:SCALAR_SMOOTHERSTEP", &scalar_smootherstep());
}

fn write_svg(id: &str, svg: &str) {
	println!("{} {}", id, svg);
	let mut s = id.split(":"); // Split between file & tag
	// Read the file contents
	use std::io::{Read, Write};
	use std::fs::File;
	let path = s.next().unwrap();
	let contents = {
		let mut file = File::open(path).unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		contents
	};
	// Replace the svg
	let tag = s.next().unwrap();
	let start = contents.find(&format!("<!--{}-->", tag)).unwrap();
	let end = start + contents[start..].find("\n").unwrap();
	let spliced = format!("{}<!--{}-->{}{}", &contents[..start], tag, svg, &contents[end..]);
	// Write the file contents back
	let mut file = File::create(path).unwrap();
	file.write_all(spliced.as_ref()).unwrap();
}

//----------------------------------------------------------------
// Drawing for len_hat

fn len_hat() -> String {
	let this = Point2(360.5, 20.0);
	let origin = Point2(40.0, 100.0);

	let vhat = Point2(this.x, origin.y);

	let mut svg = SvgWriter::new(420.0, 120.0);
	svg.arrow(origin, this, ARROW_SIZE).stroke("white");
	svg.arrow(origin, vhat, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.arrow(vhat, this, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.circle(Circle(origin, 2.0)).fill("white");
	svg.text(this + Vec2(5.0, 0.0), "this").fill("white");
	svg.text((origin + vhat) * 0.5 + Vec2(0.0, 15.0), "x").fill("grey");
	svg.text((vhat + this) * 0.5 + Vec2(5.0, 0.0), "y").fill("grey");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for distance_hat

fn distance_hat() -> String {
	let this = Point2(40.0, 100.0);
	let to = Point2(360.5, 20.0);

	let vhat = Point2(to.x, this.y);

	let mut svg = SvgWriter::new(420.0, 120.0);
	svg.line(Line2(this, to)).stroke("white");
	svg.arrow(this, vhat, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.arrow(vhat, to, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.circle(Circle(this, 2.0)).fill("white");
	svg.circle(Circle(to, 2.0)).fill("white");
	svg.text(this + Vec2(-20.0, -10.0), "this").fill("white");
	svg.text(to + Vec2(5.0, 0.0), "to").fill("white");
	svg.text((this + vhat) * 0.5 + Vec2(0.0, 15.0), "x").fill("grey");
	svg.text((vhat + to) * 0.5 + Vec2(5.0, 0.0), "y").fill("grey");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for lerp

fn lerp() -> String {
	let v1 = Point2(40.0, 100.0);
	let v2 = Point2(360.0, 20.0);

	let tgreen = 0.2;
	let vgreen = v1.lerp(v2, tgreen);

	let tblue = 0.5;
	let vblue = v1.lerp(v2, tblue);

	let mut svg = SvgWriter::new(400.0, 120.0);
	svg.line(Line2(v1, vgreen)).stroke("lime");
	svg.line(Line2(vgreen, vblue)).stroke("deepskyblue");
	svg.line(Line2(vblue, v2)).stroke("white");
	svg.circle(Circle(v1, 2.0)).fill("white");
	svg.circle(Circle(v2, 2.0)).fill("white");
	svg.circle(Circle(vgreen, 2.0)).fill("lime");
	svg.circle(Circle(vblue, 2.0)).fill("deepskyblue");
	svg.text(v1 - Vec2(20.0, 10.0), "self").fill("white");
	svg.text(v2 - Vec2(15.0, -20.0), "rhs").fill("white");
	svg.text(vgreen - Vec2(20.0, -20.0), "t = 0.2").fill("lime");
	svg.text(vblue - Vec2(20.0, -20.0), "t = 0.5").fill("deepskyblue");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for slerp and nlerp

fn slerp_nlerp<F: Fn(Point2<f32>, Point2<f32>, f32) -> Point2<f32>>(f: F, name: &str) -> String {
	let v1 = Point2(100.0, 70.0);
	let v2 = Point2(300.0, 70.0);

	// Calculate circle center given two points and a radius
	let radius = 120.0;
	let vhalf = (v1 + v2) * 0.5;
	let vdist = v1.distance(v2);
	let vbase = (v2 - v1).cw().resize((radius * radius - vdist * vdist * 0.25f32).sqrt());
	let center = vhalf + vbase; // vhalf - vbase for the other solution

	// Calculate lerp
	let lerp = v1.lerp(v2, 0.75);

	// Calculate slerps
	let leg1 = v1 - center;
	let leg2 = v2 - center;
	let slerp = center + f(leg1, leg2, 0.75);
	let p1 = center + f(leg1, leg2, 0.25);
	let p2 = center + f(leg1, leg2, 0.5);
	let cstart = center + f(leg1, leg2, -0.1);
	let cend = center + f(leg1, leg2, 1.1);

	// Render time
	let mut svg = SvgWriter::new(400.0, 140.0);
	svg.arrow(center, v1, ARROW_SIZE).stroke("white").stroke_width(0.5);
	svg.arrow(center, v2, ARROW_SIZE).stroke("white").stroke_width(0.5);
	svg.arrow(center, p1, ARROW_SIZE).stroke("lime").stroke_width(0.25);
	svg.arrow(center, p2, ARROW_SIZE).stroke("lime").stroke_width(0.25);
	svg.arrow(center, slerp, ARROW_SIZE).stroke("lime");
	svg.arc(cstart, v1, radius).stroke("white").stroke_width(0.5);
	svg.arc(v1, slerp, radius).stroke("lime");
	svg.arc(slerp, v2, radius).stroke("white");
	svg.arc(v2, cend, radius).stroke("white").stroke_width(0.5);
	svg.line(Line2(v1, lerp)).stroke("deepskyblue").stroke_width(0.5);
	svg.circle(Circle(v1, 2.0)).fill("white");
	svg.circle(Circle(v2, 2.0)).fill("white");
	svg.circle(Circle(lerp, 2.0)).fill("deepskyblue");
	svg.circle(Circle(slerp, 2.0)).fill("lime");
	svg.text(p1 - Vec2(45.0, 5.0), "t = 0.25").fill("lime").font_size(10.0);
	svg.text(p2 - Vec2(20.0, 5.0), "t = 0.50").fill("lime").font_size(10.0);
	svg.text(slerp - Vec2(0.0, 5.0), "t = 0.75").fill("lime").font_size(10.0);
	svg.text(lerp - Vec2(20.0, -20.0), "lerp").fill("deepskyblue");
	svg.text(slerp - Vec2(60.0, -10.0), name).fill("lime");
	svg.text(v1 - Vec2(50.0, 0.0), "self").fill("white");
	svg.text(v2 - Vec2(-10.0, 0.0), "rhs").fill("white");
	svg.close()
}

fn slerp() -> String {
	slerp_nlerp(Point2::slerp, "slerp")
}
fn nlerp() -> String {
	slerp_nlerp(Point2::nlerp, "nlerp")
}

//----------------------------------------------------------------
// Drawing for project_scalar

fn project_scalar() -> String {
	let v = Point2(360.0, 120.0);
	let this = Point2(200.0, 20.0);
	let origin = Point2(40.0, 160.0);

	let p = origin + (this - origin).project(v - origin);

	// Calculate the right angle symbol
	let ra = (v - origin).resize(20.0);
	let pra1 = p - ra;
	let pra2 = pra1 + ra.ccw();
	let pra3 = pra2 + ra;

	// Calculate the scalar projection length
	let offset = (p - origin).resize(15.0).cw();
	let sl1 = origin + offset;
	let sl2 = p + offset;
	let sll1 = sl1 - offset * 0.25;
	let sll2 = sl1 + offset * 0.25;
	let slr1 = sl2 - offset * 0.25;
	let slr2 = sl2 + offset * 0.25;

	// Render time
	let mut svg = SvgWriter::new(400.0, 200.0);
	svg.arrow(origin, this, ARROW_SIZE).stroke("white");
	svg.arrow(origin, v, ARROW_SIZE).stroke("white");
	svg.circle(Circle(origin, 2.0)).fill("white");
	svg.line(Line2(p, this)).stroke("white").stroke_dasharray(&[5.0, 5.0]).stroke_width(0.5);
	svg.line(Line2(pra1, pra2)).stroke("white").stroke_width(0.5);
	svg.line(Line2(pra2, pra3)).stroke("white").stroke_width(0.5);
	svg.line(Line2(sl1, sl2)).stroke("red").stroke_width(1.5);
	svg.line(Line2(sll1, sll2)).stroke("red").stroke_width(1.5);
	svg.line(Line2(slr1, slr2)).stroke("red").stroke_width(1.5);
	svg.text(this + Vec2(5.0, 5.0), "self").fill("white");
	svg.text(v + Vec2(-20.0, 22.0), "v").fill("white");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for reflect

fn reflect_2d() -> String {
	// Calculate data
	let v = Vec2 { x: 10.0, y: 2.5 };
	let this = Vec2 { x: 4.0, y: 4.0 };
	let p = this.project(v);
	let pv = p - this;
	let result = p + pv;
	let origin = Vec2::ZERO;

	// Visualize data
	let transform = Transform2::translate(Vec2(40.0f32, 120.0f32)) * Mat2::scale(Vec2(25.0, -25.0));
	let this = transform * this;
	let v = transform * v;
	let p = transform * p;
	let pv = transform * pv;
	let result = transform * result;
	let origin = transform * origin;

	let mut svg = SvgWriter::new(400.0, 200.0);
	svg.line(Line2(this, result)).stroke("white").stroke_width(0.5).stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(p, pv)).stroke("white").stroke_width(0.5).stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(pv, result)).stroke("white").stroke_width(0.5).stroke_dasharray(&[5.0, 5.0]);
	svg.arrow(origin, v, ARROW_SIZE).stroke("white");
	svg.arrow(origin, this, ARROW_SIZE).stroke("white");
	svg.arrow(origin, result, ARROW_SIZE).stroke("red");
	svg.circle(Circle(p, 2.0)).fill("white");
	svg.text(v, "v").fill("white");
	svg.text(this, "self").fill("white");
	svg.text(p + Vec2(8.0, 10.0), "p").fill("white");
	svg.text(result, "result").fill("red");
	svg.text(p.lerp(pv, 0.9) + Vec2(-15.0, -5.0), "-self").fill("white");
	svg.text(pv.lerp(result, 0.8) + Vec2(0.0, 15.0), "+p").fill("white");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for scalar lerp

fn scalar_lerp() -> String {
	let a = 40.0;
	let b = 380.0;
	let p1 = scalar::lerp(a, b, 0.25);
	let p2 = scalar::lerp(a, b, 0.5);
	let y = 40.0;

	let mut svg = SvgWriter::new(420.0, 80.0);
	svg.line(Line2(Point2(a, y), Point2(b, y))).stroke("white");
	svg.circle(Circle(Point2(a, y), 2.0)).fill("white");
	svg.circle(Circle(Point2(b, y), 2.0)).fill("white");
	svg.text(Point2(a - 4.0, y + 20.0), "a").fill("white");
	svg.text(Point2(b - 4.0, y + 20.0), "b").fill("white");
	svg.circle(Circle(Point2(p1, y), 2.0)).fill("lime");
	svg.text(Point2(p1 - 12.0, y - 15.0), "t = 0.25").fill("lime");
	svg.circle(Circle(Point2(p2, y), 2.0)).fill("deepskyblue");
	svg.text(Point2(p2 - 12.0, y - 15.0), "t = 0.5").fill("deepskyblue");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for scalar step

fn scalar_step() -> String {
	let x1 = 40.0;
	let xc = 200.0;
	let x2 = 380.0;
	let y1 = 120.0;
	let y2 = 20.0;

	let mut svg = SvgWriter::new(420.0, 140.0);
	svg.text(Point2(10.0, y1), "0").fill("white");
	svg.text(Point2(10.0, y2), "1").fill("white");
	svg.line(Line2(Point2(x1, y1), Point2(xc, y1))).stroke("white");
	svg.line(Line2(Point2(xc, y1), Point2(xc, y2))).stroke("grey").stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(Point2(xc, y2), Point2(x2, y2))).stroke("white");
	svg.text(Point2(xc + 40.0, y2 + 15.0), "x ≥ edge").fill("white");
	svg.text(Point2(xc - 100.0, y1 + 15.0), "x < edge").fill("white");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for scalar smooth(er)step

fn scalar_smoothstep() -> String {
	let x1 = 40.0;
	let edge0 = 120.0;
	let edge1 = 320.0;
	let x2 = 380.0;
	let y1 = 120.0;
	let y2 = 20.0;

	let mut svg = SvgWriter::new(420.0, 140.0);
	svg.text(Point2(10.0, y1), "0").fill("white");
	svg.text(Point2(10.0, y2), "1").fill("white");
	svg.line(Line2(Point2(edge0, y1), Point2(edge0, y2))).stroke("grey").stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(Point2(edge1, y1), Point2(edge1, y2))).stroke("grey").stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(Point2(x1, y1), Point2(edge0, y1))).stroke("white");
	// svg.line(Line2(Point2(edge0, y1), Point2(edge1, y2))).stroke("white");
	svg.polyline((0..=100).map(|i| Point2(
		scalar::lerp(edge0, edge1, i as f32 / 100.0),
		scalar::lerp(y1, y2, scalar::smoothstep(0.0, 1.0, i as f32 / 100.0))
	))).stroke("white").fill("none").stroke_width(1.5);
	svg.line(Line2(Point2(edge1, y2), Point2(x2, y2))).stroke("white");
	svg.text(Point2(edge0 - 20.0, y1 + 15.0), "edge0").fill("white");
	svg.text(Point2(edge1 - 20.0, y1 + 15.0), "edge1").fill("white");
	svg.close()
}

fn scalar_smootherstep() -> String {
	let x1 = 40.0;
	let edge0 = 120.0;
	let edge1 = 320.0;
	let x2 = 380.0;
	let y1 = 120.0;
	let y2 = 20.0;

	let mut svg = SvgWriter::new(420.0, 140.0);
	svg.text(Point2(10.0, y1), "0").fill("white");
	svg.text(Point2(10.0, y2), "1").fill("white");
	svg.line(Line2(Point2(edge0, y1), Point2(edge0, y2))).stroke("grey").stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(Point2(edge1, y1), Point2(edge1, y2))).stroke("grey").stroke_dasharray(&[5.0, 5.0]);
	svg.line(Line2(Point2(x1, y1), Point2(edge0, y1))).stroke("white");
	// svg.line(Line2(Point2(edge0, y1), Point2(edge1, y2))).stroke("white");
	svg.polyline((0..=100).map(|i| Point2(
		scalar::lerp(edge0, edge1, i as f32 / 100.0),
		scalar::lerp(y1, y2, scalar::smootherstep(0.0, 1.0, i as f32 / 100.0))
	))).stroke("white").fill("none").stroke_width(1.5);
	svg.line(Line2(Point2(edge1, y2), Point2(x2, y2))).stroke("white");
	svg.text(Point2(edge0 - 20.0, y1 + 15.0), "edge0").fill("white");
	svg.text(Point2(edge1 - 20.0, y1 + 15.0), "edge1").fill("white");
	svg.close()
}
