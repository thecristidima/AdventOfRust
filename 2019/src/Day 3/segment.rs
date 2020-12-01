use std::cmp;
use std::iter::Iterator;

#[derive(PartialEq, Copy, Clone)]
pub enum SegmentOrientation {
	Vertical,
	Horizontal,
}

#[derive(Copy, Clone)]
pub struct Point {
	x: i32,
	y: i32,
}

impl Point {
	pub fn new(x: i32, y: i32) -> Point {
		Point { x, y }
	}

	pub fn manhattan_distance_to(&self, other: &Self) -> i32 {
		(self.x - other.x).abs() + (self.y - other.y).abs()
	}
}

pub struct Segment {
	start: Point,
	end: Point,
	orientation: SegmentOrientation,
}

impl Segment {
	pub fn new(start: Point, sub_path: &String) -> Segment {
		let direction = sub_path.chars().nth(0).unwrap();
		let step_count = sub_path.chars().as_str()[1..].parse::<i32>().unwrap();

		match direction {
			'U' => Segment {
				start,
				end: Point { x: start.x, y: start.y - step_count },
				orientation: SegmentOrientation::Vertical,
			},
			'D' => Segment {
				start,
				end: Point { x: start.x, y: start.y + step_count },
				orientation: SegmentOrientation::Vertical,
			},
			'L' => Segment {
				start,
				end: Point { x: start.x - step_count, y: start.y },
				orientation: SegmentOrientation::Horizontal,
			},
			'R' => Segment {
				start,
				end: Point { x: start.x + step_count, y: start.y },
				orientation: SegmentOrientation::Horizontal,
			},
			_ => panic!("Unknown direction")
		}
	}

	pub fn get_end_point(&self) -> Point {
		self.end.clone()
	}

	pub fn get_intersections_with(&self, other: &Self) -> Vec<Point> {
		if self.orientation == other.orientation {
			match self.orientation {
				SegmentOrientation::Vertical => return Segment::get_vertical_intersection(&self, &other),
				SegmentOrientation::Horizontal => return Segment::get_horizontal_intersection(&self, &other)
			}
		}

		match (self.orientation, other.orientation) {
			(SegmentOrientation::Vertical, SegmentOrientation::Horizontal) =>
				Segment::get_cross_intersection(self, other),
			(SegmentOrientation::Horizontal, SegmentOrientation::Vertical) =>
				Segment::get_cross_intersection(other, self),
			_ => panic!("Can't get here, but Rust is a bit of a dum-dum")
		}
	}

	fn get_vertical_intersection(a: &Self, b: &Self) -> Vec<Point> {
		let mut intersection_points = Vec::<Point>::new();

		if a.start.x != b.start.x {
			return intersection_points;
		}

		let lower_a = cmp::max(a.start.y, a.end.y);
		let upper_a = cmp::min(a.start.y, a.end.y);
		let lower_b = cmp::max(b.start.y, b.end.y);
		let upper_b = cmp::min(b.start.y, b.end.y);

		// no intersection
		if lower_a < upper_b || upper_a > lower_b {
			return intersection_points;
		}

		// if intersection is in the lower part of b
		if upper_a > upper_b && lower_a > lower_b {
			for y in upper_a..=lower_b {
				intersection_points.push(Point::new(a.start.x, y));
			}
			return intersection_points;
		}

		// b is in the lower part of a
		if upper_b > upper_a && lower_b > lower_a {
			for y in upper_b..=lower_a {
				intersection_points.push(Point::new(a.start.x, y));
			}
			return intersection_points;
		}

		// one segment is part of the other
		let upper = cmp::max(upper_a, upper_b);
		let lower = cmp::min(lower_a, lower_b);
		for y in upper..=lower {
			intersection_points.push(Point::new(a.start.x, y));
		}

		intersection_points
	}

	fn get_horizontal_intersection(a: &Self, b: &Self) -> Vec<Point> {
		let mut intersection_points = Vec::<Point>::new();

		if a.start.y != b.start.y {
			return intersection_points;
		}

		let lower_a = cmp::max(a.start.x, a.end.x);
		let upper_a = cmp::min(a.start.x, a.end.x);
		let lower_b = cmp::max(b.start.x, b.end.x);
		let upper_b = cmp::min(b.start.x, b.end.x);

		// no intersection
		if lower_a < upper_b || upper_a > lower_b {
			return intersection_points;
		}

		// if intersection is in the lower part of b
		if upper_a > upper_b && lower_a > lower_b {
			for x in upper_a..=lower_b {
				intersection_points.push(Point::new(x, a.start.y));
			}
			return intersection_points;
		}

		// b is in the lower part of a
		if upper_b > upper_a && lower_b > lower_a {
			for x in upper_b..=lower_a {
				intersection_points.push(Point::new(x, a.start.y));
			}
			return intersection_points;
		}

		// one segment is part of the other
		let upper = cmp::max(upper_a, upper_b);
		let lower = cmp::min(lower_a, lower_b);
		for x in upper..=lower {
			intersection_points.push(Point::new(x, a.start.y));
		}

		intersection_points
	}

	fn get_cross_intersection(vertical: &Self, horizontal: &Self) -> Vec<Point> {
		let mut intersection_points = Vec::<Point>::new();

		let vert_x = vertical.start.x;
		let hor_y = horizontal.start.y;

		if vert_x < horizontal.start.x && vert_x < horizontal.end.x
			|| vert_x > horizontal.start.x && vert_x > horizontal.end.x {
			return intersection_points;
		}

		if hor_y < vertical.start.y && hor_y < vertical.end.y
			|| hor_y > vertical.start.y && hor_y > vertical.end.y {
			return intersection_points;
		}

		intersection_points.push(Point::new(vert_x, hor_y));
		intersection_points
	}

	pub fn contains_point(&self, point: &Point) -> bool {
		match self.orientation {
			SegmentOrientation::Vertical =>
				point.x == self.start.x
					&& (point.y > self.start.y && point.y < self.end.y
					|| point.y < self.start.y && point.y > self.end.y),
			SegmentOrientation::Horizontal =>
				point.y == self.start.y
					&& (point.x > self.start.x && point.x < self.end.x
					|| point.x < self.start.x && point.x > self.end.x)
		}
	}

	pub fn get_length(&self) -> i32 {
		match self.orientation {
			SegmentOrientation::Horizontal => (self.start.x - self.end.x).abs(),
			SegmentOrientation::Vertical => (self.start.y - self.end.y).abs()
		}
	}

	pub fn steps_to(&self, point: &Point) -> i32 {
		match self.orientation {
			SegmentOrientation::Horizontal => (point.x - self.start.x).abs(),
			SegmentOrientation::Vertical => (point.y - self.start.y).abs()
		}
	}
}
