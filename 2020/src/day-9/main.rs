use utils::files::read_lines_as_vec;

fn all_slice_sums(slice: &[i64]) -> Vec<i64> {
    let mut sums = Vec::<i64>::new();

    for first in 0..slice.len() - 1 {
        for second in first + 1..slice.len() {
            sums.push(slice[first] + slice[second]);
        }
    }

    sums
}

fn part_one(nums: &Vec<i64>, window_size: usize) -> i64 {
    let mut start = 0 as usize;
    let mut end = start + window_size;

    while end < nums.len() {
        let slice_sums = all_slice_sums(&nums[start..end]);

        if !slice_sums.contains(&nums[end]) {
            return nums[end];
        }

        start += 1;
        end += 1;
    }

    panic!("the fuck?");
}

fn part_two(nums: &Vec<i64>, target: i64) -> &[i64] {
    let mut start = 0 as usize;
    let mut end = 1 as usize;

    while end < nums.len() - 1 {
        let sum = nums[start..=end].iter().sum::<i64>();

        match sum {
            s if s == target => {
                return &nums[start..=end];
            }
            s if s < target => {
                end += 1;
            }
            s if s > target && start != end => {
                start += 1;
            }
            // we reached a number larger than target
            // so advance to the next 2-element slice
            s if s > target && start == end => {
                start += 1;
                end += 2;
            }
            _ => {}
        }
    }

    panic!("should not have reached this point");
}

/**
 * You hack into the little screen in front of you seat using a paperclip.
 * You notice the screen uses the eXchange-Masking Addition System (XMAS).
 *
 * The system will send a list of numbers starting with a windows of 25.
 * Every nth number should be the sum of two of the previous 25 numbers.
 *
 * Part 1:
 *  The first step in cracking XMAS is to identify the only number in the
 *   input that doesn't follow the rule of 25. Which number is it?
 *
 * Part 2:
 *  The second step is to find the encryption weakness. To do this, find
 *   a continuous sequence of numbers that sums up to the invalid number.
 *  
 *  The encryption weakness will be the sum of the minimum and maximum
 *   elements of that sequence.
 *
 *  What is the encryption weakness?
 */
fn main() {
    let nums = read_lines_as_vec::<i64>("src/day-9/input.txt");

    let invalid_number = part_one(&nums, 25);

    println!("Solution for part 1: {}", invalid_number); // 90433990

    let biggest_slice = part_two(&nums, invalid_number);
    let min = biggest_slice.iter().min().unwrap();
    let max = biggest_slice.iter().max().unwrap();

    println!("Solution for part 2: {}", min + max); // 11691646
}
