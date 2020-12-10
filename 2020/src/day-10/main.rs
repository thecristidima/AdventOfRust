use utils::files::read_lines_as_vec;

fn part_one(nums: &Vec<u32>) -> i32 {
    let mut count_1 = 0;
    let mut count_3 = 0;

    for idx in 0..nums.len() - 1 {
        if nums[idx + 1] - nums[idx] == 1 {
            count_1 += 1;
        } else if nums[idx + 1] - nums[idx] == 3 {
            count_3 += 1;
        }
    }

    count_1 * count_3
}

fn part_two(nums: &Vec<u32>) -> u64 {
    let mut paths: Vec<u64> = vec![1; nums.len()];

    // We always include the last two elements (max->device), skip them.
    // From the second to last element, we only have one possible branch
    //  (second_to_last->max->device), so we can skip that one too.
    for idx in (0..nums.len() - 3).rev() {
        if nums[idx + 3] - nums[idx] <= 3 {
            // 3 branches
            paths[idx] = paths[idx + 1] + paths[idx + 2] + paths[idx + 3]
        } else if nums[idx + 2] - nums[idx] <= 3 {
            // 2 branches
            paths[idx] = paths[idx + 1] + paths[idx + 2];
        } else {
            // 1 branch, copy value from right
            paths[idx] = paths[idx + 1];
        }
    }

    paths[0]
}

/**
 * Your device is dead and you need to charge it. You have some joltage
 *  adapters. You can connect adapter n to n-1 if jolt(n)-jolt(n-1) <= 3.
 *
 * Keep in mind jolt(socket) = 0 and jolt(device) = jolt(max_adapter) + 3.
 *
 * Part 1:
 *  Using all your adapters, find the number of joltage jumps of 1 and 3.
 *  A joltage jump is jolt(n+1) - jolt(n).
 *
 *  What is count_jump_1 * count_jump_3?
 *
 * Part 2:
 *  You don't really have to use all your adapters. How many ways can you
 *   chain them up to charge your device?
 */
fn main() {
    let mut nums = read_lines_as_vec::<u32>("src/day-10/input.txt");

    nums.push(0); // add wall socket joltage
    nums.push(nums.iter().max().unwrap() + 3); // device joltage

    nums.sort();

    println!("Solution for part 1: {}", part_one(&nums)); // 2475

    // DYNAMIC PROGRAMMING CAN GO FUCK ITSELF
    println!("Solution for part 2: {}", part_two(&nums)); // 442136281481216
}
