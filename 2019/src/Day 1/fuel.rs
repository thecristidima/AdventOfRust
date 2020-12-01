use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute_needed_fuel(mass: i32) -> i32 {
    return i32::max(mass / 3 - 2, 0);
}

fn compute_actual_needed_fuel(mass: i32) -> i32 {
    let mut fuel = compute_needed_fuel(mass);
    let mut additional_fuel = fuel;

    while additional_fuel > 0 {
        additional_fuel = compute_needed_fuel(additional_fuel);
        fuel += additional_fuel;
    }

    return fuel;
}

fn main() {
    let input = File::open("src/Day 1/fuel.in").unwrap();
    let reader = BufReader::new(input);

    let mut total_fuel = 0;
    let mut actual_total_fuel = 0;
    for module in reader.lines() {
        let mass: i32 = module.unwrap().parse::<i32>().unwrap();

        total_fuel += compute_needed_fuel(mass);
        actual_total_fuel += compute_actual_needed_fuel(mass);
    }

    println!("Total fuel needed: {}", total_fuel);
    println!("Actual total fuel needed: {}", actual_total_fuel);
}