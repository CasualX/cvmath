/*!
Generates the SVG for the documentation and serves as a simple example.
*/

#![allow(dead_code)]

extern crate cvmath;
use cvmath::prelude::*;

type Line2<T> = ::std::ops::Range<Point2<T>>;

fn main() {
	println!("LERP : {}", lerp());
	println!("SLERP: {}", slerp());
	println!("NLERP: {}", nlerp());
}

//----------------------------------------------------------------

struct SVG {
	svg: String,
}
impl SVG {
	pub fn new(width: i32, height: i32) -> SVG {
		SVG { svg: format!(r#"<svg width="{}" height="{}" font-family="monospace" xmlns="http://www.w3.org/2000/svg">"#, width, height) }
	}
	pub fn line(&mut self, line: Line2<f32>, width: f32, color: &str) {
		self.svg += &format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke-width="{}" stroke="{}" />"#, line.start.x, line.start.y, line.end.x, line.end.y, width, color);
	}
	pub fn dashed(&mut self, line: Line2<f32>, dash_len: f32, num: i32, width: f32, color: &str) {
		let step = (line.end - line.start) / num as f32;
		let offset = dash_len / step.len();
		let end = line.end + (line.end - line.start).resize(step.len() - dash_len);
		let step = (end - line.start) / num as f32;
		for i in 0..num {
			let i = i as f32;
			self.line(line.start + step * i..line.start + step * (i + offset), width, color);
		}
	}
	pub fn arrow(&mut self, line: Line2<f32>, arrow_size: f32, width: f32, color: &str) {
		let unit = (line.end - line.start).resize(arrow_size);
		let p1 = (line.end - unit) + unit.cw() * 0.5;
		let p2 = (line.end - unit) + unit.ccw() * 0.5;
		self.svg += &format!(r#"<path d="M{} {} L{} {} M{} {} L{} {} L{} {}" stroke-width="{}" stroke="{}" fill="none" />"#,
			line.start.x, line.start.y, line.end.x, line.end.y, p1.x, p1.y, line.end.x, line.end.y, p2.x, p2.y, width, color);
	}
	pub fn circle(&mut self, c: Point2<f32>, r: f32, color: &str) {
		self.svg += &format!(r#"<circle cx="{}" cy="{}" r="{}" fill="{}" />"#, c.x, c.y, r, color);
	}
	pub fn arc(&mut self, start: Point2<f32>, end: Point2<f32>, radius: f32, width: f32, color: &str) {
		self.svg += &format!(r#"<path d="M{} {} A{} {} 0 0 1 {} {}" stroke-width="{}" stroke="{}" fill="none" />"#, start.x, start.y, radius, radius, end.x, end.y, width, color);
	}
	pub fn text(&mut self, p: Point2<f32>, text: &str, color: &str) {
		self.svg += &format!(r#"<text x="{}" y="{}" fill="{}">{}</text>"#, p.x, p.y, color, text);
	}
	pub fn close(self) -> String {
		self.svg + r#"</svg>"#
	}
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

	let mut svg = SVG::new(400, 120);
	svg.line(v1..vgreen, 1.0, "green");
	svg.line(vgreen..vblue, 1.0, "blue");
	svg.line(vblue..v2, 1.0, "black");
	svg.circle(v1, 2.0, "black");
	svg.circle(v2, 2.0, "black");
	svg.circle(vgreen, 2.0, "green");
	svg.circle(vblue, 2.0, "blue");
	svg.text(v1 - Point2(20.0, 10.0), "self", "black");
	svg.text(v2 - Point2(15.0, -20.0), "rhs", "black");
	svg.text(vgreen - Point2(20.0, -20.0), "t = 0.2", "green");
	svg.text(vblue - Point2(20.0, -20.0), "t = 0.5", "blue");
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
	let mut svg = SVG::new(400, 140);
	svg.arrow(center..v1, 5.0, 0.5, "black");
	svg.line(center..v2, 0.5, "black");
	svg.arrow(center..p1, 5.0, 0.25, "green");
	svg.arrow(center..p2, 5.0, 0.25, "green");
	svg.arrow(center..slerp, 5.0, 0.5, "green");
	svg.arc(cstart, v1, radius, 0.5, "black");
	svg.arc(v1, slerp, radius, 1.0, "green");
	svg.arc(slerp, v2, radius, 1.0, "black");
	svg.arc(v2, cend, radius, 0.5, "black");
	svg.line(v1..lerp, 0.5, "blue");
	svg.circle(v1, 2.0, "black");
	svg.circle(v2, 2.0, "black");
	svg.circle(lerp, 2.0, "blue");
	svg.circle(slerp, 2.0, "green");
	svg.text(p1 - Point2(45.0, 5.0), "t = 0.25", "green\" font-size=\"10");
	svg.text(p2 - Point2(20.0, 5.0), "t = 0.50", "green\" font-size=\"10");
	svg.text(slerp - Point2(0.0, 5.0), "t = 0.75", "green\" font-size=\"10");
	svg.text(lerp - Point2(20.0, -20.0), "lerp", "blue");
	svg.text(slerp - Point2(60.0, -10.0), name, "green");
	svg.text(v1 - Point2(50.0, 0.0), "self", "black");
	svg.text(v2 - Point2(-10.0, 0.0), "rhs", "black");
	svg.close()
}

fn slerp() -> String {
	slerp_nlerp(Point2::slerp, "slerp")
}
fn nlerp() -> String {
	slerp_nlerp(Point2::nlerp, "nlerp")
}
