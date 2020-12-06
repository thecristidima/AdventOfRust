use std::collections::HashSet;

use utils::files::read_lines_iter;

fn part_one(path: &str) {
    let mut yes_count: u32 = 0;

    let mut current_group_answers = String::new();
    for line in read_lines_iter(path) {
        let line = line.unwrap();

        if line.is_empty() {
            let crt_yes_count = current_group_answers
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
                .len() as u32;

            yes_count += crt_yes_count;

            current_group_answers = String::new();
        } else {
            current_group_answers.push_str(&line);
        }
    }

    println!("Solution for part 1: {}", yes_count);
}

fn part_two(path: &str) {
    let mut yes_count: u32 = 0;

    let mut is_first_in_group = true;
    let mut crt_grp_common_yes_answers = HashSet::<char>::new();
    for line in read_lines_iter(path) {
        let line = line.unwrap();

        if line.is_empty() {
            yes_count += crt_grp_common_yes_answers.len() as u32;

            // reset stuff
            is_first_in_group = true;
            crt_grp_common_yes_answers.clear();
        } else {
            let crt_yes_answers = line
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>();

            if is_first_in_group {
                is_first_in_group = false;
                crt_grp_common_yes_answers = crt_yes_answers;
            } else {
                crt_grp_common_yes_answers = crt_grp_common_yes_answers
                    .intersection(&crt_yes_answers)
                    .map(|c| *c)
                    .collect::<HashSet<_>>();
            }
        }
    }

    println!("Solution for part 2: {}", yes_count);
}

/**
 * You have to fill in a customs declaration form by answering a few
 *  `yes`/`no` questions. Some people on the plane don't speak the language
 *  so they ask you to help them, too. You end up doing everybody's forms.
 *
 * The form only asks you to write down the questions for which your answer
 *  is `yes`. Each person's answer fits in one line, all people in a group
 *  will have answers on consecutive lines. Group answers are separed by
 *  an empty line.
 *
 * For example, the following is the input for three groups, the first one
 *  consisting of only one person and the other one of two. The next group
 *  is made up of two people, one answers a, the other one a,b. The last group
 *  also consists of two people with answers a and b,c, respectively.
 *
 *     abc
 *
 *     a
 *     ab
 *
 *     a
 *     bc
 *
 * Part 1:
 *  You count the distinct `yes` answers per group. What's the total sum?
 *  For our example, the sum is:
 *
 *      3 (a,b,c) + 2 (a, b) + 3 (a,b,c) = 8
 *
 * Part 2:
 *  You realise you were supposed to find out how many common `yes` answers
 *   there are per group. What's the total sum in this case? In our case:
 *  
 *      3 (a,b,c) + 1 (a) + 0 = 4
 */
fn main() {
    part_one("src/day-6/input.txt"); // 6443
    part_two("src/day-6/input.txt"); // 3232
}
