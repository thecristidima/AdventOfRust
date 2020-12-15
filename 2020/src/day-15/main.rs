use std::collections::HashMap;

fn get_initial_memory(input: &Vec<i32>) -> HashMap<i32, (i32, i32)> {
    let mut memory = HashMap::new();

    for (idx, val) in input.iter().enumerate() {
        memory.insert(*val, (-1, idx as i32));
    }

    memory
}

fn find_nth_number(input: &Vec<i32>, n: i32) -> i32 {
    let mut last_number = *input.last().unwrap();
    let mut memory = get_initial_memory(&input);

    for idx in input.len() as i32..n {
        let crt_num: i32;
        let num_info = memory.get(&last_number).unwrap();

        if num_info.0 == -1 {
            // last_num was said for the first time
            crt_num = 0;
        } else {
            crt_num = num_info.1 - num_info.0;
        }

        if memory.contains_key(&crt_num) {
            let num_memory = memory.get_mut(&crt_num).unwrap();

            num_memory.0 = num_memory.1;
            num_memory.1 = idx;
        } else {
            memory.insert(crt_num, (-1, idx));
        }

        last_number = crt_num;
    }

    last_number
}

/**
 * Note: This was by far the laziest task so far.
 *
 * You wait for your flight and decide to play a memory game with some elves.
 * Starting from an input, each person then says the next number in the series
 * based on the following rules:
 *  - if the last number was *never* said before, you say 0
 *  - if the last number *was* said before, you say how many turns before
 *    it was said
 *
 * Part 1:
 *  Find the 2020th number.
 *
 * Part 2:
 *  Find the 30000000th number.
 */
fn main() {
    let input = vec![8, 0, 17, 4, 1, 12];

    println!("Solution for part 1: {}", find_nth_number(&input, 2020)); // 981
    println!("Solution for part 2: {}", find_nth_number(&input, 30000000)); // 164878
}
