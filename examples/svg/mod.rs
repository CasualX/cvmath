/*!
Hacked together SVG writer.
*/

use ::std::borrow::Borrow;

use ::cvmath::prelude::Point2;
type Line2<T> = ::std::ops::Range<Point2<T>>;

//----------------------------------------------------------------

pub struct SvgWriter(String);

impl SvgWriter {
	pub fn new(width: i32, height: i32) -> SvgWriter {
		SvgWriter(format!(r#"<svg width="{}" height="{}" font-family="monospace" xmlns="http://www.w3.org/2000/svg">"#, width, height))
	}
	pub fn circle(&mut self, center: Point2<f32>, radius: f32) -> Attributes<&'static str> {
		self.0 += &format!(r#"<circle cx="{}" cy="{}" r="{}""#, center.x, center.y, radius);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn line(&mut self, line: Line2<f32>) -> Attributes<&'static str> {
		self.0 += &format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}""#, line.start.x, line.start.y, line.end.x, line.end.y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn arrow(&mut self, line: Line2<f32>, arrowsize: f32) -> Attributes<&'static str> {
		let unit = (line.end - line.start).resize(arrowsize);
		let p1 = line.end - unit + unit.cw() * 0.5;
		let p2 = line.end - unit + unit.ccw() * 0.5;
		self.0 += &format!(r#"<path fill="none" d="M{} {} L{} {} M{} {} L{} {} L{} {}""#,
			line.start.x, line.start.y, line.end.x, line.end.y, p1.x, p1.y, line.end.x, line.end.y, p2.x, p2.y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn arc(&mut self, start: Point2<f32>, end: Point2<f32>, radius: f32) -> Attributes<&'static str> {
		self.0 += &format!(r#"<path fill="none" d="M{} {} A{} {} 0 0 1 {} {}""#,
			start.x, start.y, radius, radius, end.x, end.y);
		Attributes { svg: &mut self.0, closing: " />" }
	}
	pub fn text(&mut self, p: Point2<f32>, text: &str) -> Attributes<String> {
		self.0 += &format!(r#"<text x="{}" y="{}""#, p.x, p.y);
		Attributes { svg: &mut self.0, closing: format!(">{}</text>", text) }
	}
	pub fn close(mut self) -> String {
		self.0 += "</svg>"; self.0
	}
}

//----------------------------------------------------------------

pub struct Attributes<'a, S: ?Sized + Borrow<str>> {
	svg: &'a mut String,
	closing: S,
}
impl<'a, S: ?Sized + Borrow<str>> Attributes<'a, S> {
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
impl<'a, S: ?Sized + Borrow<str>> Drop for Attributes<'a, S> {
	fn drop(&mut self) {
		*self.svg += self.closing.borrow();
	}
}
