///
/// To anyone reading this, I know this code is ugly,
/// but I'll fix it after I am done with all the other stages.
///

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

mod planet;

use planet::Planet;
use std::collections::HashMap;

fn generate_planet_map() -> HashMap<String, Planet> {
	let input = File::open("src/Day 6/orbits.in").unwrap();
	let reader = BufReader::new(input);

	let mut planet_map = HashMap::<String, Planet>::new();

	for orbit in reader.lines() {
		let orbiting_components: Vec<String> = orbit
			.unwrap()
			.split(")")
			.map(|s| FromStr::from_str(s).unwrap())
			.collect();

		let parent = orbiting_components[0].to_string();
		let child = orbiting_components[1].to_string();

		if planet_map.contains_key(&parent) {
			let planet = planet_map.get_mut(&parent).unwrap();
			planet.child_names.push(child);
		} else {
			let planet = Planet::new(&parent, &child);
			planet_map.insert(parent, planet);
		}
	}

	planet_map
}

fn get_orbit_count(planet_map: &HashMap<String, Planet>, planet_name: &String, level: u32) -> u32 {
	let mut crt_count = level;

	if !planet_map.contains_key(planet_name) {
		return crt_count;
	}

	let planet = planet_map.get(planet_name).unwrap();
	for child in &planet.child_names {
		crt_count += get_orbit_count(planet_map, child, level + 1);
	}

	crt_count
}

fn planet_contains(planet_map: &HashMap<String, Planet>, parent: &String, target: &String) -> bool {
    if !planet_map.contains_key(parent) {
        return false;
    }

    let planet = planet_map.get(parent).unwrap();

    if planet.child_names.is_empty() {
        return false;
    }

    if planet.child_names.contains(&target) {
        return true;
    }

    for child_name in &planet.child_names {
        if planet_contains(planet_map, child_name, target) {
            return true;
        }
    }

    false
}

fn planet_contains_both(planet_map: &HashMap<String, Planet>, parent: &String, a: &String, b: &String) -> bool {
	planet_contains(planet_map, parent, a) && planet_contains(planet_map, parent, b)
}

fn get_common_parent(planet_map: &HashMap<String, Planet>, a: &String, b: &String) -> String {
	let mut crt_parent = String::from("COM");

	loop {
		let mut planet = planet_map.get(&crt_parent).unwrap();

		while planet.child_names.len() == 1 {
            crt_parent = planet.child_names[0].to_string();
            planet = planet_map.get(&crt_parent).unwrap();
        }

        let mut child_has_both = false;
        for child_name in &planet.child_names {
            if planet_contains_both(planet_map, child_name, a, b) {
                child_has_both = true;
                crt_parent = child_name.to_string();
                break;
            }
        }

        // We found the common parent
        if !child_has_both {
            break;
        }
	}

    crt_parent
}

fn get_distance_to_child(planet_map: &HashMap<String, Planet>, parent: &String, child: &String) -> u32 {
    let mut distance: u32 = 0;

    let mut crt_planet = planet_map.get(parent).unwrap();

    loop {
        if crt_planet.child_names.contains(child) {
            return distance + 1;
        }

        for child_name in &crt_planet.child_names {
            if planet_contains(planet_map, child_name, child) {
                crt_planet = planet_map.get(child_name).unwrap();
                distance += 1;
                break;
            }
        }
    }
}

fn main() {
	let planet_map = generate_planet_map();

	let start_planet = String::from("COM");
	println!("Total orbit count: {}", get_orbit_count(&planet_map, &start_planet, 0));

	let my_planet = String::from("YOU");
	let santas_planet = String::from("SAN");

    let common_parent = get_common_parent(&planet_map, &my_planet, &santas_planet);

    let orbital_transfers =
        get_distance_to_child(&planet_map, &common_parent, &my_planet)
        + get_distance_to_child(&planet_map, &common_parent, &santas_planet)
        - 2; // we subtract 2 cause we ignore the hops to the target planets

    println!("Number of orbital transfers: {}", orbital_transfers);
}