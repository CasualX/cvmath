extern crate cgm;

type Point2 = cgm::Point2<f64>;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Line {
	start: Point2,
	end: Point2,
}

enum Cut {
	Left(Line),
	Right(Line),
	Cut(Line, Line),
}

fn cut(line: Line, x: f64) -> Cut {
	if line.start.x <= x && line.end.x <= x {
		Cut::Left(line)
	}
	else if line.start.x >= x && line.end.x >= x {
		Cut::Right(line)
	}
	else {
		let ratio = (x - line.start.x) / (line.end.x - line.start.x);
		let y = line.start.y + (line.end.y - line.start.y) * ratio;
		Cut::Cut(Line { start: line.start, end: Point2::new(x, y) }, Line { start: Point2::new(x, y), end: line.end })
	}
}

#[test]
fn cut_it() {

}
