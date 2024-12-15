use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> (i32, i32) {
    let (first, second) = read_input("inputs/day1.full");

    (solve_first(&first, &second), solve_second(&first, &second))
}

fn read_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = Vec::<i32>::new();
    let mut second = Vec::<i32>::new();

    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let nums = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        first.push(nums[0]);
        second.push(nums[1]);
    }

    (first, second)
}

fn solve_first(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let mut first = first.to_owned();
    let mut second = second.to_owned();

    first.sort();
    second.sort();

    let mut res = 0;
    for idx in 0..first.len() {
        res += (first[idx] - second[idx]).abs();
    }

    res
}

fn solve_second(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let mut dict = HashMap::<i32, i32>::new();

    for num in second {
        if dict.contains_key(num) {
            dict.insert(num.to_owned(), dict[num] + 1);
        } else {
            dict.insert(num.to_owned(), 1);
        }
    }

    let mut res = 0;
    for num in first {
        if dict.contains_key(num) {
            res += num * dict[num];
        }
    }

    res
}
