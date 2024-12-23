use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> (i32, i32) {
    let mut map = GuardMap::from_file("inputs/day6.demo");
    (part_one(&map), part_two(&mut map))
}

fn part_one(map: &GuardMap) -> i32 {
    let mut map = map.clone();
    map.compute_guard_path();
    map.count_visited()
}

fn part_two(map: &mut GuardMap) -> i32 {
    // I'm not doing this one, it's dumb
    -1
}

#[derive(Clone, Debug, PartialEq)]
enum MapPoint {
    Free,
    Visited,
    Obstacle,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
struct GuardMap {
    map: Vec<Vec<MapPoint>>,
    direction: Direction,
    start_pos: (usize, usize),
    crt_pos: (usize, usize),
}

impl GuardMap {
    pub fn from_file(path: &str) -> Self {
        let mut map = Vec::<Vec<MapPoint>>::new();
        let mut direction: Direction = Direction::Up;
        let mut start_pos: (usize, usize) = (0, 0);

        let reader = BufReader::new(File::open(path).unwrap());
        for (row, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut map_row = Vec::new();

            for (col, symbol) in line.chars().enumerate() {
                if symbol == '.' {
                    map_row.push(MapPoint::Free);
                } else if symbol == '#' {
                    map_row.push(MapPoint::Obstacle);
                } else {
                    map_row.push(MapPoint::Free);
                    start_pos = (row, col);
                    direction = match symbol {
                        '^' => Direction::Up,
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        _ => panic!("unexpected symbol {}", symbol),
                    };
                }
            }
            map.push(map_row);
        }

        GuardMap {
            map,
            direction,
            start_pos,
            crt_pos: start_pos,
        }
    }

    pub fn compute_guard_path(&mut self) {
        loop {
            let row = self.crt_pos.0;
            let col = self.crt_pos.1;
            self.map[row][col] = MapPoint::Visited;

            if !self.advance() {
                println!("Exited the map at {:?}", self.crt_pos);
                break;
            }
        }
    }

    fn advance(&mut self) -> bool {
        let row = self.crt_pos.0;
        let col = self.crt_pos.1;
        let last_row = self.map.len() - 1;
        let last_col = self.map[0].len() - 1;

        match self.direction {
            Direction::Up => {
                if row == 0 {
                    return false;
                } else if self.map[row - 1][col] == MapPoint::Obstacle {
                    self.change_direction();
                } else {
                    self.crt_pos = (row - 1, col);
                }
            }
            Direction::Right => {
                if col == last_col {
                    return false;
                } else if self.map[row][col + 1] == MapPoint::Obstacle {
                    self.change_direction();
                } else {
                    self.crt_pos = (row, col + 1);
                }
            }
            Direction::Down => {
                if row == last_row {
                    return false;
                } else if self.map[row + 1][col] == MapPoint::Obstacle {
                    self.change_direction();
                } else {
                    self.crt_pos = (row + 1, col);
                }
            }
            Direction::Left => {
                if col == 0 {
                    return false;
                } else if self.map[row][col - 1] == MapPoint::Obstacle {
                    self.change_direction();
                } else {
                    self.crt_pos = (row, col - 1);
                }
            }
        }

        true
    }

    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn count_visited(&self) -> i32 {
        let mut ret = 0;
        for row in &self.map {
            for cell in row {
                ret += match cell {
                    MapPoint::Visited => 1,
                    _ => 0,
                };
            }
        }
        ret
    }
}
