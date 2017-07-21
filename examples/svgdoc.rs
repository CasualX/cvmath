/*!
Generates the SVG for the documentation and serves as a simple example.
*/

#![allow(dead_code)]

extern crate cvmath;
use cvmath::prelude::*;
type Line2<T> = ::std::ops::Range<Point2<T>>;

mod svg;
use self::svg::SvgWriter;

const ARROW_SIZE: f32 = 8.0;

//----------------------------------------------------------------

fn main() {
	write_svg("src/vec.rs:LEN_HAT", &len_hat());
	write_svg("src/vec.rs:DIST_HAT", &dist_hat());
	write_svg("src/vec.rs:LERP", &lerp());
	write_svg("src/vec.rs:SLERP", &slerp());
	write_svg("src/vec.rs:NLERP", &nlerp());
	write_svg("src/vec.rs:SCALAR_PROJECT", &scalar_project());
	write_svg("src/vec.rs:REFLECT_2D", &reflect_2d());
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

	let mut svg = SvgWriter::new(400, 120);
	svg.arrow(origin..this, ARROW_SIZE).stroke("black");
	svg.arrow(origin..vhat, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.arrow(vhat..this, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.circle(origin, 2.0);
	svg.text(this + Point2(5.0, 0.0), "this");
	svg.text((origin + vhat) * 0.5 + Point2(0.0, 15.0), "x").fill("grey");
	svg.text((vhat + this) * 0.5 + Point2(5.0, 0.0), "y").fill("grey");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for dist_hat

fn dist_hat() -> String {
	let this = Point2(40.0, 100.0);
	let to = Point2(360.5, 20.0);

	let vhat = Point2(to.x, this.y);

	let mut svg = SvgWriter::new(400, 120);
	svg.line(this..to).stroke("black");
	svg.arrow(this..vhat, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.arrow(vhat..to, ARROW_SIZE).stroke("grey").stroke_width(0.5);
	svg.circle(this, 2.0);
	svg.circle(to, 2.0);
	svg.text(this + Point2(-20.0, -10.0), "this");
	svg.text(to + Point2(5.0, 0.0), "to");
	svg.text((this + vhat) * 0.5 + Point2(0.0, 15.0), "x").fill("grey");
	svg.text((vhat + to) * 0.5 + Point2(5.0, 0.0), "y").fill("grey");
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

	let mut svg = SvgWriter::new(400, 120);
	svg.line(v1..vgreen).stroke("green");
	svg.line(vgreen..vblue).stroke("blue");
	svg.line(vblue..v2).stroke("black");
	svg.circle(v1, 2.0).fill("black");
	svg.circle(v2, 2.0).fill("black");
	svg.circle(vgreen, 2.0).fill("green");
	svg.circle(vblue, 2.0).fill("blue");
	svg.text(v1 - Point2(20.0, 10.0), "self").fill("black");
	svg.text(v2 - Point2(15.0, -20.0), "rhs").fill("black");
	svg.text(vgreen - Point2(20.0, -20.0), "t = 0.2").fill("green");
	svg.text(vblue - Point2(20.0, -20.0), "t = 0.5").fill("blue");
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
	let vdist = v1.dist(v2);
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
	let mut svg = SvgWriter::new(400, 140);
	svg.arrow(center..v1, ARROW_SIZE).stroke("black").stroke_width(0.5);
	svg.arrow(center..v2, ARROW_SIZE).stroke("black").stroke_width(0.5);
	svg.arrow(center..p1, ARROW_SIZE).stroke("green").stroke_width(0.25);
	svg.arrow(center..p2, ARROW_SIZE).stroke("green").stroke_width(0.25);
	svg.arrow(center..slerp, ARROW_SIZE).stroke("green");
	svg.arc(cstart, v1, radius).stroke("black").stroke_width(0.5);
	svg.arc(v1, slerp, radius).stroke("green");
	svg.arc(slerp, v2, radius).stroke("black");
	svg.arc(v2, cend, radius).stroke("black").stroke_width(0.5);
	svg.line(v1..lerp).stroke("blue").stroke_width(0.5);
	svg.circle(v1, 2.0).fill("black");
	svg.circle(v2, 2.0).fill("black");
	svg.circle(lerp, 2.0).fill("blue");
	svg.circle(slerp, 2.0).fill("green");
	svg.text(p1 - Point2(45.0, 5.0), "t = 0.25").fill("green").font_size(10.0);
	svg.text(p2 - Point2(20.0, 5.0), "t = 0.50").fill("green").font_size(10.0);
	svg.text(slerp - Point2(0.0, 5.0), "t = 0.75").fill("green").font_size(10.0);
	svg.text(lerp - Point2(20.0, -20.0), "lerp").fill("blue");
	svg.text(slerp - Point2(60.0, -10.0), name).fill("green");
	svg.text(v1 - Point2(50.0, 0.0), "self").fill("black");
	svg.text(v2 - Point2(-10.0, 0.0), "rhs").fill("black");
	svg.close()
}

fn slerp() -> String {
	slerp_nlerp(Point2::slerp, "slerp")
}
fn nlerp() -> String {
	slerp_nlerp(Point2::nlerp, "nlerp")
}

//----------------------------------------------------------------
// Drawing for scalar_project

fn scalar_project() -> String {
	let v = Point2(200.0, 20.0);
	let this = Point2(360.0, 120.0);
	let origin = Point2(40.0, 160.0);

	let p = origin + (this - origin).project(v - origin);

	// Calculate the right angle symbol
	let ra = (this - origin).resize(20.0);
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
	let mut svg = SvgWriter::new(400, 200);
	svg.arrow(origin..this, ARROW_SIZE).stroke("black");
	svg.arrow(origin..v, ARROW_SIZE).stroke("black");
	svg.circle(origin, 2.0).fill("black");
	svg.line(p..v).stroke("black").stroke_dasharray(&[5.0, 5.0]).stroke_width(0.5);
	svg.line(pra1..pra2).stroke("black").stroke_width(0.5);
	svg.line(pra2..pra3).stroke("black").stroke_width(0.5);
	svg.line(sl1..sl2).stroke("black").stroke_width(1.5);
	svg.line(sll1..sll2).stroke("black").stroke_width(1.5);
	svg.line(slr1..slr2).stroke("black").stroke_width(1.5);
	svg.text(v + Vec2(5.0, 5.0), "v").fill("black");
	svg.text(this + Vec2(-20.0, 22.0), "self").fill("black");
	svg.close()
}

//----------------------------------------------------------------
// Drawing for reflect

fn reflect_2d() -> String {
	// Calculate data
	let this = Vec2 { x: 10.0, y: 2.5 };
	let v = Vec2 { x: 4.0, y: 4.0 };
	let p = this.project(v);
	let pv = p - v;
	let result = p + pv;
	let origin = Vec2::origin();

	// Visualize data
	let transform = Affine2::translate((40.0f32, 120.0f32)) * Mat2::scale((25.0, -25.0));
	let this = transform * this;
	let v = transform * v;
	let p = transform * p;
	let pv = transform * pv;
	let result = transform * result;
	let origin = transform * origin;

	let mut svg = SvgWriter::new(400, 200);
	svg.line(v..result).stroke("black").stroke_width(0.5).stroke_dasharray(&[5.0, 5.0]);
	svg.line(p..pv).stroke("black").stroke_width(0.5).stroke_dasharray(&[5.0, 5.0]);
	svg.line(pv..result).stroke("black").stroke_width(0.5).stroke_dasharray(&[5.0, 5.0]);
	svg.arrow(origin..v, ARROW_SIZE).stroke("black");
	svg.arrow(origin..this, ARROW_SIZE).stroke("black");
	svg.arrow(origin..result, ARROW_SIZE).stroke("red");
	svg.circle(p, 2.0).fill("black");
	svg.text(v, "v").fill("black");
	svg.text(this, "self").fill("black");
	svg.text(p + Vec2(8.0, 10.0), "p").fill("black");
	svg.text(result, "result").fill("red");
	svg.text(p.lerp(pv, 0.9) + Vec2(-15.0, -5.0), "-v").fill("black");
	svg.text(pv.lerp(result, 0.8) + Vec2(0.0, 15.0), "+p").fill("black");
	svg.close()
}
