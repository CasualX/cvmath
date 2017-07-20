/*!
Generates the SVG for the documentation and serves as a simple example.
*/

#![allow(dead_code)]

extern crate cvmath;
use cvmath::prelude::*;
type Line2<T> = ::std::ops::Range<Point2<T>>;

mod svg;
use self::svg::SvgWriter;

//----------------------------------------------------------------

fn main() {
	write_svg("src/vec.rs:LERP", &lerp());
	write_svg("src/vec.rs:SLERP", &slerp());
	write_svg("src/vec.rs:NLERP", &nlerp());
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

	let s = 0.75;

	// Calculate lerp
	let lerp = v1.lerp(v2, s);

	// Calculate slerps
	let leg1 = v1 - center;
	let leg2 = v2 - center;
	let slerp = center + f(leg1, leg2, s);
	let p1 = center + f(leg1, leg2, 0.25);
	let p2 = center + f(leg1, leg2, 0.5);
	let cstart = center + f(leg1, leg2, -0.1);
	let cend = center + f(leg1, leg2, 1.1);

	// Render time
	let mut svg = SvgWriter::new(400, 140);
	svg.arrow(center..v1, 8.0).stroke("black").stroke_width(0.5);
	svg.arrow(center..v2, 8.0).stroke("black").stroke_width(0.5);
	svg.arrow(center..p1, 8.0).stroke("green").stroke_width(0.25);
	svg.arrow(center..p2, 8.0).stroke("green").stroke_width(0.25);
	svg.arrow(center..slerp, 8.0).stroke("green");
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
