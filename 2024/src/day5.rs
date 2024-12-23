use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> (i32, i32) {
    let (order_dict, mut rules) = read_input("inputs/day5.full");

    (
        part_one(&order_dict, &rules),
        part_two(&order_dict, &mut rules),
    )
}

fn read_input(path: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let reader = BufReader::new(File::open(path).unwrap());

    let mut order_dict = HashMap::<i32, HashSet<i32>>::new();
    let mut rules = Vec::<Vec<i32>>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() { // do nothing, it's the blank line
        } else if line.contains("|") {
            // before|after
            let nums: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
            let first = nums[0];
            let second = nums[1];

            order_dict
                .entry(second)
                .and_modify(|entry| {
                    entry.insert(first);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(first);
                    set
                });
        } else {
            // whole line with a rule, comma-separated
            rules.push(line.split(',').map(|s| s.parse().unwrap()).collect());
        }
    }

    (order_dict, rules)
}

fn part_one(dict: &HashMap<i32, HashSet<i32>>, rules: &[Vec<i32>]) -> i32 {
    let mut ret = 0;
    for valid_rule in rules.iter().filter(|rule| is_rule_valid(dict, rule).0) {
        let mid = valid_rule.len() / 2;
        ret += valid_rule[mid];
    }

    ret
}

fn is_rule_valid(dict: &HashMap<i32, HashSet<i32>>, rule: &[i32]) -> (bool, (usize, usize)) {
    for (idx, num) in rule.iter().enumerate() {
        if !dict.contains_key(num) {
            continue;
        }

        let set = dict.get(num).unwrap();
        for (next_idx, next_num) in rule.iter().enumerate().skip(idx) {
            if set.contains(next_num) {
                return (false, (idx, next_idx));
            }
        }
    }

    (true, (usize::MAX, usize::MAX))
}

fn part_two(dict: &HashMap<i32, HashSet<i32>>, rules: &mut [Vec<i32>]) -> i32 {
    let mut ret = 0;

    let invalid_rules = rules.iter_mut().filter(|rule| !is_rule_valid(dict, rule).0);

    for rule in invalid_rules {
        loop {
            let (is_valid, to_swap) = is_rule_valid(dict, rule);

            if is_valid {
                break;
            }

            rule.swap(to_swap.0, to_swap.1);
        }

        ret += rule[rule.len() / 2];
    }

    ret
}
