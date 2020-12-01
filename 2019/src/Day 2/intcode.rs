use std::fs;
use std::str::FromStr;

use calculator::Calculator;

fn main() {
    let memory: Vec<i32> = fs::read_to_string("src/Day 2/intcode.in")
        .unwrap()
        .trim()
        .split(",")
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let mut mem_clone = Vec::<i32>::from(memory.as_slice());

    let mut calculator = Calculator::new();
    calculator.load_memory(mem_clone.as_mut_slice());
    calculator.run();

    println!("Part 1 - {}", calculator.dump_memory()[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            mem_clone = Vec::<i32>::from(memory.as_slice());
            mem_clone[1] = noun;
            mem_clone[2] = verb;

            calculator.load_memory(mem_clone.as_mut_slice());
            calculator.run();

            if calculator.dump_memory()[0] == 19690720 {
                println!("Part 2 - noun: {}, verb: {}, result: {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}