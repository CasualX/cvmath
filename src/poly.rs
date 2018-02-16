
use std::{cmp, mem, ops};
use std::marker::PhantomData;

use point::{Point2, Point3};
use line::{Line2, Line3};

#[derive(Debug)]
pub struct Polyline2<T>([Point2<T>]);
pub struct Polyline3<T>([Point3<T>]);

#[derive(Debug)]
pub struct Polygon2<T>([Point2<T>]);
pub struct Polygon3<T>([Point3<T>]);

macro_rules! poly {
	(
		$polyline:ident $iterline:ident
		$polygon:ident $itergon:ident
		$line:ident $point:ident
	) => {
		//----------------------------------------------------------------
		// Polyline

		impl<T> $polyline<T> {
			/// Closes the polyline creating a polygon.
			pub fn close(&self) -> &$polygon<T> {
				unsafe { mem::transmute(self) }
			}
			/// Returns an iterator over the line segments.
			pub fn lines(&self) -> $iterline<T> {
				$iterline { pts: self }
			}
		}
		impl<'a, T: 'a> From<&'a [$point<T>]> for &'a $polyline<T> {
			fn from(pts: &'a [$point<T>]) -> &'a $polyline<T> {
				unsafe { mem::transmute(pts) }
			}
		}
		impl<T> ops::Deref for $polyline<T> {
			type Target = [$point<T>];
			fn deref(&self) -> &[$point<T>] {
				&self.0
			}
		}

		//----------------------------------------------------------------
		// Iterline

		#[derive(Debug)]
		pub struct $iterline<'a, T: 'a> {
			pts: &'a [$point<T>],
		}
		impl<'a, T> Clone for $iterline<'a, T> {
			fn clone(&self) -> $iterline<'a, T> {
				$iterline { pts: self.pts }
			}
		}
		impl<'a, T: Copy> Iterator for $iterline<'a, T> {
			type Item = $line<T>;
			fn next(&mut self) -> Option<$line<T>> {
				if self.pts.len() < 2 {
					None
				}
				else {
					let line = $line(self.pts[0], self.pts[1]);
					self.pts = &self.pts[1..];
					Some(line)
				}
			}
			fn size_hint(&self) -> (usize, Option<usize>) {
				let len = cmp::max(1, self.pts.len()) - 1;
				(len, Some(len))
			}
			fn count(self) -> usize {
				cmp::max(1, self.pts.len()) - 1
			}
			fn last(self) -> Option<$line<T>> {
				let len = self.pts.len();
				if len < 2 {
					None
				}
				else {
					Some($line(self.pts[len - 2], self.pts[len - 1]))
				}
			}
		}
		impl<'a, T: Copy> ExactSizeIterator for $iterline<'a, T> {}
		impl<'a, T: Copy> DoubleEndedIterator for $iterline<'a, T> {
			fn next_back(&mut self) -> Option<$line<T>> {
				let len = self.pts.len();
				if len < 2 {
					None
				}
				else {
					let line = $line(self.pts[len - 2], self.pts[len - 1]);
					self.pts = &self.pts[..len - 1];
					Some(line)
				}
			}
		}

		//----------------------------------------------------------------
		// Polygon

		impl<T> $polygon<T> {
			pub fn open(&self) -> &$polyline<T> {
				unsafe { mem::transmute(self) }
			}
			pub fn lines(&self) -> $itergon<T> {
				let p = self.0.as_ptr();
				// I think this is technically UB...
				let last = unsafe { p.offset(self.0.len() as isize - 1) };
				$itergon {
					start: last,
					end: p,
					last,
					_marker: PhantomData,
				}
			}
		}
		impl<'a, T: 'a> From<&'a [$point<T>]> for &'a $polygon<T> {
			fn from(pts: &'a [$point<T>]) -> &'a $polygon<T> {
				unsafe { mem::transmute(pts) }
			}
		}
		impl<T> ops::Deref for $polygon<T> {
			type Target = [$point<T>];
			fn deref(&self) -> &[$point<T>] {
				&self.0
			}
		}

		//----------------------------------------------------------------
		// Itergon

		pub struct $itergon<'a, T: 'a> {
			start: *const $point<T>,
			end: *const $point<T>,
			last: *const $point<T>,
			_marker: PhantomData<&'a [$point<T>]>,
		}
		impl<'a, T> Clone for $itergon<'a, T> {
			fn clone(&self) -> $itergon<'a, T> {
				$itergon {
					start: self.start,
					end: self.end,
					last: self.last,
					_marker: self._marker,
				}
			}
		}
		impl<'a, T: Copy> Iterator for $itergon<'a, T> {
			type Item = $line<T>;
			fn next(&mut self) -> Option<$line<T>> {
				if self.start == self.last {
					None
				}
				else {
					let line = unsafe { $line(*self.start, *self.end) };
					self.start = self.end;
					self.end = unsafe { self.end.offset(1) };
					Some(line)
				}
			}
			fn size_hint(&self) -> (usize, Option<usize>) {
				let len = (self.last as usize - self.end as usize) / mem::size_of::<$point<T>>();
				(len, Some(len))
			}
			fn count(self) -> usize {
				self.size_hint().0
			}
		}
		impl<'a, T: Copy> ExactSizeIterator for $itergon<'a, T> {}

	};
}

poly!(Polyline2 Iterline2 Polygon2 Itergon2 Line2 Point2);
poly!(Polyline3 Iterline3 Polygon3 Itergon3 Line3 Point3);
