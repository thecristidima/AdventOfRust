use std::fs::{self};

use regex::Regex;

pub fn solve() -> (i32, i32) {
    let input = read_input("inputs/day3.full");

    (part_one(&input), part_two(&input))
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn part_one(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut ret = 0;

    for (_, [first, second]) in regex.captures_iter(input).map(|c| c.extract()) {
        ret += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }

    ret
}

fn part_two(input: &str) -> i32 {
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut process_muls = true;
    let mut ret = 0;

    for regex_match in regex.find_iter(input).map(|m| m.as_str()) {
        match regex_match {
            "do()" => process_muls = true,
            "don't()" => process_muls = false,
            _ => {
                if !process_muls {
                    continue;
                }

                let caps = mul_regex.captures(regex_match).unwrap();
                let first = &caps[1].parse::<i32>().unwrap();
                let second = &caps[2].parse::<i32>().unwrap();
                ret += first * second;
            }
        }
    }

    ret
}
