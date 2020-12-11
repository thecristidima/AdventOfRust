use utils::files::read_lines_iter;

#[derive(Clone, Debug, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

type Room = Vec<Vec<Seat>>;

fn get_input(path: &str) -> Room {
    let mut room = Room::new();

    for line in read_lines_iter(path) {
        let line = line.unwrap();

        let mut row = Vec::<Seat>::new();

        for c in line.chars() {
            match c {
                'L' => row.push(Seat::Empty),
                '#' => row.push(Seat::Occupied),
                '.' => row.push(Seat::Floor),
                _ => panic!("this shouldn't happen {}", c),
            }
        }

        room.push(row);
    }

    room
}

fn get_seat_neighbours_one(room: &Room, row: usize, col: usize) -> u32 {
    let mut neighbours: u32 = 0;

    // NW
    if row != 0 && col != 0 {
        neighbours += if room[row - 1][col - 1] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // N
    if row != 0 {
        neighbours += if room[row - 1][col] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // NE
    if row != 0 && col != room[0].len() - 1 {
        neighbours += if room[row - 1][col + 1] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // W
    if col != 0 {
        neighbours += if room[row][col - 1] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // E
    if col != room[0].len() - 1 {
        neighbours += if room[row][col + 1] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // SW
    if row != room.len() - 1 && col != 0 {
        neighbours += if room[row + 1][col - 1] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // S
    if row != room.len() - 1 {
        neighbours += if room[row + 1][col] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    // SE
    if row != room.len() - 1 && col != room[0].len() - 1 {
        neighbours += if room[row + 1][col + 1] == Seat::Occupied {
            1
        } else {
            0
        };
    }

    neighbours
}

fn part_one(room: &mut Room) -> usize {
    let mut room_changed: bool;

    loop {
        room_changed = false;

        let mut next_room = room.clone();

        for row in 0..room.len() {
            for col in 0..room[0].len() {
                match room[row][col] {
                    Seat::Floor => continue,
                    Seat::Empty => {
                        let neighbours = get_seat_neighbours_one(room, row, col);
                        if neighbours == 0 {
                            next_room[row][col] = Seat::Occupied;
                            room_changed = true;
                        }
                    }
                    Seat::Occupied => {
                        let neighbours = get_seat_neighbours_one(room, row, col);
                        if neighbours >= 4 {
                            next_room[row][col] = Seat::Empty;
                            room_changed = true;
                        }
                    }
                }
            }
        }

        if !room_changed {
            break;
        }

        *room = next_room;
    }

    room.into_iter()
        .flatten()
        .filter(|seat| **seat == Seat::Occupied)
        .count()
}

fn get_seat_neighbours_two(room: &Room, row: usize, col: usize) -> u32 {
    let mut neighbours: u32 = 0;

    let mut crt_row: isize;
    let mut crt_col: isize;

    // NW
    if row != 0 && col != 0 {
        crt_row = (row - 1) as isize;
        crt_col = (col - 1) as isize;

        while crt_row >= 0 && crt_col >= 0 {
            match room[crt_row as usize][crt_col as usize] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_row -= 1;
            crt_col -= 1;
        }
    }

    // N
    if row != 0 {
        crt_row = (row - 1) as isize;

        while crt_row >= 0 {
            match room[crt_row as usize][col] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_row -= 1;
        }
    }

    // NE
    if row != 0 && col != room[0].len() - 1 {
        crt_row = (row - 1) as isize;
        crt_col = (col + 1) as isize;

        while crt_row >= 0 && crt_col != room[0].len() as isize {
            match room[crt_row as usize][crt_col as usize] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_row -= 1;
            crt_col += 1;
        }
    }

    // W
    if col != 0 {
        crt_col = (col - 1) as isize;

        while crt_col >= 0 {
            match room[row][crt_col as usize] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_col -= 1;
        }
    }

    // E
    if col != room[0].len() - 1 {
        crt_col = (col + 1) as isize;

        while crt_col != room[0].len() as isize {
            match room[row][crt_col as usize] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_col += 1;
        }
    }

    // SW
    if row != room.len() - 1 && col != 0 {
        crt_row = (row + 1) as isize;
        crt_col = (col - 1) as isize;

        while crt_row != room.len() as isize && crt_col >= 0 {
            match room[crt_row as usize][crt_col as usize] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_row += 1;
            crt_col -= 1;
        }
    }

    // S
    if row != room.len() - 1 {
        crt_row = (row + 1) as isize;

        while crt_row != room.len() as isize {
            match room[crt_row as usize][col] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_row += 1;
        }
    }

    // SE
    if row != room.len() - 1 && col != room[0].len() - 1 {
        crt_row = (row + 1) as isize;
        crt_col = (col + 1) as isize;

        while crt_row != room.len() as isize && crt_col != room[0].len() as isize {
            match room[crt_row as usize][crt_col as usize] {
                Seat::Empty => break,
                Seat::Floor => {}
                Seat::Occupied => {
                    neighbours += 1;
                    break;
                }
            }

            crt_row += 1;
            crt_col += 1;
        }
    }

    neighbours
}

fn part_two(room: &mut Room) -> usize {
    let mut room_changed: bool;

    loop {
        room_changed = false;

        let mut next_room = room.clone();

        for row in 0..room.len() {
            for col in 0..room[0].len() {
                match room[row][col] {
                    Seat::Floor => continue,
                    Seat::Empty => {
                        let neighbours = get_seat_neighbours_two(room, row, col);
                        if neighbours == 0 {
                            next_room[row][col] = Seat::Occupied;
                            room_changed = true;
                        }
                    }
                    Seat::Occupied => {
                        let neighbours = get_seat_neighbours_two(room, row, col);
                        if neighbours >= 5 {
                            next_room[row][col] = Seat::Empty;
                            room_changed = true;
                        }
                    }
                }
            }
        }

        if !room_changed {
            break;
        }

        *room = next_room;
    }

    room.iter()
        .flatten()
        .filter(|seat| **seat == Seat::Occupied)
        .count()
}

/**
 * You are on a ferry and analyse how the empty seats in the waiting room
 *  are filled over time. It's Conway's Game of Life with a small twist.
 *
 * There are three types of cells on our map:
 *
 *     L - empty seat (can flip to occupied)
 *     . - floor (can't be used for anything)
 *     # - occupied seat (can flip to empty)
 *
 * Part 1:
 *  An empty seat is occupied if it has no neighbours.
 *  An occupied seat will be emptied if there are at least 4 direct neighbours.
 *
 *  How many rounds does it take for the room to stop changing?
 *
 * Part 2:
 *  You notice that people don't look for direct neighbours. They instead
 *   look in each direction until they see *any* chair.
 *  
 * The rule for occupied seats has also changed. They will be emptied at
 *   5 or more neighbours instead of 4.
 *
 * How many rounds does it take for the room to stop changing?
 *
 * IMPORTANT!
 * Looking ahead, if the first seat is empty then you don't
 *  have a neighbour in front of you.
 */
fn main() {
    let room = get_input("src/day-11/input.txt");

    let mut part_one_room = room.clone();
    println!("Solution for part 1: {}", part_one(&mut part_one_room)); // 2310

    let mut part_two_room = room.clone();
    println!("Solution for part 2: {}", part_two(&mut part_two_room)); // 2074
}
