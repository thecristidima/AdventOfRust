use utils::files::read_lines_iter;

#[derive(Debug, Clone)]
enum OperationType {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
struct Instruction {
    pub op_type: OperationType,
    pub arg: i32,
}

fn read_input(path: &str) -> Vec<(Instruction, u32)> {
    let mut instructions = Vec::<(Instruction, u32)>::new();

    for line in read_lines_iter(path) {
        let line = line.unwrap();

        let op_type = match line.split_ascii_whitespace().nth(0).unwrap() {
            "nop" => OperationType::Nop,
            "acc" => OperationType::Acc,
            "jmp" => OperationType::Jmp,
            _ => {
                panic!("Something is wrong")
            }
        };

        let arg = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        instructions.push((Instruction { op_type, arg }, 0));
    }

    instructions
}

fn part_one(instructions: &Vec<(Instruction, u32)>) -> (i32, bool) /* (accumulator, finished execution) */
{
    let mut instructions = instructions.clone();

    let mut acc = 0;
    let mut idx = 0 as usize;

    loop {
        if idx >= instructions.len() {
            return (acc, true);
        } else if instructions[idx].1 == 1 {
            return (acc, false);
        }

        instructions[idx].1 += 1;

        let instruction = &instructions[idx].0;

        match instruction.op_type {
            OperationType::Nop => {
                idx += 1;
            }
            OperationType::Acc => {
                idx += 1;
                acc += instruction.arg;
            }
            OperationType::Jmp => {
                idx = (idx as i32 + instruction.arg) as usize;
            }
        }
    }
}

fn part_two(instructions: &mut Vec<(Instruction, u32)>) -> i32 {
    for idx in 0..instructions.len() {
        let mut cloned_instructions = instructions.clone();

        // flip instruction
        match cloned_instructions[idx].0.op_type {
            OperationType::Nop => {
                cloned_instructions[idx].0.op_type = OperationType::Jmp;
            }
            OperationType::Jmp => {
                cloned_instructions[idx].0.op_type = OperationType::Nop;
            }
            OperationType::Acc => {
                continue;
            }
        }

        let res = part_one(&cloned_instructions);
        if res.1 {
            return res.0;
        }

        // flip it back
        match cloned_instructions[idx].0.op_type {
            OperationType::Nop => {
                cloned_instructions[idx].0.op_type = OperationType::Jmp;
            }
            OperationType::Jmp => {
                cloned_instructions[idx].0.op_type = OperationType::Nop;
            }
            OperationType::Acc => {}
        }
    }

    panic!("the fuck?");
}

/**
 * Someone on the plane gives you their broken GameBoy.
 * The console runs using only 3 instructions - nop, jmp, acc.
 * Each instruction is followed by an argument (nop always has +0).
 *
 * Nop -> Do nothing, go to next instruction
 * Acc -> Add the argument (can be negative) to a global accumulator
 * Jmp -> Jmp forward (or backwards) the number of instructions specified
 *          in the argument (+n forwards, -n backwards)
 *
 * Part 1:
 *  The program has an infinite loop. What is the accumulator value *before*
 *   any instruction is ran a second time?
 *
 * Part 2:
 *  There is a single corrupted instruction in the program, but you don't know
 *   which one. It can be fixed by flipping a nop to a jmp, or vice versa.
 *  
 *  What is the accumulator value after the program execution ends successfully?
 */
fn main() {
    let mut instructions = read_input("src/day-8/input.txt");

    println!("Solution for part 1: {}", part_one(&instructions).0); // 1744

    println!("Solution for part 2: {}", part_two(&mut instructions)); // 1174
}
