use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

use segment::{Point, Segment};

mod segment;

fn read_input(path: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    let mut second_line = String::new();

    let _ = reader.read_line(&mut first_line);
    let _ = reader.read_line(&mut second_line);

    let first_path = first_line.split(",").map(|s| String::from(s.trim())).collect::<Vec<String>>();
    let second_path = second_line.split(",").map(|s| String::from(s.trim())).collect::<Vec<String>>();

    (first_path, second_path)
}

fn create_segments_from_path(path: Vec<String>) -> Vec<Segment> {
    let mut crt_start_point = Point::new(0, 0);
    let mut segments = Vec::<Segment>::new();
    for sub_path in &path {
        let segment = Segment::new(crt_start_point, sub_path);
        crt_start_point = segment.get_end_point();
        segments.push(segment);
    }

    segments
}

fn get_path_intersections(path_a: &Vec<Segment>, path_b: &Vec<Segment>) -> Vec<Point> {
    let mut intersections = Vec::<Point>::new();

    for seg_a in path_a {
        for seg_b in path_b {
            intersections.append(&mut seg_a.get_intersections_with(seg_b));
        }
    }

    intersections.remove(0);
    intersections
}

fn get_distance_to_closest_intersection(intersections: &Vec<Point>, point: Point) -> i32 {
    let mut min_distance = std::i32::MAX;

    for intersection in intersections {
        min_distance = cmp::min(min_distance, point.manhattan_distance_to(&intersection));
    }

    min_distance
}

fn get_steps_to_point(path: &Vec<Segment>, point: &Point) -> Option<i32> {
    let mut step_count = 0;
    for segment in path {
        if segment.contains_point(point) {
            step_count += segment.steps_to(point);
            return Some(step_count);
        } else {
            step_count += segment.get_length();
        }
    }

    return None;
}

fn main() {
    let (first_path, second_path) = read_input("src/Day 3/wires.in");

    let first_path_segments = create_segments_from_path(first_path);
    let second_path_segments = create_segments_from_path(second_path);

    let intersections = get_path_intersections(&first_path_segments, &second_path_segments);

    println!("Closest intersection: {}", get_distance_to_closest_intersection(&intersections, Point::new(0, 0)));

    let steps_to_intersection = |point: &Point| -> i32 {
        get_steps_to_point(&first_path_segments, point).unwrap() +
            get_steps_to_point(&second_path_segments, point).unwrap()
    };

    let min_step_count = intersections
        .iter()
        .map(steps_to_intersection)
        .fold(std::i32::MAX, |acc, count| cmp::min(acc, count));
    print!("Minimum step count to intersection: {}", min_step_count);
}
