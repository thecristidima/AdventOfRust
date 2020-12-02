use std::collections::HashSet;

use utils::files::read_lines_iter;

pub fn part_one(input_path: &str, target_sum: i32) {
    let mut set = HashSet::<i32>::new();

    for line in read_lines_iter(input_path) {
        let num = line
            .expect("line exists")
            .parse::<i32>()
            .expect("line is number");

        let target = target_sum - num;

        if set.contains(&target) {
            println!("Solution for part 1 is: {}", num * target); // 921504
            break;
        } else {
            set.insert(num);
        }
    }
}

pub fn part_two(input_path: &str, target_sum: i32) {
    let mut list = Vec::<(i32, i32, i8)>::new();

    for line in read_lines_iter(input_path) {
        let num = line
            .expect("line exists")
            .parse::<i32>()
            .expect("line is number");

        let target = target_sum - num;

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
