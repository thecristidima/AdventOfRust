use std::collections::{HashMap, HashSet};

use utils::files::read_lines_iter;

#[derive(Debug)]
struct Bag {
    pub children: HashSet<(String, u32)>,
    pub parents: HashSet<(String, u32)>,
}

impl Bag {
    fn new() -> Bag {
        Bag {
            children: HashSet::new(),
            parents: HashSet::new(),
        }
    }
}

fn get_bag_map(input_path: &str) -> HashMap<String, Bag> {
    let mut bags = HashMap::new();

    for line in read_lines_iter(input_path) {
        let line = line.unwrap();

        if line.contains("contain no other bags") {
            let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let empty_bag_name = format!("{} {}", parts[0], parts[1]);

            if !bags.contains_key(&empty_bag_name) {
                bags.insert(empty_bag_name, Bag::new());
            }
        } else {
            // Get parent name
            let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let parent_bag_name = format!("{} {}", parts[0], parts[1]);

            let mut parent_bag = Bag::new();

            if bags.contains_key(&parent_bag_name) {
                parent_bag = bags.remove(&parent_bag_name).unwrap();
            }

            // Get child names
            let mut child_bag_str = line.split("contain ").nth(1).unwrap().to_string();
            child_bag_str.pop();

            for child_bag in child_bag_str.split(", ").into_iter() {
                let parts = child_bag.split_ascii_whitespace().collect::<Vec<&str>>();
                let child_count = parts[0].parse::<u32>().unwrap();

                let child_bag_name = format!("{} {}", parts[1], parts[2]);

                let mut child_bag = Bag::new();

                if bags.contains_key(&child_bag_name) {
                    child_bag = bags.remove(&child_bag_name).unwrap();
                }

                child_bag.parents.insert((parent_bag_name.clone(), 0));

                bags.insert(child_bag_name.clone(), child_bag);

                parent_bag.children.insert((child_bag_name, child_count));
            }

            bags.insert(parent_bag_name, parent_bag);
        }
    }

    bags
}

fn part_one_recursive(
    bags: &HashMap<String, Bag>,
    target: &str,
    found_so_far: &mut HashSet<String>,
) {
    let crt_bag = bags.get(target).unwrap();

    for parent in &crt_bag.parents {
        found_so_far.insert(parent.0.clone());
        part_one_recursive(bags, &parent.0, found_so_far);
    }
}

fn part_one(bags: &HashMap<String, Bag>, target: String) {
    let mut parents = HashSet::<String>::new();

    part_one_recursive(&bags, &target, &mut parents);

    println!("Solution for part 1: {}", parents.len());
}

fn part_two_recursive(bags: &HashMap<String, Bag>, target: &str) -> u32 {
    let crt_bag = bags.get(target).unwrap();

    if crt_bag.children.is_empty() {
        return 0;
    }

    let mut total: u32 = 0;

    for child in &crt_bag.children {
        total += (part_two_recursive(&bags, &child.0) + 1) * child.1;
    }

    total
}

fn part_two(bags: &HashMap<String, Bag>, target: String) -> () {
    println!(
        "Solution for part 2: {}",
        part_two_recursive(&bags, &target)
    )
}

/**
 * The airport has some strict bag rules. Each bag type must hold a certain
 *  number of bags of each type (or none at all). For example:
 *
 *     yellow bags contain 1 red bag, 2 blue bags.
 *     red bags contain no other bags.
 *     blue bags contain no other bags.
 *
 * Your target bag is _shiny gold_.
 *
 * Part 1:
 *  How many bags can contain our target bag, regardless of depth?
 *
 * Part 2:
 *  How many bags will our target bag hold?
 *
 */
fn main() {
    let bags = get_bag_map("src/day-7/input.txt");

    part_one(&bags, String::from("shiny gold")); // 177
    part_two(&bags, String::from("shiny gold")); // 34988
}
