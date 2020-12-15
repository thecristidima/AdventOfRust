use std::collections::HashMap;

use utils::files::read_lines_iter;

fn parse_memory_write(input: &str) -> (u64, u64) {
    let bracket_start = input.chars().into_iter().position(|c| c == '[').unwrap();
    let bracket_end = input.chars().into_iter().position(|c| c == ']').unwrap();

    let address = input[bracket_start + 1..bracket_end].parse().unwrap();

    let value = input[bracket_end + 4..].parse().unwrap();

    (address, value)
}

fn apply_mask(val: u64, mask: &str) -> u64 {
    let mut masked_val = val;

    for (idx, chr) in mask.chars().rev().enumerate() {
        match chr {
            '0' => {
                masked_val &= !(1 << idx);
            }
            '1' => {
                masked_val |= 1 << idx;
            }
            _ => continue,
        }
    }

    masked_val
}

fn apply_mask_v2(val: u64, mask: &str) -> Vec<u64> {
    let mut masked_vals = vec![val];

    for (idx, chr) in mask.chars().rev().enumerate() {
        match chr {
            '1' => {
                for masked_val in &mut masked_vals {
                    *masked_val |= 1 << idx;
                }
            }
            'X' => {
                let mut new_masked_vals = Vec::<u64>::new();

                for val in masked_vals {
                    new_masked_vals.push(val | (1 << idx)); // bit set to 0
                    new_masked_vals.push(val & !(1 << idx)); // bit set to 1
                }

                masked_vals = new_masked_vals;
            }
            _ => continue,
        }
    }

    masked_vals
}

/**
 * You hack into the ship's computer and take a look at the docking system.
 * The system's initalization sets some values in memory. You notice there
 * are two possible commands
 *
 *     mem[idx] = val
 *     mask = some 32-bit value with digits 0/1/X
 *
 * Part 1:
 *  You think the program works as follows:
 *   - The mask command just sets the global mask
 *   - mem[idx] = val will apply the mask to val and set it at mem[idx]
 *
 *  The mask works as follows:
 *   - 0 sets current bit to 0
 *   - 1 sets current bit to 1
 *   - X doesn't change the current bit
 *  
 *  What is the sum of all the values in memory?
 *
 * Part 2:
 *  The first attempt failed. You try the following:
 *   - The mask command still does the same thing
 *   - mem[idx] = val will actually write val (not masked) to all masked idx
 *
 *  You notice that the masking works like this:
 *   - 0 doesn't change the current bit
 *   - 1 sets current bit to 1
 *   - X sets a *floating* bit
 *
 *  Floating bits can actually be either 0 or 1, so you decide to write val to
 *  all possible indices. This means that for index 10X we will write to 4 and 5.
 *
 *  What is the sum of all the values in memory?
 *
 */
fn main() {
    let mut part_one_memory = HashMap::<u64, u64>::new();
    let mut part_two_memory = HashMap::<u64, u64>::new();
    let mut mask = String::from("");

    for line in read_lines_iter("src/day-14/input.txt") {
        let line = line.unwrap();

        if line.contains("mask") {
            mask = String::from(&line[7..]);
        } else {
            let (address, value) = parse_memory_write(&line);

            let masked_value = apply_mask(value, &mask);

            part_one_memory.insert(address, masked_value);

            let masked_addresses = apply_mask_v2(address, &mask);

            for addr in masked_addresses {
                part_two_memory.insert(addr, value);
            }
        }
    }

    println!(
        "Solution for part 1: {}",
        part_one_memory.values().sum::<u64>()
    ); // 12408060320841
    println!(
        "Solution for part 2: {}",
        part_two_memory.values().sum::<u64>()
    ); // 4466434626828
}
