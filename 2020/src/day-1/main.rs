use std::collections::HashSet;

use utils::files::read_lines_iter;

fn part_1() {
    let sum = 2020;
    let mut set = HashSet::<i32>::new();

    for line in read_lines_iter("src/day-1/input.txt") {
        let num = line
            .expect("line exists")
            .parse::<i32>()
            .expect("line is number");

        let target = sum - num;

        if set.contains(&target) {
            println!("Solution for part 1 is: {}", num * target); // 921504
            break;
        } else {
            set.insert(num);
        }
    }
}

fn part_2() {
    let sum = 2020;
    let mut list = Vec::<(i32, i32, i8)>::new();

    for line in read_lines_iter("src/day-1/input.txt") {
        let num = line
            .expect("line exists")
            .parse::<i32>()
            .expect("line is number");

        let target = sum - num;

        let mut new_entries = Vec::<(i32, i32, i8)>::new();
        for entry in &list {
            match entry {
                (a, b, 2) => {
                    if a + b == target {
                        println!("Solution for part 2 is {}", a * b * num); // 195700142
                        return;
                    }
                }
                (a, _, 1) => {
                    new_entries.push((*a, num, 2));
                }
                _ => {}
            }
        }

        list.push((num, 0, 1)); // (first_num, second_num, entries_in_tuple)
        list.append(&mut new_entries);
    }
}

/**
 * Part 1:
 * 	You get a list of expenses, find the two expenses that add up to 2020.
 * 	Return their product.
 *
 * Part 2:
 * 	Using the same list, find the _three_ expenses that add up to the same target.
 * 	Return their product
 */
fn main() {
    part_1();
    part_2();
}
