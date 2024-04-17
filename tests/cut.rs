type Point2 = cvmath::Point2<f64>;
type Line2 = cvmath::Line2<f64>;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cut {
	Left,
	Right,
	Cut(Point2),
}

fn cut(line: Line2, x: f64) -> Cut {
	if line.start.x <= x && line.end.x <= x {
		Cut::Left
	}
	else if line.start.x >= x && line.end.x >= x {
		Cut::Right
	}
	else {
		let ratio = (x - line.start.x) / (line.end.x - line.start.x);
		let y = line.start.y + (line.end.y - line.start.y) * ratio;
		Cut::Cut(Point2::new(x, y))
	}
}

#[test]
fn cut_it() {
	let cut = cut(Line2 { start: Point2::new(0.0, 0.0), end: Point2::new(2.0, 2.0) }, 1.0);
	assert_eq!(cut, Cut::Cut(Point2::new(1.0, 1.0)));
}
