use std::fs;
use std::str::FromStr;

fn run_opcode(memory: &mut [i32], idx: usize) {
    if memory[idx] == 1 {
        memory[memory[idx + 3] as usize] =
            memory[memory[idx + 1] as usize] + memory[memory[idx + 2] as usize];
    } else if memory[idx] == 2 {
        memory[memory[idx + 3] as usize] =
            memory[memory[idx + 1] as usize] * memory[memory[idx + 2] as usize];
    }
}

fn run_program(memory: &mut [i32]) {
    for idx in (0..memory.len()).step_by(4) {
        if memory[idx] == 99 {
            break;
        }

        run_opcode(memory.as_mut(), idx);
    }
}

fn main() {
    let memory: Vec<i32> = fs::read_to_string("src/Day 2/intcode.in")
        .unwrap()
        .trim()
        .split(",")
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let mut mem_clone = Vec::<i32>::from(memory.as_slice());

    run_program(mem_clone.as_mut());
    println!("Part 1 - {}", mem_clone[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            mem_clone = Vec::<i32>::from(memory.as_slice());
            mem_clone[1] = noun;
            mem_clone[2] = verb;

            run_program(mem_clone.as_mut());
            if mem_clone[0] == 19690720 {
                println!("Part 2 - noun: {}, verb: {}, result: {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}