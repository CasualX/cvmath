#![allow(dead_code)]

use std::borrow::Borrow;
use super::*;

//----------------------------------------------------------------

pub struct SvgWriter(String);

impl SvgWriter {
	pub fn new(width: f32, height: f32) -> SvgWriter {
		SvgWriter(format!(r#"<svg width="{}" height="{}" font-family="monospace" xmlns="http://www.w3.org/2000/svg">"#, width, height))
	}
	pub fn circle(&mut self, circle: Circle<f32>) -> Attributes<'_, &'static str> {
		self.0 += &format!(r#"<circle cx="{}" cy="{}" r="{}""#, circle.center.x, circle.center.y, circle.radius);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn line(&mut self, line: Line2<f32>) -> Attributes<'_, &'static str> {
		self.0 += &format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}""#, line.start.x, line.start.y, line.end.x, line.end.y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn polyline(&mut self, pts: impl IntoIterator<Item = Point2<f32>>) -> Attributes<'_, &'static str> {
		self.0 += &format!(r#"<polyline points="{}""#,
			pts.into_iter().map(|p| format!("{:.1} {:.1}", p.x, p.y)).collect::<Vec<_>>().join(" "));
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn arrow(&mut self, start: Point2<f32>, end: Point2<f32>, arrowsize: f32) -> Attributes<'_, &'static str> {
		let unit = (end - start).resize(arrowsize);
		let p1 = end - unit + unit.cw() * 0.5;
		let p2 = end - unit + unit.ccw() * 0.5;
		self.0 += &format!(r#"<path fill="none" d="M{} {} L{} {} M{} {} L{} {} L{} {}""#,
			start.x, start.y, end.x, end.y, p1.x, p1.y, end.x, end.y, p2.x, p2.y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn triangle(&mut self, tri: Triangle2<f32>) -> Attributes<'_, &'static str> {
		self.0 += &format!(r#"<path d="M{} {} L{} {} L{} {} Z""#,
			tri.p1().x, tri.p1().y, tri.p2().x, tri.p2().y, tri.p3().x, tri.p3().y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn arc(&mut self, start: Point2<f32>, end: Point2<f32>, radius: f32) -> Attributes<'_, &'static str> {
		self.0 += &format!(r#"<path fill="none" d="M{} {} A{} {} 0 0 1 {} {}""#, start.x, start.y, radius, radius, end.x, end.y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn text(&mut self, p: Point2<f32>, text: &str) -> Attributes<'_, String> {
		self.0 += &format!(r#"<text x="{}" y="{}""#, p.x, p.y);
		Attributes { svg: &mut self.0, closing: format!(">{}</text>", text) }
	}
	pub fn rect(&mut self, rect: Bounds2<f32>) -> Attributes<'_, &'static str> {
		self.0 += &format!(r#"<rect x="{}" y="{}" width="{}" height="{}""#, rect.mins.x, rect.mins.y, rect.width(), rect.height());
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn close(mut self) -> String {
		self.0 += "</svg>"; self.0
	}
	pub fn save(self, path: &str) -> std::io::Result<()> {
		std::fs::write(path, self.close())
	}
}

//----------------------------------------------------------------

pub struct Attributes<'a, S: Borrow<str>> {
	svg: &'a mut String,
	closing: S,
}
impl<'a, S: Borrow<str>> Attributes<'a, S> {
	pub fn fill(&mut self, paint: &str) -> &mut Self {
		*self.svg += &format!(r#" fill="{}""#, paint); self
	}
	pub fn fill_opacity(&mut self, opacity: f32) -> &mut Self {
		*self.svg += &format!(r#" fill-opacity="{}""#, opacity); self
	}
	pub fn fill_rule(&mut self, fill_rule: &str) -> &mut Self {
		*self.svg += &format!(r#" fill-rule="{}""#, fill_rule); self
	}
	pub fn stroke(&mut self, paint: &str) -> &mut Self {
		*self.svg += &format!(r#" stroke="{}""#, paint); self
	}
	pub fn stroke_dasharray(&mut self, dasharray: &[f32]) -> &mut Self {
		let dasharray = format!("{:?}", dasharray);
		*self.svg += &format!(r#" stroke-dasharray="{}""#, &dasharray[1..dasharray.len() - 1]); self
	}
	pub fn stroke_dashoffset(&mut self, dashoffset: f32) -> &mut Self {
		*self.svg += &format!(r#" stroke-dashoffset="{}""#, dashoffset); self
	}
	pub fn stroke_linecap(&mut self, linecap: &str) -> &mut Self {
		*self.svg += &format!(r#" stroke-linecap="{}""#, linecap); self
	}
	pub fn stroke_linejoin(&mut self, linejoin: &str) -> &mut Self {
		*self.svg += &format!(r#" stroke-linejoin="{}""#, linejoin); self
	}
	pub fn stroke_miterlimit(&mut self, miterlimit: &str) -> &mut Self {
		*self.svg += &format!(r#" stroke-miterlimit="{}""#, miterlimit); self
	}
	pub fn stroke_opacity(&mut self, opacity: f32) -> &mut Self {
		*self.svg += &format!(r#" stroke-opacity="{}""#, opacity); self
	}
	pub fn stroke_width(&mut self, stroke_width: f32) -> &mut Self {
		*self.svg += &format!(r#" stroke-width="{}""#, stroke_width); self
	}
	pub fn font_family(&mut self, font_family: &str) -> &mut Self {
		*self.svg += &format!(r#" font-family="{}""#, font_family); self
	}
	pub fn font_size(&mut self, font_size: f32) -> &mut Self {
		*self.svg += &format!(r#" font-size="{}""#, font_size); self
	}
}
impl<'a, S: Borrow<str>> Drop for Attributes<'a, S> {
	fn drop(&mut self) {
		*self.svg += self.closing.borrow();
	}
}
