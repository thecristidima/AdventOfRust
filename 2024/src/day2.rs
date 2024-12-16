use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> (i32, i32) {
    let reports = read_input("inputs/day2.full");

    (part_one(&reports), part_two(&reports))
}

fn read_input(path: &str) -> Vec<Vec<i32>> {
    let mut ret = Vec::<Vec<i32>>::new();

    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let nums = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        ret.push(nums);
    }

    ret
}

fn part_one(reports: &[Vec<i32>]) -> i32 {
    reports.iter().filter(|r| is_report_valid(r, false)).count() as i32
}

fn part_two(reports: &[Vec<i32>]) -> i32 {
    reports.iter().filter(|r| is_report_valid(r, true)).count() as i32
}

fn is_report_valid(report: &[i32], with_retry: bool) -> bool {
    match with_retry {
        false => validate_report(report).is_ok(),
        true => validate_report_with_retry(report).is_ok(),
    }
}

fn validate_report(report: &[i32]) -> Result<(), usize> {
    let mut ascending = false;
    let mut descending = false;

    for idx in 0..report.len() - 1 {
        let diff = report[idx + 1] - report[idx];

        // change isn't gradual
        if diff < -3 || (diff > -1 && diff < 1) || diff > 3 {
            return Err(idx);
        }

        // descending and ascending at the same time
        match (diff > 0, ascending, descending) {
            (true, _, true) => return Err(idx),
            (true, _, _) => ascending = true,
            (false, true, _) => return Err(idx),
            (false, _, _) => descending = true,
        }
    }

    Ok(())
}

fn validate_report_with_retry(report: &[i32]) -> Result<(), usize> {
    match validate_report(report) {
        Ok(()) => Ok(()),
        Err(idx) => {
            // try without idx - 1 -> 1 5 6 7
            if idx > 0 && validate_report(vec_without(report, idx - 1).as_slice()).is_ok() {
                return Ok(());
            }

            // try without idx -> 2 1 3 4
            if validate_report(vec_without(report, idx).as_slice()).is_ok() {
                return Ok(());
            }

            // try without idx + 1 -> 1 2 3 8
            if validate_report(vec_without(report, idx + 1).as_slice()).is_ok() {
                return Ok(());
            }

            Err(idx)
        }
    }
}

fn vec_without(vec: &[i32], idx: usize) -> Vec<i32> {
    if idx == 0 {
        vec[1..].to_vec()
    } else if idx < vec.len() {
        let mut partial_vec = vec[..idx].to_vec();
        partial_vec.extend_from_slice(&vec[idx + 1..]);
        partial_vec
    } else {
        vec.to_vec()
    }
}
