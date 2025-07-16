use std::{fmt, mem};

pub struct PrintMatrix {
	strings: String,
	indices: Vec<u32>,
	element_width: u32,
}

impl PrintMatrix {
	/// Adds a formattable element to print.
	pub fn push_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
		let start = self.strings.len();
		<String as fmt::Write>::write_fmt(&mut self.strings, args)?;
		let len = self.strings.len() - start;

		if start >= 0x1000000 || len >= 0x100 {
			return Err(fmt::Error);
		}
		self.element_width = u32::max(self.element_width, len as u32);
		self.indices.push((start << 8 | len) as u32);
		Ok(())
	}
}

pub fn print<'a>(get: &'a dyn Fn(usize) -> &'a dyn fmt::Display, dims: u32, f: &mut fmt::Formatter) -> fmt::Result {
	let rows = (dims >> 4) as usize;
	let cols = (dims & 0xf) as usize;
	let len = rows * cols;

	if f.alternate() {
		let mut printer = PrintMatrix {
			strings: String::new(),
			indices: Vec::with_capacity(len),
			element_width: 0,
		};
		for i in 0..len {
			printer.push_fmt(format_args!("{}", get(i)))?;
		}
		let mut i = 0;
		for row in 0..rows {
			f.write_str(if row != 0 { "],\n [" } else { "\n [" })?;
			for col in 0..cols {
				if col != 0 {
					f.write_str(", ")?;
				}

				let index = printer.indices[i];
				let start = (index >> 8) as usize;
				let end = start + (index & 0xff) as usize;
				let value = &printer.strings[start..end];

				write!(f, "{: >1$}", value, printer.element_width as usize)?;
				i += 1;
			}
		}
		f.write_str("]")?;
	}
	else {
		let mut i = 0;
		for row in 0..rows {
			f.write_str(if row != 0 { "], [" } else { "[" })?;
			for col in 0..cols {
				if col != 0 {
					f.write_str(", ")?;
				}
				get(i).fmt(f)?;
				i += 1;
			}
		}
		f.write_str("]")?;
	}

	Ok(())
}

#[repr(transparent)]
pub struct Debug<T> { value: T }
#[inline]
#[allow(non_snake_case)]
pub fn Debug<T>(value: &T) -> &Debug<T> {
	unsafe { mem::transmute(value) }
}
impl<T: fmt::Debug> fmt::Display for Debug<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&self.value, f)
	}
}
