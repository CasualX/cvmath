use super::*;

/// Point2 shape.
pub type Point2<T> = Vec2<T>;

/// Point2 constructor.
#[allow(non_snake_case)]
#[inline]
pub const fn Point2<T>(x: T, y: T) -> Point2<T> {
	Point2 { x, y }
}

specialized_type!(Point2, Point2f, f32, x, y);
specialized_type!(Point2, Point2d, f64, x, y);
specialized_type!(Point2, Point2i, i32, x, y);
