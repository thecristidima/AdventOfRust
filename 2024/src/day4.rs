use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> (i32, i32) {
    let board = read_input("inputs/day4.full");

    (part_one(&board), part_two(&board))
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn part_one(board: &[Vec<char>]) -> i32 {
    let mut ret = 0;

    for (row, _) in board.iter().enumerate() {
        for (col, letter) in board[row].iter().enumerate() {
            if *letter == 'X' {
                ret += search_xmas(board, row, col);
            }
        }
    }

    ret
}

fn search_xmas(board: &[Vec<char>], row: usize, col: usize) -> i32 {
    let row_count = board.len();
    let col_count = board[0].len();
    let mut ret = 0;

    // NW
    if col >= 3
        && row >= 3
        && board[row - 1][col - 1] == 'M'
        && board[row - 2][col - 2] == 'A'
        && board[row - 3][col - 3] == 'S'
    {
        ret += 1;
    }

    // N
    if row >= 3
        && board[row - 1][col] == 'M'
        && board[row - 2][col] == 'A'
        && board[row - 3][col] == 'S'
    {
        ret += 1;
    }

    // NE
    if row >= 3
        && col < col_count - 3
        && board[row - 1][col + 1] == 'M'
        && board[row - 2][col + 2] == 'A'
        && board[row - 3][col + 3] == 'S'
    {
        ret += 1;
    }

    // E
    if col < col_count - 3
        && board[row][col + 1] == 'M'
        && board[row][col + 2] == 'A'
        && board[row][col + 3] == 'S'
    {
        ret += 1;
    }

    // SE
    if row < row_count - 3
        && col < col_count - 3
        && board[row + 1][col + 1] == 'M'
        && board[row + 2][col + 2] == 'A'
        && board[row + 3][col + 3] == 'S'
    {
        ret += 1;
    }

    // S
    if row < row_count - 3
        && board[row + 1][col] == 'M'
        && board[row + 2][col] == 'A'
        && board[row + 3][col] == 'S'
    {
        ret += 1;
    }

    // SW
    if row < row_count - 3
        && col >= 3
        && board[row + 1][col - 1] == 'M'
        && board[row + 2][col - 2] == 'A'
        && board[row + 3][col - 3] == 'S'
    {
        ret += 1;
    }

    // W
    if col >= 3
        && board[row][col - 1] == 'M'
        && board[row][col - 2] == 'A'
        && board[row][col - 3] == 'S'
    {
        ret += 1;
    }

    ret
}

fn part_two(board: &[Vec<char>]) -> i32 {
    let mut ret = 0;

    for (row, _) in board.iter().enumerate() {
        for (col, letter) in board[row].iter().enumerate() {
            if *letter == 'A' {
                ret += search_cross_mas(board, row, col);
            }
        }
    }

    ret
}

fn search_cross_mas(board: &[Vec<char>], row: usize, col: usize) -> i32 {
    let row_count = board.len();
    let col_count = board[0].len();
    let mut ret = 0;

    // Return early in case we're on the edge of the board
    if row == 0 || col == 0 || row == row_count - 1 || col == col_count - 1 {
        return ret;
    }

    // We only want to find MAS in the shape of an X, so only diagonally
    if ((board[row - 1][col - 1] == 'M' && board[row + 1][col + 1] == 'S')
        || (board[row - 1][col - 1] == 'S' && board[row + 1][col + 1] == 'M'))
        && ((board[row + 1][col - 1] == 'M' && board[row - 1][col + 1] == 'S')
            || (board[row + 1][col - 1] == 'S' && board[row - 1][col + 1] == 'M'))
    {
        ret += 1;
    }

    ret
}
