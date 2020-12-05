use regex::Regex;
use utils::files::read_lines_iter;

/**
 * You got on the plane, but lost your boarding pass. A boarding pass looks like this:
 *
 *     [FB]+[LR]{3}
 *
 * The first part divides the seat row (front half or back half), while the second one
 * divides the seat column (the plane has a fixed width of 8).
 *
 * For example, FFBF means there are 16 rows and you get the row as follows:
 * 	F - first half of 0-15, so 0-7
 *  F - first half of 0-7, so 0-3
 *  B - second half of 0-3, so 1-2
 *  F - first half of 1-2, so 1
 *
 * The same thing goes for columns (L - first half, R - second half).
 * The seat id is row * 8 (number of columns) + col.
 *
 * Part 1:
 *  What's the biggest seat id?
 *
 * Part 2:
 *  The airplane is full, so what is your seat id?
 *  Note: The first and last few seats aren't used in the plane, so your seat
 *        is, actually, somewhere in the middle.
 */
fn main() {
    let re = Regex::new("(?P<rows>[FB]+)(?P<cols>[LR]{3})").expect("regex is valid");

    let mut max_seat_id: u32 = 0;
    let mut seat_ids = Vec::<u32>::new();

    for line in read_lines_iter("src/day-5/input.txt") {
        let line = line.expect("can read line");

        let capture = re.captures(&line).unwrap();

        let row_as_str = capture
            .name("rows")
            .unwrap()
            .as_str()
            .replace("F", "0")
            .replace("B", "1");

        let col_as_str = capture
            .name("cols")
            .unwrap()
            .as_str()
            .replace("L", "0")
            .replace("R", "1");

        let row = u32::from_str_radix(&row_as_str, 2).unwrap();
        let col = u32::from_str_radix(&col_as_str, 2).unwrap();

        let seat_id = row * 8 + col;
        seat_ids.push(seat_id);

        max_seat_id = max_seat_id.max(seat_id);
    }

    println!("Part 1 solution: {}", max_seat_id); // 858

    seat_ids.sort();
    for (idx, _) in seat_ids.iter().enumerate() {
        if seat_ids[idx + 1] != seat_ids[idx] + 1 {
            println!("Part 2 solution {}:", seat_ids[idx] + 1); // 557
            break;
        }
    }
}
