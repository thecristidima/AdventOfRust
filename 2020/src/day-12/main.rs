use utils::files::read_lines_iter;

#[derive(Clone, Copy, PartialEq)]
enum Orientation {
    North,
    South,
    East,
    West,
}

enum Command {
    Forward,
    Move,
    Rotate,
}

fn parse_command(command: &str) -> (Command, i32) {
    let arg = command[1..].parse::<i32>().unwrap();

    match command.chars().nth(0).unwrap() {
        'F' => (Command::Forward, arg),
        'N' | 'E' | 'S' | 'W' => (Command::Move, arg),
        'L' | 'R' => (Command::Rotate, arg),
        _ => panic!("Invalid command"),
    }
}

fn move_by_orientation(orientation: Orientation, distance: i32, coordinates: &mut (i32, i32)) {
    match orientation {
        Orientation::North => coordinates.0 -= distance,
        Orientation::South => coordinates.0 += distance,
        Orientation::East => coordinates.1 += distance,
        Orientation::West => coordinates.1 -= distance,
    }
}

fn move_to_waypoint(
    ship_coordinates: &mut (i32, i32),
    waypoint_coordinates: &(i32, i32),
    times: i32,
) {
    ship_coordinates.0 += waypoint_coordinates.0 * times;
    ship_coordinates.1 += waypoint_coordinates.1 * times;
}

fn char_to_orientation(c: char) -> Orientation {
    match c {
        'N' => Orientation::North,
        'S' => Orientation::South,
        'E' => Orientation::East,
        'W' => Orientation::West,
        _ => panic!("Invalid orientation"),
    }
}

fn rotate(orientation: &mut Orientation, rotation_direction: char, degrees: i32) {
    let orientations = vec![
        Orientation::East,
        Orientation::South,
        Orientation::West,
        Orientation::North,
    ];

    let crt_orientation_idx = orientations.iter().position(|o| o == orientation).unwrap();

    match rotation_direction {
        'L' => *orientation = orientations[(crt_orientation_idx + 4 - (degrees / 90) as usize) % 4],
        'R' => *orientation = orientations[(crt_orientation_idx + (degrees / 90) as usize) % 4],
        _ => panic!("Invalid rotation direction"),
    }
}

fn rotate_waypoint(coordinates: &mut (i32, i32), direction: char, degrees: i32) {
    let times_to_rotate = degrees / 90;

    match direction {
        'R' => {
            for _ in 0..times_to_rotate {
                let aux = coordinates.0;
                coordinates.0 = coordinates.1;
                coordinates.1 = -aux;
            }
        }
        'L' => {
            for _ in 0..times_to_rotate {
                let aux = coordinates.1;
                coordinates.1 = coordinates.0;
                coordinates.0 = -aux;
            }
        }
        _ => panic!("Invalid rotation direction"),
    }
}

/**
 * Your ship moves based on some commands.
 * 
 * Offtopic: Both parts are extremely boring, probably the worst ones so far.
 *
 * Part 1:
 *  Your ship starts at (0, 0) facing east. The commands are as follows:
 *      Fn       -> Move forward (depending on orientation) n spots
 * 	    L/Rn     -> Rotate ship n degrees (90-180-270) to the left or right
 *      N/E/S/Wn -> Move north/east/south/west n spots (ignore orientation)
 *
 *  What is the Manhattan distance to the centre of the map?
 *
 * Part 2:
 *  Your ship starts at (0, 0). You have a waypoint for orientation.
 *  This waypoint is always relative to your position and starts at (-1, 10).
 *  The commands are as follows:
 *      Fn       -> Move towards waypoint n times (ship_coords += way_coords * n)
 *      L/Rn     -> Rotate waypoint n degrees to the left or right
 *      N/E/S/Wn -> Move waypoint north/east/south/west n spots
 *
 */
fn main() {
    let mut orientation = Orientation::East;
    let mut coordinates = (0, 0); // (Vertical, Horizontal)

    let mut ship_coordinates = (0, 0);
    let mut waypoint_coordinates = (-1, 10);

    for line in read_lines_iter("src/day-12/input.txt") {
        let line = line.unwrap();

        let first_char = line.chars().next().unwrap();

        let (cmd, arg) = parse_command(&line);

        // Part 1
        match cmd {
            Command::Forward => move_by_orientation(orientation, arg, &mut coordinates),
            Command::Move => {
                move_by_orientation(char_to_orientation(first_char), arg, &mut coordinates)
            }
            Command::Rotate => rotate(&mut orientation, first_char, arg),
        }

        // Part 2
        match cmd {
            Command::Forward => move_to_waypoint(&mut ship_coordinates, &waypoint_coordinates, arg),
            Command::Move => move_by_orientation(
                char_to_orientation(first_char),
                arg,
                &mut waypoint_coordinates,
            ),
            Command::Rotate => rotate_waypoint(&mut waypoint_coordinates, first_char, arg),
        }
    }

    println!(
        "Solution for part 1: {}",
        coordinates.0.abs() + coordinates.1.abs()
    ); // 796

    println!(
        "Solution for part 2: {}",
        ship_coordinates.0.abs() + ship_coordinates.1.abs()
    ); // 39446
}
