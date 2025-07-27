use cvmath::*;

struct Image {
	pixels: Vec<u8>,
	width: i32,
	height: i32,
}
impl Image {
	fn new(width: i32, height: i32) -> Image {
		let size = (width * height * 3) as usize;
		Image {
			pixels: vec![0; size],
			width,
			height,
		}
	}
	fn index(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || x >= self.width || y < 0 || y >= self.height {
			return None;
		}
		Some((y * self.width + x) as usize * 3)
	}
}

fn scene_save(path: &str, image: &Image) -> std::io::Result<()> {
	use std::fs::File;
	use std::io::{BufWriter, Write};
	let file = File::create(path)?;
	let mut writer = BufWriter::new(file);

	// Write P6 header
	writer.write_all(format!("P6\n{} {}\n255\n", image.width, image.height).as_bytes())?;

	// Write binary RGB data
	writer.write_all(&image.pixels)?;

	Ok(())
}

fn main() {
	let mut image = Image::new(800, 600);
	scene_save("raytracing.ppm", &image).expect("Failed to save image");
}
