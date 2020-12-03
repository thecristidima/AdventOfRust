use utils::files::read_lines_iter;

enum MapCell {
    Clear,
    Tree,
}

type Map = Vec<Vec<MapCell>>;

fn get_map(path: &str) -> Map {
    let mut map = Vec::<Vec<MapCell>>::new();

    for line in read_lines_iter(path) {
        let line = line.expect("line is valid input");

        let map_row = line
            .chars()
            .map(|c| match c {
                '.' => MapCell::Clear,
                '#' => MapCell::Tree,
                _ => panic!("shouldn't be here"),
            })
            .collect::<Vec<MapCell>>();

        map.push(map_row);
    }

    map
}

fn get_trees_hit_for_slope(map: &Map, slope: (i32, i32)) -> u32 {
    let mut row = 0 as usize;
    let mut col = 0 as usize;

    let (row_trajectory, col_trajectory) = slope;

    let col_limit = map[0].len();

    let mut trees_hit = 0;

    while row < map.len() {
        match map[row][col] {
            MapCell::Clear => {}
            MapCell::Tree => {
                trees_hit += 1;
            }
        }

        row += row_trajectory as usize;
        col = (col + col_trajectory as usize) % col_limit;
    }

    trees_hit
}

/**
 * You head down to the airport using a sled.
 * The area in front of you, however, is filled with trees.
 *
 * Part 1:
 * 	The cheapest sled you found has a single slope of 1 down, 3 right.
 * 	How many trees will you hit?
 *
 * Part 2:
 * 	You now have a sled with several slopes.
 * 	Multiply the number of trees you hit for each slope.
 */
fn main() {
    let map = get_map("src/day-3/input.txt");

    let part_one_solution = get_trees_hit_for_slope(&map, (1, 3));
    println!("Solution for part 1: {}", part_one_solution); // 230

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut part_two_solution: u64 = 1;

    for slope in slopes {
        part_two_solution *= get_trees_hit_for_slope(&map, slope) as u64;
    }

    println!("Solution for part 2: {}", part_two_solution); // 9533698720
}
