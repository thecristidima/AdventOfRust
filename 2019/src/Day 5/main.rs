use std::fs;
use std::str::FromStr;

use calculator::Calculator;

fn read_input() -> Vec<i32> {
	fs::read_to_string("src/Day 5/intcode.in")
		.unwrap()
		.trim()
		.split(",")
		.map(|s| FromStr::from_str(s).unwrap())
		.collect()
}

fn main() {
	let mut memory = read_input();

	let mut calculator = Calculator::new();

	calculator.set_input(1);
	calculator.load_memory(memory.as_mut_slice());
	calculator.run();
}