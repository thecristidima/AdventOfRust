use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> (i32, i32) {
    let mut map = GuardMap::from_file("inputs/day6.demo");
    (part_one(&mut map), part_two(&mut map))
}

fn part_one(map: &mut GuardMap) -> i32 {
    map.compute_guard_path();
    map.count_visited()
}

fn part_two(map: &mut GuardMap) -> i32 {
    map.reset();
    map.try_create_loop()
}

#[derive(Clone, Debug)]
struct GuardMap {
    map: Vec<Vec<MapPoint>>,
    start_direction: Direction,
    crt_direction: Direction,
    start_pos: (usize, usize),
    crt_pos: (usize, usize),
    obstacle_set: HashSet<(Direction, (usize, usize))>,
}

impl GuardMap {
    pub fn from_file(path: &str) -> Self {
        let mut map = Vec::<Vec<MapPoint>>::new();
        let mut start_direction: Direction = Direction::Up;
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
                    start_direction = match symbol {
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
            start_direction,
            crt_direction: start_direction,
            start_pos,
            crt_pos: start_pos,
            obstacle_set: HashSet::new(),
        }
    }

    pub fn compute_guard_path(&mut self) -> bool {
        loop {
            let row = self.crt_pos.0;
            let col = self.crt_pos.1;
            self.map[row][col] = MapPoint::Visited;

            match self.advance() {
                AdvanceResult::Advanced => continue,
                AdvanceResult::HitObstacle => {
                    if self
                        .obstacle_set
                        .contains(&(self.crt_direction, self.crt_pos))
                    {
                        return false;
                    }

                    self.obstacle_set.insert((self.crt_direction, self.crt_pos));
                    self.change_direction();
                }
                AdvanceResult::ExitedMap => {
                    return true;
                }
            }
        }
    }

    fn advance(&mut self) -> AdvanceResult {
        let row = self.crt_pos.0;
        let col = self.crt_pos.1;
        let last_row = self.map.len() - 1;
        let last_col = self.map[0].len() - 1;

        match self.crt_direction {
            Direction::Up => {
                if row == 0 {
                    AdvanceResult::ExitedMap
                } else if self.map[row - 1][col] == MapPoint::Obstacle {
                    AdvanceResult::HitObstacle
                } else {
                    self.crt_pos = (row - 1, col);
                    AdvanceResult::Advanced
                }
            }
            Direction::Right => {
                if col == last_col {
                    AdvanceResult::ExitedMap
                } else if self.map[row][col + 1] == MapPoint::Obstacle {
                    AdvanceResult::HitObstacle
                } else {
                    self.crt_pos = (row, col + 1);
                    AdvanceResult::Advanced
                }
            }
            Direction::Down => {
                if row == last_row {
                    AdvanceResult::ExitedMap
                } else if self.map[row + 1][col] == MapPoint::Obstacle {
                    AdvanceResult::HitObstacle
                } else {
                    self.crt_pos = (row + 1, col);
                    AdvanceResult::Advanced
                }
            }
            Direction::Left => {
                if col == 0 {
                    AdvanceResult::ExitedMap
                } else if self.map[row][col - 1] == MapPoint::Obstacle {
                    AdvanceResult::HitObstacle
                } else {
                    self.crt_pos = (row, col - 1);
                    AdvanceResult::Advanced
                }
            }
        }
    }

    fn change_direction(&mut self) {
        self.crt_direction = match self.crt_direction {
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

    pub fn try_create_loop(&mut self) -> i32 {
        let mut ret = 0;

        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if (row, col) == self.start_pos {
                    continue; // can't place obstacle on guard (starting position)
                }

                if self.map[row][col] == MapPoint::Free {
                    self.map[row][col] = MapPoint::Obstacle;
                    if !self.compute_guard_path() {
                        ret += 1;
                    }
                    self.reset();
                    self.map[row][col] = MapPoint::Free;
                }
            }
        }

        ret
    }

    fn reset(&mut self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == MapPoint::Visited {
                    self.map[row][col] = MapPoint::Free;
                }
            }
        }
        self.crt_direction = self.start_direction;
        self.crt_pos = self.start_pos;
        self.obstacle_set = HashSet::new();
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                match self.map[row][col] {
                    MapPoint::Free => print!("."),
                    MapPoint::Visited => print!("|"),
                    MapPoint::Obstacle => print!("#"),
                };
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapPoint {
    Free,
    Visited,
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum AdvanceResult {
    Advanced,
    HitObstacle,
    ExitedMap,
}
