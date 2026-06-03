use super::*;
use std::mem::MaybeUninit;

struct InitGuard<'a, T> {
	values: &'a mut [MaybeUninit<T>],
	initialized: usize,
}

impl<T> InitGuard<'_, T> {
	#[inline]
	fn len(&self) -> usize {
		self.values.len()
	}

	#[inline]
	fn push(&mut self, value: T) {
		self.values[self.initialized].write(value);
		self.initialized += 1;
	}

	#[inline]
	fn finish(self) {}
}

impl<T> Drop for InitGuard<'_, T> {
	fn drop(&mut self) {
		for value in &mut self.values[..self.initialized] {
			unsafe { value.assume_init_drop() };
		}
	}
}

/// An error which can be returned when parsing a matrix type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseMatrixError<E> {
	/// Matrix syntax does not match the expected bracketed row layout.
	SyntaxError,
	/// The number of parsed elements does not match the matrix dimensions.
	DimMismatch,
	/// Error parsing one of the matrix elements.
	ParseValue(E),
}
impl<E> From<E> for ParseMatrixError<E> {
	#[inline]
	fn from(err: E) -> ParseMatrixError<E> {
		ParseMatrixError::ParseValue(err)
	}
}
impl<E: Error + 'static> fmt::Display for ParseMatrixError<E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#[allow(deprecated)]
		self.description().fmt(f)
	}
}
impl<E: Error + 'static> Error for ParseMatrixError<E> {
	fn description(&self) -> &str {
		#[allow(deprecated)]
		match self {
			ParseMatrixError::SyntaxError => "syntax error",
			ParseMatrixError::DimMismatch => "dim mismatch",
			ParseMatrixError::ParseValue(inner) => inner.description(),
		}
	}
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ParseMatrixError::SyntaxError => None,
			ParseMatrixError::DimMismatch => None,
			ParseMatrixError::ParseValue(inner) => Some(inner),
		}
	}
}

fn parse_matrix_into<'a, T: FromStr>(s: &str, name: &str, dims: u32, values: &'a mut [MaybeUninit<T>]) -> Result<(), ParseMatrixError<T::Err>> {
	let rows = (dims >> 4) as usize;
	let cols = (dims & 0xf) as usize;
	if values.len() != rows * cols {
		return Err(ParseMatrixError::DimMismatch);
	}
	let mut guard = InitGuard { values, initialized: 0 };

	let Some(s) = s.strip_prefix(name) else {
		return Err(ParseMatrixError::SyntaxError);
	};
	let Some(s) = s.strip_prefix('(') else {
		return Err(ParseMatrixError::SyntaxError);
	};
	let Some(s) = s.strip_suffix(')') else {
		return Err(ParseMatrixError::SyntaxError);
	};

	let s = s.trim_ascii();
	if let Some(mut rest) = s.strip_prefix('[') {
		for row in 0..rows {
			let Some(end) = rest.find(']') else {
				return Err(ParseMatrixError::SyntaxError);
			};
			let row_values = &rest[..end];
			rest = &rest[end + 1..];

			let mut elems = row_values.split(',');
			for _ in 0..cols {
				let Some(elem) = elems.next() else {
					return Err(ParseMatrixError::DimMismatch);
				};
				match elem.trim_ascii().parse() {
					Ok(value) => guard.push(value),
					Err(err) => return Err(ParseMatrixError::ParseValue(err)),
				}
			}
			if elems.next().is_some() {
				return Err(ParseMatrixError::DimMismatch);
			}

			if row + 1 < rows {
				rest = rest.trim_ascii_start();
				let Some(s) = rest.strip_prefix(',') else {
					return Err(ParseMatrixError::SyntaxError);
				};
				rest = s.trim_ascii_start();
				let Some(s) = rest.strip_prefix('[') else {
					return Err(ParseMatrixError::SyntaxError);
				};
				rest = s;
			}
		}

		if !rest.is_empty() {
			return Err(ParseMatrixError::SyntaxError);
		}
	}
	else {
		let mut elems = s.split(',');
		for _ in 0..guard.len() {
			let Some(elem) = elems.next() else {
				return Err(ParseMatrixError::DimMismatch);
			};
			match elem.trim_ascii().parse() {
				Ok(value) => guard.push(value),
				Err(err) => return Err(ParseMatrixError::ParseValue(err)),
			}
		}
		if elems.next().is_some() {
			return Err(ParseMatrixError::DimMismatch);
		}
	}

	guard.finish();
	Ok(())
}

#[inline]
pub fn parse_matrix<T: FromStr, const N: usize>(s: &str, name: &str, dims: u32) -> Result<[T; N], ParseMatrixError<T::Err>> {
	let mut values = [const { MaybeUninit::<T>::uninit() }; N];
	parse_matrix_into(s, name, dims, &mut values)?;
	Ok(values.map(|value| unsafe { value.assume_init() }))
}
